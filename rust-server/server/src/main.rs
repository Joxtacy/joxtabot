use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use discord_utils::DiscordBuilder;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use warp::{
    http::{HeaderMap, StatusCode},
    Filter,
};

use twitch_irc_parser::parse_message;

mod twitch;

use twitch::{
    handle_webhook_message, parse_twitch_request_header, verify_twitch_message,
    RevokedSubscription, TwitchMessage, VerificationChallenge, NOTIFICATION_TYPE,
    SUBSCRIPTION_REVOKED_TYPE, WEBHOOK_CALLBACK_VERIFICATION_TYPE,
};

mod utils;
use utils::init_env;

const TARGET_WS_CLIENT: &str = "WS_CLIENT";
const TWITCH_WS_URL: &str = "ws://irc-ws.chat.twitch.tv:80";

// Not sure how to support SSL with tokio-tungstenite
const _TWITCH_WS_URL_SSL: &str = "wss://irc-ws.chat.twitch.tv:443";

mod string_utils;

mod websocket {
    pub mod client_utils;
    pub mod server_utils;
    pub mod utils;
}

#[tokio::main]
async fn main() {
    let message_ids = Arc::new(Mutex::new(HashSet::<String>::new()));

    // Used to notify tasks to shutdown.
    let (notify_shutdown, _) = tokio::sync::broadcast::channel::<()>(1);

    // Used in tasks to notify that they have finished shutting down.
    let (send_shutdown, mut recv_shutdown) = mpsc::channel::<()>(1);

    // Init env variables
    let (token, port) = init_env();

    info!(target: "MAIN", "Running on port {}", port);

    let (tx, mut rx) = mpsc::channel(32);
    let (ws_client_tx, _) = broadcast::channel::<String>(32);

    // Used to listen for shutdown command.
    let mut shutdown_ws_client = notify_shutdown.subscribe();
    // Used to notify that the task has shut down.
    let send_shutdown_notifier = send_shutdown.clone();

    // Run our WebSocket client in its own task.
    tokio::task::spawn(async move {
        // Connect our WebSocket client to Twitch.
        let mut res = connect_async(TWITCH_WS_URL).await;

        while let Err(e) = res {
            warn!(
                target: TARGET_WS_CLIENT,
                "Failed to connect to Twitch: {:?}", e
            );
            warn!(target: TARGET_WS_CLIENT, "Retrying...");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            res = connect_async(TWITCH_WS_URL).await;
        }
        debug!(target: TARGET_WS_CLIENT, "Connected to Twitch");

        let (mut ws_stream, _) = res.unwrap();

        // Init the WebSocket connection to our Twitch channel.
        crate::websocket::client_utils::init_ws(&mut ws_stream, &token).await;

        debug!(target: TARGET_WS_CLIENT, "Sending intro message");
        ws_stream
            .send(string_utils::create_privmsg("joxtacy", "I have risen!").into())
            .await
            .unwrap();

        loop {
            tokio::select! {
                Some(msg) = ws_stream.next() => {
                    if let Ok(msg) = msg {
                        match msg {
                            Message::Text(text) => {
                                let lines = text.lines();

                                for msg in lines {
                                    debug!(target: TARGET_WS_CLIENT, "Twitch Message: {}", msg);

                                    // Parse the Twitch Message
                                    let parsed_message = parse_message(msg);

                                    // Match on the different Twitch Commands
                                    crate::websocket::client_utils::handle_message(&mut ws_stream, parsed_message).await;
                                }
                            },
                            Message::Binary(bin) => {
                                debug!(target: TARGET_WS_CLIENT, "WS BINARY: {:?}", bin);
                            },
                            msg => {
                                debug!(target: TARGET_WS_CLIENT, "We got something else: {:?}", msg);
                            }
                        }
                    } else if let Err(e) = msg {
                        // We should try to reconnect here
                        error!(target: TARGET_WS_CLIENT, "Error: {:?}", e);
                    }
                },
                Some(msg) = rx.recv() => {
                    match msg {
                        crate::websocket::client_utils::TwitchCommand::Privmsg { message } => {
                            let priv_msg = string_utils::create_privmsg("joxtacy", &message);
                            let res = ws_stream.send(priv_msg.into()).await;
                            if let Err(e) = res {
                                error!(target: TARGET_WS_CLIENT, "Failed to send message to Twitch: {:?}", e);
                            }
                        },
                        command => debug!(target: TARGET_WS_CLIENT, "MPSC Twitch Command: Not supported yet: {:?}", command)
                    }
                },
                _ = shutdown_ws_client.recv() => {
                    // If a shutdown signal is received, close the socket and return.

                    debug!(target: TARGET_WS_CLIENT, "Sending good bye message");
                    ws_stream.send(string_utils::create_privmsg("joxtacy", "So long, and thanks for all the fish.").into()).await.unwrap_or_else(|_err| warn!(target: TARGET_WS_CLIENT, "Failed to send bye bye message"));
                    let _ = ws_stream.close(None).await;
                    break;
                }
                else => {
                    break;
                }
            }
        }

        // By dropping the `send_shutdown_notifier` we tell the `recv_shutdown` that we have
        // finished shutdown. It will stop awaiting this instance of this mpsc channel sender.
        drop(send_shutdown_notifier);

        info!(target: TARGET_WS_CLIENT, "Disconnected");
    });

    // Clone the sender of the broadcast channel so that we can also use it in the
    // webhook callback.
    let ws_client_tx1 = ws_client_tx.clone();

    // Create a new subscription on the sender for the broadcast channel.
    let with_receiver = warp::any().map(move || ws_client_tx1.subscribe());

    // Used to listen for shutdown command.
    let shutdown_ws = notify_shutdown.clone();
    // Used to notify that the task has shut down.
    let send_shutdown_notifier = send_shutdown.clone();

    let with_shutdown_notifier = warp::any().map(move || shutdown_ws.subscribe());
    let with_shutdown_sender = warp::any().map(move || send_shutdown_notifier.clone());

    // This is where our own client will connect
    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(with_receiver)
        .and(with_shutdown_notifier)
        .and(with_shutdown_sender)
        .map(
            |ws: warp::ws::Ws,
             ws_client_rx: broadcast::Receiver<String>,
             notify_shutdown: broadcast::Receiver<()>,
             send_shutdown: mpsc::Sender<()>| {
                ws.on_upgrade(move |websocket| {
                    crate::websocket::server_utils::client_connected(
                        websocket,
                        ws_client_rx,
                        notify_shutdown,
                        send_shutdown,
                    )
                })
            },
        );

    // Root path. Just return some text for now.
    let root = warp::path::end().map(|| "Hello World!");

    // Clone the sending part of the broadcast channel for the websocket server.
    let with_ws_sender = warp::any().map(move || ws_client_tx.clone());

    // Clone the sending part of the mpsc channel for the websocket client.
    let with_sender = warp::any().map(move || tx.clone());

    let with_message_ids = warp::any().map(move || Arc::clone(&message_ids));

    // This is where Twitch will send their callbacks
    let post_routes = warp::post()
        .and(warp::path!("twitch" / "webhooks" / "callback"))
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and(with_sender)
        .and(with_ws_sender)
        .and(with_message_ids)
        .then(
            move |headers: HeaderMap,
                  bytes: bytes::Bytes,
                  tx: mpsc::Sender<crate::websocket::client_utils::TwitchCommand>,
                  ws_tx: broadcast::Sender<String>,
                  message_ids: Arc<Mutex<HashSet<String>>>| {
                webhook_callback(headers, bytes, tx, ws_tx, message_ids)
            },
        );

    // Use this as shutdown signal for the warp server.
    let mut shutdown_server = notify_shutdown.subscribe();

    let get_routes = warp::get().and(root.or(websocket));
    let routes = get_routes.or(post_routes);
    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], port), async move {
            // Once we receive a shutdown signal here, we will stop awaiting and the server will
            // shutdown once the closure finishes.
            let _ = shutdown_server.recv().await;

            info!(target: "SERVER", "Shutting down the server");
        });
    tokio::task::spawn(server);

    // We are waiting for a CTRL-C command, which means we should shut down.
    // Could possibly add other mechanisms to listen to for shutdown signals.
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!(target: "MAIN", "Starting shutdown process...");
        }
        Err(err) => {
            error!(target: "MAIN", "Failed to listen for shutdown signal. Reason: {}", err.to_string());
        }
    }

    // Broadcast message to shutdown all tasks.
    let _ = notify_shutdown.send(());

    // Wait for the tasks to finish.
    //
    // We drop our sender first because the recv() call otherwise sleeps forever.
    drop(send_shutdown);

    // When every sender has gone out of scope, the recv call will return
    // with an error. We ignore the error. This just means that all tasks
    // have finished.
    let _ = recv_shutdown.recv().await;

    info!(target: "MAIN", "So long, and thanks for all the fish.");
}

