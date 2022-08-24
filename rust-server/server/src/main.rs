use tokio_tungstenite::{connect_async, tungstenite::Message};
use warp::Filter;
use futures_util::{SinkExt, FutureExt, StreamExt};

use twitch_irc_parser::{parse_message, Command, Tag};

const TWITCH_WS_URL: &str = "ws://irc-ws.chat.twitch.tv:80";

// Not sure how to support SSL with tokio-tungstenite
const _TWITCH_WS_URL_SSL: &str = "wss://irc-ws.chat.twitch.tv:443";

fn create_privmsg(channel: &str, message: &str) -> String {
    format!("PRIVMSG #{} :{}", channel, message)
}

#[tokio::main]
async fn main() {
    // Init env variables
    let (token, port) = {
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
    };

    println!("Running on port {}", port);

    // Run our WebSocket client in its own task.
    tokio::task::spawn(async move {
        // Connect our WebSocket client to Twitch.
        let (mut ws_stream, _) = connect_async(TWITCH_WS_URL).await.expect("Could not connect to Twitch IRC server");
        println!("Websocket client connected to Twitch");

        // Init the WebSocket connection to our Twitch channel.
        {
            ws_stream.send(format!("PASS {}", token).into())
                .await
                .unwrap();
            ws_stream.send("NICK joxtabot".into())
                .await
                .unwrap();
            ws_stream.send("JOIN #joxtacy".into())
                .await
                .unwrap();
            ws_stream.send("CAP REQ :twitch.tv/membership".into())
                .await
                .unwrap();
            ws_stream.send("CAP REQ :twitch.tv/tags twitch.tv/commands".into())
                .await
                .unwrap();
        }

        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(msg) => {
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
                                match parsed_message.command {
                                    // Respond with a PONG to keep message alive
                                    Command::PING => {
                                        println!("Sending PONG...");
                                        let res = ws_stream.send("PONG :tmi.twitch.tv".into()).await;

                                        if let Err(e) = res {
                                            eprintln!("COULD NOT SEND PONG: {e:?}");
                                        } else {
                                            println!("PONG sent");
                                        }
                                    },
                                    Command::JOIN(channel) => {
                                        let nick = match parsed_message.source {
                                            Some(source) => source.nick,
                                            None => None,
                                        };

                                        match nick {
                                            Some(nick) => {
                                                println!("{} joined #{}", nick, channel);
                                            }
                                            None => {}
                                        }
                                    },
                                    Command::PART(channel) => {
                                        let nick = match parsed_message.source {
                                            Some(source) => source.nick,
                                            None => None,
                                        };

                                        match nick {
                                            Some(nick) => {
                                                println!("{} left #{}", nick, channel);
                                            }
                                            None => {}
                                        }
                                    },
                                    Command::PRIVMSG { channel, message, tags, .. } => {

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
                                    },
                                    _ => {
                                        println!("UNSUPPORTED MESSAGE");
                                    }
                                }
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
                },
                Err(e) => {
                    // We should try to reconnect here
                    println!("ws client err: {:?}", e);
                }
            }
        }
    });

    // This is where our own client will connect
    let websocket = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| {
            let (tx, rx) = websocket.split();
            rx.forward(tx).map(|result| {
                if let Err(e) = result {
                    eprintln!("websocket error: {:?}", e);
                }
            })
        })
    });

    // Root path. Just return some text for now.
    let root = warp::path::end().map(|| "Hello World!");

    use serde_derive::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    struct Data {
        name: String,
    }

    // This is where Twitch will send their callbacks
    let post_routes = warp::post()
        .and(warp::path("twitch"))
        .and(warp::path("webhooks"))
        .and(warp::path("callback"))
        .and(warp::body::bytes())
        .map(|bytes: bytes::Bytes| {
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            println!("received POST request webhook: {:?}", body);
            let data = serde_json::from_str::<Data>(&body).unwrap();
            // data.name = "Jox".into();
            // warp::reply::json(&data)
            warp::reply::json(&data)
        });

    let get_routes = warp::get()
        .and(root.or(websocket));
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
