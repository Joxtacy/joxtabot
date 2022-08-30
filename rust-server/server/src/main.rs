use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::{broadcast, mpsc},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use warp::{
    http::{HeaderMap, StatusCode},
    Filter,
};

use twitch_irc_parser::{parse_message, Command, ParsedTwitchMessage, Tag};

mod api;
use api::webhooks::twitch::{
    handle_webhook_message, parse_header, verify_twitch_message, RevokedSubscription,
    TwitchMessage, VerificationChallenge, NOTIFICATION_TYPE, SUBSCRIPTION_REVOKED_TYPE,
    WEBHOOK_CALLBACK_VERIFICATION_TYPE,
};

const TWITCH_WS_URL: &str = "ws://irc-ws.chat.twitch.tv:80";

// Not sure how to support SSL with tokio-tungstenite
const _TWITCH_WS_URL_SSL: &str = "wss://irc-ws.chat.twitch.tv:443";

///
/// Hi @everyone! I am **live**!
/// > Playing: {game}
/// > Title: {title}
/// https://twitch.tv/joxtacy
///
fn create_stream_online_message(game: &str, title: &str) -> String {
    format!(
        "Hi @everyone! I am **live**!\n\
         > Playing: {}\n\
         > Title: {}\n\
         https://twitch.tv/joxtacy",
        game, title
    )
}
fn create_privmsg(channel: &str, message: &str) -> String {
    format!("PRIVMSG #{} :{}", channel, message)
}

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

mod websocket_utils {
    use tokio::sync::broadcast;

    pub fn broadcast_message<T>(tx: &broadcast::Sender<T>, msg: T)
    where
        T: std::fmt::Debug,
    {
        if let Err(e) = tx.send(msg) {
            eprintln!("Could not send message to socket server: {:?}", e);
        }
    }
}