async fn webhook_callback(
    headers: HeaderMap,
    bytes: bytes::Bytes,
    tx: mpsc::Sender<crate::websocket::client_utils::TwitchCommand>,
    ws_tx: broadcast::Sender<String>,
    message_ids: Arc<Mutex<HashSet<String>>>,
) -> warp::reply::WithStatus<String> {
    use crate::websocket::client_utils::TwitchCommand;

    let body_str = String::from_utf8(bytes.clone().into()).unwrap_or_else(|_| "".to_string());

    let verification = verify_twitch_message(&headers, &body_str);
    if !verification {
        debug!(target: "WEBHOOK", "Message not from Twitch. Abort.");
        return warp::reply::with_status("BAD_REQUEST".to_string(), StatusCode::BAD_REQUEST);
    }

    // Verify that the message from Twitch is not too old
    {
        let twitch_message_timestamp = headers.get("Twitch-Eventsub-Message-Timestamp");

        let result = twitch::verify_twitch_message_age(twitch_message_timestamp);

        if result.is_err() {
            let error_message = result.unwrap_err();
            match error_message {
                twitch::TwitchTimestampError::TooOld => {
                    error!(target: "WEBHOOK", "Message from Twitch is too old. Rejecting.");
                    return warp::reply::with_status("Message Too Old".to_string(), StatusCode::OK);
                }
                twitch::TwitchTimestampError::NotAValidTimestamp => {
                    error!(target: "WEBHOOK", "Could not parse timestamp in Twitch-Eventsub-Message-Timestamp header");
                    return warp::reply::with_status(
                        "Invalid Timestamp".to_string(),
                        StatusCode::BAD_REQUEST,
                    );
                }
            };
        };
    }

    // Verify that we haven't already received this message
    {
        // Do this in a separate block to make sure the reference to the lock is
        // dropped before any new call to `await`

        let twitch_message_id = headers.get("Twitch-Eventsub-Message-Id");
        let twitch_message_id = parse_twitch_request_header(twitch_message_id);

        let mut ids = message_ids.lock().expect("Lock could not be aquired!");

        if ids.contains(&twitch_message_id) {
            return warp::reply::with_status(
                "Already received message".to_string(),
                StatusCode::OK,
            );
        } else {
            ids.insert(twitch_message_id);
        }
    }

    debug!(target: "WEBHOOK", "Twitch message verified");

    let twitch_message_type = headers.get("Twitch-Eventsub-Message-Type");
    let twitch_message_type = parse_twitch_request_header(twitch_message_type);

    if twitch_message_type == NOTIFICATION_TYPE {
        let message = serde_json::from_str::<TwitchMessage>(&body_str).unwrap();
        let twitch_command = handle_webhook_message(message);

        match twitch_command {
            TwitchCommand::Ded => {
                crate::websocket::utils::broadcast_message(&ws_tx, "Death".to_string());
            }
            TwitchCommand::FourTwenty => {
                crate::websocket::utils::broadcast_message(&ws_tx, "420".to_string());
            }
            TwitchCommand::Nice => {
                crate::websocket::utils::broadcast_message(&ws_tx, "Nice".to_string());
            }
            TwitchCommand::First(ref username) => {
                let res = std::fs::write("first.txt", format!("First: {}", username));
                match res {
                    Ok(()) => info!(target: "WEBHOOK", "Writing `first` succeeded"),
                    Err(e) => error!(target: "WEBHOOK", "Writing `first` failed: {:?}", e),
                }
            }
            TwitchCommand::StreamOnline => {
                info!(target: "WEBHOOK", "Received StreamOnline message");
                debug!(target: "WEBHOOK", "Resetting `first`");
                let res = std::fs::write("first.txt", "First:");
                match res {
                    Ok(()) => info!(target: "WEBHOOK", "Resetting `first` succeeded"),
                    Err(e) => error!(target: "WEBHOOK", "Resetting `first` failed: {:?}", e),
                }

                let token = std::env::var("TWITCH_APP_ACCESS_TOKEN").unwrap();
                let client_id = std::env::var("TWITCH_CLIENT_ID").unwrap();
                let user_id = std::env::var("TWITCH_JOXTACY_USER_ID")
                    .unwrap()
                    .parse()
                    .unwrap();

                info!(target: "WEBHOOK", "Getting stream info");
                if let Ok(stream_info) =
                    twitch_utils::get_stream_info(token, client_id, user_id).await
                {
                    debug!(target: "WEBHOOK", "Got stream info: {:?}", stream_info);
                    let token = std::env::var("DISCORD_BOT_TOKEN").unwrap();
                    // let channel_id = std::env::var("DISCORD_TESTING_JOXTABOT_CHANNELID")
                    let channel_id = std::env::var("DISCORD_JOXTACY_IS_LIVE_CHANNELID")
                        .unwrap()
                        .parse()
                        .unwrap();

                    let message = if !stream_info.data.is_empty() {
                        let stream_info = stream_info.data.first().unwrap();
                        string_utils::create_stream_online_message(
                            &stream_info.game_name,
                            &stream_info.title,
                        )
                    } else {
                        string_utils::create_stream_online_message(
                            "something went wrong",
                            "something went wrong",
                        )
                    };
                    info!(target: "WEBHOOK", "Sending message to Discord: {}", message);
                    let _res = DiscordBuilder::new(&token)
                        .build()
                        .create_message(channel_id, &message)
                        .await;
                    debug!(target: "WEBHOOK", "Message to Discord sent: {:?}", _res);
                }
            }
            TwitchCommand::EmoteOnly => {
                let res = tx
                    .send(TwitchCommand::Privmsg {
                        message: "/emoteonly".to_string(),
                    })
                    .await;

                if res.is_ok() {
                    let tx1 = tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_secs(120)).await;
                        let _res = tx1
                            .send(TwitchCommand::Privmsg {
                                message: "/emoteonlyoff".to_string(),
                            })
                            .await;
                    });
                }
            }
            ref unsupported_message => {
                warn!(target: "WEBHOOK", "Unsupported Message: {:?}", unsupported_message);
            }
        }

        let res = tx.send(twitch_command).await;
        if let Err(e) = res {
            error!(target: "WEBHOOK",
                "Could not send message to our MPSC channel: {:?}",
                e
            );
        }

        return warp::reply::with_status("".to_string(), StatusCode::NO_CONTENT);
    } else if twitch_message_type == WEBHOOK_CALLBACK_VERIFICATION_TYPE {
        // This is when subscribing to a webhook
        let message = serde_json::from_str::<VerificationChallenge>(&body_str).unwrap();
        return warp::reply::with_status(message.challenge, StatusCode::OK);
    } else if twitch_message_type == SUBSCRIPTION_REVOKED_TYPE {
        // This is when webhook subscription was revoked
        let message = serde_json::from_str::<RevokedSubscription>(&body_str).unwrap();
        error!(target: "WEBHOOK",
            "Webhook subscription revoked. Reason: {}",
            message.subscription.status
        );
        return warp::reply::with_status("".to_string(), StatusCode::NO_CONTENT);
    }

    let body = String::from_utf8(bytes.to_vec()).unwrap();
    debug!(target: "WEBHOOK", "Received POST request: {:?}", body);
    warp::reply::with_status(body, StatusCode::OK)
}
