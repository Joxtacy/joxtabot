use futures_util::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use warp::{
    http::{HeaderMap, StatusCode},
    Filter,
};

use twitch_irc_parser::parse_message;

mod api;
use api::webhooks::twitch::{
    handle_webhook_message, parse_header, verify_twitch_message, RevokedSubscription,
    TwitchMessage, VerificationChallenge, NOTIFICATION_TYPE, SUBSCRIPTION_REVOKED_TYPE,
    WEBHOOK_CALLBACK_VERIFICATION_TYPE,
};

const TWITCH_WS_URL: &str = "ws://irc-ws.chat.twitch.tv:80";

// Not sure how to support SSL with tokio-tungstenite
const _TWITCH_WS_URL_SSL: &str = "wss://irc-ws.chat.twitch.tv:443";

fn init_env() -> (String, u16) {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    dotenv::dotenv().ok();
    env_logger::init();

    let token = match std::env::var("TWITCH_IRC_BOT_OAUTH") {
        Ok(token) => token,
        Err(e) => panic!("Can't proceed without bot token: {}", e),
    };
    let port = get_env_port();

    (token, port)
}

mod string_utils {
    ///
    /// Hi @everyone! I am **live**!
    /// > Playing: {game}
    /// > Title: {title}
    /// https://twitch.tv/joxtacy
    ///
    pub fn create_stream_online_message(game: &str, title: &str) -> String {
        format!(
            "Hi @everyone! I am **live**!\n\
             > Playing: {}\n\
             > Title: {}\n\
             https://twitch.tv/joxtacy",
            game, title
        )
    }

    pub fn create_privmsg(channel: &str, message: &str) -> String {
        format!("PRIVMSG #{} :{}", channel, message)
    }
}

mod websocket_utils {
    use crate::string_utils;
    use futures_util::SinkExt;
    use tokio::{net::TcpStream, sync::broadcast};
    use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
    use twitch_irc_parser::{Command, ParsedTwitchMessage, Tag};

    type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

    #[derive(Debug, Clone)]
    pub enum TwitchCommand {
        Ded,
        EmoteOnly,
        First(String),
        FourTwenty,
        Nice,
        Privmsg { message: String },
        Pushup,
        Situp,
        StreamOnline,
        Timeout { timeout: u32, user: String },
        UnsupportedMessage,
    }

    /// Initialize the Twitch WebSocket Connection
    ///
    /// Send a `PASS` message to authorize
    /// Send a `NICK` message to establish username
    /// Send a `JOIN` message to join the IRC channel
    /// Send a `CAP REQ /membership` message
    /// Send a `CAP REQ /tags` message to get tags with messages
    pub async fn init_ws(ws_stream: &mut WsStream, token: &str) {
        ws_stream
            .send(format!("PASS {}", token).into())
            .await
            .unwrap();
        ws_stream.send("NICK joxtabot".into()).await.unwrap();
        ws_stream.send("JOIN #joxtacy".into()).await.unwrap();
        ws_stream
            .send("CAP REQ :twitch.tv/membership".into())
            .await
            .unwrap();
        ws_stream
            .send("CAP REQ :twitch.tv/tags twitch.tv/commands".into())
            .await
            .unwrap();
    }

    pub fn broadcast_message<T>(tx: &broadcast::Sender<T>, msg: T)
    where
        T: std::fmt::Debug,
    {
        if let Err(e) = tx.send(msg) {
            eprintln!("Could not send message to socket server: {:?}", e);
        }
    }