/// Initialize the Twitch WebSocket Connection
///
/// Send a `PASS` message to authorize
/// Send a `NICK` message to establish username
/// Send a `JOIN` message to join the IRC channel
/// Send a `CAP REQ /membership` message
/// Send a `CAP REQ /tags` message to get tags with messages
async fn init_ws(ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, token: &str) {
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

/// Handle the parsed Twitch IRC message
async fn handle_message(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    message: ParsedTwitchMessage,
) {
    match message.command {
        // Respond with a PONG to keep message alive
        Command::PING => {
            println!("Sending PONG...");
            let res = ws_stream.send("PONG :tmi.twitch.tv".into()).await;

            if let Err(e) = res {
                eprintln!("COULD NOT SEND PONG: {e:?}");
            } else {
                println!("PONG sent");
            }
        }
        Command::JOIN(channel) => {
            let nick = match message.source {
                Some(source) => source.nick,
                None => None,
            };

            match nick {
                Some(nick) => {
                    println!("{} joined #{}", nick, channel);
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
                    println!("{} left #{}", nick, channel);
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
            println!("@{} #{}: {}", display_name, channel, message);

            if message.contains("catJAM") {
                let response = create_privmsg(&channel, "catJAM");
                ws_stream.send(response.into()).await.unwrap();
            } else if message.contains("widepeepoHappy") {
                let response = create_privmsg(&channel, "widepeepoHappy");
                ws_stream.send(response.into()).await.unwrap();
            }
        }
        _ => {
            println!("UNSUPPORTED MESSAGE");
        }
    }
}

#[tokio::main]
async fn main() {
    // Init env variables
    let (token, port) = init_env();

    println!("Running on port {}", port);

    let (tx, mut rx) = mpsc::channel(32);
    let (ws_client_tx, mut ws_client_rx) = broadcast::channel::<String>(32);

    // Run our WebSocket client in its own task.
    tokio::task::spawn(async move {
        // Connect our WebSocket client to Twitch.
        let (mut ws_stream, _) = connect_async(TWITCH_WS_URL)
            .await
            .expect("Could not connect to Twitch IRC server");
        println!("Websocket client connected to Twitch");

        // Init the WebSocket connection to our Twitch channel.
        init_ws(&mut ws_stream, &token).await;

        loop {
            tokio::select! {
                Some(msg) = ws_stream.next() => {
                    if let Ok(msg) = msg {
                        match msg {
                            Message::Text(text) => {
                                println!("ws TEXT:");
                                let split = text.split("\r\n");
                                let filtered = split
                                    .filter(|s| !s.is_empty());

                                for msg in filtered {
                                    println!("msg: {}", msg);

                                    // Parse the Twitch Message
                                    let parsed_message = parse_message(msg);

                                    // Match on the different Twitch Commands
                                    handle_message(&mut ws_stream, parsed_message).await;
                                }
                            },
                            Message::Binary(bin) => {
                                println!("ws BINARY:");
                                println!("{bin:?}");
                            },
                            _ => {
                                println!("ws client: We got something else");
                            }
                        }
                    } else if let Err(e) = msg {
                        // We should try to reconnect here
                        println!("ws client err: {:?}", e);
                    }
                },
                Some(msg) = rx.recv() => {
                    match msg {
                        TwitchCommand::Privmsg { message } => {
                            let priv_msg = create_privmsg("joxtacy", &message);
                            let res = ws_stream.send(priv_msg.into()).await;
                            if let Err(e) = res {
                                eprintln!("Failed to send message on ws: {:?}", e);
                            }
                        },
                        _ => println!("Not supported yet")
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
    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(with_receiver.clone())
        .map(
            |ws: warp::ws::Ws, mut ws_client_rx: broadcast::Receiver<String>| {
                ws.on_upgrade(|websocket| async move {
                    let (mut tx, rx) = websocket.split();

                    while let Ok(msg) = ws_client_rx.recv().await {
                        println!("Received message: {}", msg);
                        tx.send(warp::ws::Message::text(msg)).await;
                    }

                    // Echo back messages from `rx`
                    let _res = rx.forward(tx).map(|result| {
                        if let Err(e) = result {
                            eprintln!("websocket error: {:?}", e);
                        }
                    });
                })
            },
        );

    // Root path. Just return some text for now.
    let root = warp::path::end().map(|| "Hello World!");

    use serde_derive::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    struct Data {
        name: String,
    }

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
             tx: mpsc::Sender<TwitchCommand>,
             ws_tx: broadcast::Sender<String>| async move {
                let body_str = String::from_utf8(bytes.clone().into()).unwrap();

                let verification = verify_twitch_message(&headers, &body_str);
                if !verification {
                    eprintln!("Message not from Twitch. Abort.");
                    return warp::reply::with_status(
                        "BAD_REQUEST".to_string(),
                        StatusCode::BAD_REQUEST,
                    );
                }

                println!("Twitch message verified");

                let twitch_message_type = headers.get("Twitch-Eventsub-Message-Type");
                let twitch_message_type = parse_header(twitch_message_type);

                if twitch_message_type == NOTIFICATION_TYPE {
                    // This is where we got a notification
                    // TODO: Check if message is duplicate. https://dev.twitch.tv/docs/eventsub/handling-webhook-events#processing-an-event
                    let message = serde_json::from_str::<TwitchMessage>(&body_str).unwrap();
                    let twitch_command = handle_webhook_message(message);

                    match twitch_command {
                        TwitchCommand::Ded => {
                            // Send ws message from our ws server
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
                                    create_stream_online_message(
                                        &stream_info.game_name,
                                        &stream_info.title,
                                    )
                                } else {
                                    create_stream_online_message(
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
                        _ => {}
                    }

                    let res = tx.send(twitch_command).await;
                    if let Err(e) = res {
                        eprintln!("Could not send message: {:?}", e);
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
                        "ERROR: Webhook subscription revoked. Reason: {}",
                        message.subscription.status
                    );
                    return warp::reply::with_status("".to_string(), StatusCode::NO_CONTENT);
                }

                let body = String::from_utf8(bytes.to_vec()).unwrap();
                println!("received POST request webhook: {:?}", body);
                warp::reply::with_status(body, StatusCode::OK)
            },
        );

    let get_routes = warp::get().and(root.or(websocket));
    let routes = get_routes.or(post_routes);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn get_env_port() -> u16 {
    let default_port = 3030;
    std::env::var("RUST_PORT")
        .unwrap_or_else(|_| default_port.to_string())
        .parse::<u16>()
        .unwrap_or(default_port)
}
