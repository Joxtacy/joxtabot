use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use discord_utils::DiscordBuilder;
use futures_util::{SinkExt, StreamExt};
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

    // Init env variables
    let (token, port) = init_env();

    println!("Running on port {}", port);

    let (tx, mut rx) = mpsc::channel(32);
    let (ws_client_tx, _) = broadcast::channel::<String>(32);

    // Run our WebSocket client in its own task.
    tokio::task::spawn(async move {
        // Connect our WebSocket client to Twitch.
        let mut res = connect_async(TWITCH_WS_URL).await;

        while let Err(e) = res {
            eprintln!("[WS CLIENT] Failed to connect to Twitch: {:?}", e);
            eprintln!("[WS CLIENT] Retrying...");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            res = connect_async(TWITCH_WS_URL).await;
        }
        println!("[WS CLIENT] Connected to Twitch");

        let (mut ws_stream, _) = res.unwrap();

        // Init the WebSocket connection to our Twitch channel.
        crate::websocket::client_utils::init_ws(&mut ws_stream, &token).await;

        loop {
            tokio::select! {
                Some(msg) = ws_stream.next() => {
                    if let Ok(msg) = msg {
                        match msg {
                            Message::Text(text) => {
                                let lines = text.lines();

                                for msg in lines {
                                    println!("[WS CLIENT] Twitch Message: {}", msg);

                                    // Parse the Twitch Message
                                    let parsed_message = parse_message(msg);

                                    // Match on the different Twitch Commands
                                    crate::websocket::client_utils::handle_message(&mut ws_stream, parsed_message).await;
                                }
                            },
                            Message::Binary(bin) => {
                                println!("[WS CLIENT] WS BINARY: {:?}", bin);
                            },
                            msg => {
                                println!("[WS CLIENT] We got something else: {:?}", msg);
                            }
                        }
                    } else if let Err(e) = msg {
                        // We should try to reconnect here
                        println!("[WS CLIENT] Error: {:?}", e);
                    }
                },
                Some(msg) = rx.recv() => {
                    match msg {
                        crate::websocket::client_utils::TwitchCommand::Privmsg { message } => {
                            let priv_msg = string_utils::create_privmsg("joxtacy", &message);
                            let res = ws_stream.send(priv_msg.into()).await;
                            if let Err(e) = res {
                                eprintln!("[WS CLIENT] Failed to send message to Twitch: {:?}", e);
                            }
                        },
                        command => println!("MPSC Twitch Command: Not supported yet: {:?}", command)
                    }
                },
                else => {
                    break;
                }
            }
        }
    });

    // Clone the sender of the broadcast channel so that we can also use it in the
    // webhook callback.
    let ws_client_tx1 = ws_client_tx.clone();

    // Create a new subscription on the sender for the broadcast channel.
    let with_receiver = warp::any().map(move || ws_client_tx1.subscribe());

    // This is where our own client will connect
    let websocket = warp::path("ws").and(warp::ws()).and(with_receiver).map(
        |ws: warp::ws::Ws, ws_client_rx: broadcast::Receiver<String>| {
            ws.on_upgrade(move |websocket| {
                crate::websocket::server_utils::client_connected(websocket, ws_client_rx)
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

    let get_routes = warp::get().and(root.or(websocket));
    let routes = get_routes.or(post_routes);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
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
        eprintln!("[WEBHOOK] Message not from Twitch. Abort.");
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
                    eprintln!("[WEBHOOK] Message from Twitch is too old. Rejecting.");
                    return warp::reply::with_status("Message Too Old".to_string(), StatusCode::OK);
                }
                twitch::TwitchTimestampError::NotAValidTimestamp => {
                    eprintln!("[WEBHOOK] Could not parse timestamp in Twitch-Eventsub-Message-Timestamp header");
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

    println!("[WEBHOOK] Twitch message verified");

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
                    Ok(()) => println!("Writing `first` succeeded"),
                    Err(e) => eprintln!("Writing `first` failed: {:?}", e),
                }
            }
            TwitchCommand::StreamOnline => {
                let res = std::fs::write("first.txt", "First:");
                match res {
                    Ok(()) => println!("Resetting `first` succeeded"),
                    Err(e) => eprintln!("Resetting `first` failed: {:?}", e),
                }

                let token = std::env::var("TWITCH_APP_ACCESS_TOKEN").unwrap();
                let client_id = std::env::var("TWITCH_CLIENT_ID").unwrap();
                let user_id = std::env::var("TWITCH_JOXTACY_USER_ID")
                    .unwrap()
                    .parse()
                    .unwrap();

                if let Ok(stream_info) =
                    twitch_utils::get_stream_info(token, client_id, user_id).await
                {
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
                    let _res = DiscordBuilder::new(&token)
                        .build()
                        .create_message(channel_id, &message)
                        .await;
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
                eprintln!("[WEBHOOK] Unsupported Message: {:?}", unsupported_message);
            }
        }

        let res = tx.send(twitch_command).await;
        if let Err(e) = res {
            eprintln!(
                "[WEBHOOK] Could not send message to our MPSC channel: {:?}",
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
        println!(
            "[WEBHOOK] ERROR: Webhook subscription revoked. Reason: {}",
            message.subscription.status
        );
        return warp::reply::with_status("".to_string(), StatusCode::NO_CONTENT);
    }

    let body = String::from_utf8(bytes.to_vec()).unwrap();
    println!("[WEBHOOK] Received POST request: {:?}", body);
    warp::reply::with_status(body, StatusCode::OK)
}