    /// Handle the parsed Twitch IRC message
    pub async fn handle_message(ws_stream: &mut WsStream, message: ParsedTwitchMessage) {
        match message.command {
            // Respond with a PONG to keep message alive
            Command::PING => {
                let res = ws_stream.send("PONG :tmi.twitch.tv".into()).await;

                if let Err(e) = res {
                    eprintln!("[WS CLIENT] COULD NOT SEND PONG: {e:?}");
                }
            }
            Command::JOIN(channel) => {
                let nick = match message.source {
                    Some(source) => source.nick,
                    None => None,
                };

                match nick {
                    Some(nick) => {
                        println!("[WS CLIENT] {} joined #{}", nick, channel);
                    }
                    None => {}
                }
            }
            Command::PART(channel) => {
                let nick = match message.source {
                    Some(source) => source.nick,
                    None => None,
                };

                match nick {
                    Some(nick) => {
                        println!("[WS CLIENT] {} left #{}", nick, channel);
                    }
                    None => {}
                }
            }
            Command::PRIVMSG {
                channel,
                message,
                tags,
                ..
            } => {
                let display_name = if let Some(tags) = tags {
                    let dis = tags.get("display-name");
                    if let Some(Tag::DisplayName(display_name)) = dis {
                        display_name.clone()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };
                println!("[WS CLIENT] @{} #{}: {}", display_name, channel, message);

                if message.contains("catJAM") {
                    let response = string_utils::create_privmsg(&channel, "catJAM");
                    ws_stream.send(response.into()).await.unwrap();
                } else if message.contains("widepeepoHappy") {
                    let response = string_utils::create_privmsg(&channel, "widepeepoHappy");
                    ws_stream.send(response.into()).await.unwrap();
                }
            }
            unsupported_message => {
                println!(
                    "[WS CLIENT] UNSUPPORTED TWITCH COMMAND: {:?}",
                    unsupported_message
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
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
        websocket_utils::init_ws(&mut ws_stream, &token).await;

        loop {
            tokio::select! {
                Some(msg) = ws_stream.next() => {
                    if let Ok(msg) = msg {
                        match msg {
                            Message::Text(text) => {
                                let split = text.split("\r\n");
                                let filtered = split
                                    .filter(|s| !s.is_empty());
                                for msg in filtered {
                                    println!("[WS CLIENT] Twitch Message: {}", msg);

                                    // Parse the Twitch Message
                                    let parsed_message = parse_message(msg);

                                    // Match on the different Twitch Commands
                                    websocket_utils::handle_message(&mut ws_stream, parsed_message).await;
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
                        websocket_utils::TwitchCommand::Privmsg { message } => {
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
        |ws: warp::ws::Ws, mut ws_client_rx: broadcast::Receiver<String>| {
            ws.on_upgrade(|websocket| async move {
                println!("[WS SERVER] User connected");

                let (mut tx, mut rx) = websocket.split();

                // Some tips might be found here: https://tms-dev-blog.com/build-basic-rust-websocket-server/
                loop {
                    tokio::select! {
                        msg = rx.next() => {
                            match msg {
                                Some(msg) => println!("[WS SERVER] Received message: {:?}", msg),
                                None => {
                                    println!("[WS SERVER] User disconnected");
                                    break;
                                }
                            }
                        },
                        msg = ws_client_rx.recv() => {
                            match msg {
                                Ok(msg) => {
                                    println!("[WS SERVER] Received message from broadcast: {}", msg);
                                    let res = tx.send(warp::ws::Message::text(msg)).await;
                                    if let Err(e) = res {
                                        eprintln!(
                                            "[WS SERVER] Could not send message to client. Reason: {:?}",
                                            e
                                            );
                                        // If we end up here we exit out of the loop since the
                                        // client is no longer connected.
                                        break;
                                    }
                                },
                                Err(e) => {
                                    eprintln!("[WS SERVER] Error while receiving message on broadcast channel: {:?}", e);
                                }
                            }

                        },
                        else => {
                            println!("[WS SERVER] Else branch executed");
                        }
                    }
                }
            })
        },
    );

    // Root path. Just return some text for now.
    let root = warp::path::end().map(|| "Hello World!");

    // Clone the sending part of the broadcast channel for the websocket server.
    let with_ws_sender = warp::any().map(move || ws_client_tx.clone());

    // Clone the sending part of the mpsc channel for the websocket client.
    let with_sender = warp::any().map(move || tx.clone());

    // This is where Twitch will send their callbacks
    let post_routes = warp::post()
        .and(warp::path!("twitch" / "webhooks" / "callback"))
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and(with_sender)
        .and(with_ws_sender)
        .then(
            |headers: HeaderMap,
             bytes: bytes::Bytes,
             tx: mpsc::Sender<websocket_utils::TwitchCommand>,
             ws_tx: broadcast::Sender<String>| async move {
                use websocket_utils::TwitchCommand;

                let body_str = String::from_utf8(bytes.clone().into()).unwrap();

                let verification = verify_twitch_message(&headers, &body_str);
                if !verification {
                    eprintln!("[WEBHOOK] Message not from Twitch. Abort.");
                    return warp::reply::with_status(
                        "BAD_REQUEST".to_string(),
                        StatusCode::BAD_REQUEST,
                    );
                }

                println!("[WEBHOOK] Twitch message verified");

                let twitch_message_type = headers.get("Twitch-Eventsub-Message-Type");
                let twitch_message_type = parse_header(twitch_message_type);

                if twitch_message_type == NOTIFICATION_TYPE {
                    // TODO: Check if message is duplicate. https://dev.twitch.tv/docs/eventsub/handling-webhook-events#processing-an-event
                    let message = serde_json::from_str::<TwitchMessage>(&body_str).unwrap();
                    let twitch_command = handle_webhook_message(message);

                    match twitch_command {
                        TwitchCommand::Ded => {
                            websocket_utils::broadcast_message(&ws_tx, "Death".to_string());
                        }
                        TwitchCommand::FourTwenty => {
                            websocket_utils::broadcast_message(&ws_tx, "420".to_string());
                        }
                        TwitchCommand::Nice => {
                            websocket_utils::broadcast_message(&ws_tx, "Nice".to_string());
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
                                let _res =
                                    discord_utils::create_message(token, channel_id, message).await;
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
            },
        );

    let get_routes = warp::get().and(root.or(websocket));
    let routes = get_routes.or(post_routes);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn get_env_port() -> u16 {
    let default_port = 8000;
    std::env::var("RUST_PORT")
        .unwrap_or_else(|_| default_port.to_string())
        .parse::<u16>()
        .unwrap_or(default_port)
}
