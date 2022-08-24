use std::env;

use actix_web::web::Bytes;
use awc::ws;
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::select;

use twitch_irc_parser::{parse_message, Command, Tag};

const TWITCH_WS_URL: &str = "wss://irc-ws.chat.twitch.tv:443";

fn create_privmsg(channel: &str, message: &str) -> String {
    format!("PRIVMSG #{} :{}", channel, message)
}

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = match env::var("TWITCH_IRC_BOT_OAUTH") {
        Ok(token) => token,
        Err(e) => panic!("Can't proceed without bot token: {}", e),
    };

    let (res, mut ws) = awc::Client::new()
        .ws(TWITCH_WS_URL)
        .connect()
        .await
        .unwrap();

    println!("WS RESPONSE: {:?}", res);

    // Init the Twitch IRC connection
    ws.send(ws::Message::Text(format!("PASS {}", token).into()))
        .await
        .unwrap();
    ws.send(ws::Message::Text("NICK joxtabot".into()))
        .await
        .unwrap();
    ws.send(ws::Message::Text("JOIN #joxtacy".into()))
        .await
        .unwrap();
    ws.send(ws::Message::Text("CAP REQ :twitch.tv/membership".into()))
        .await
        .unwrap();
    ws.send(ws::Message::Text(
        "CAP REQ :twitch.tv/tags twitch.tv/commands".into(),
    ))
    .await
    .unwrap();

    loop {
        select! {
            Some(msg) = ws.next() => {
                match msg {
                    Ok(ws::Frame::Text(txt)) => {
                        let message = std::str::from_utf8(txt.as_ref()).unwrap();
                        let messages = message.split("\r\n").filter(|m| !m.is_empty()).map(|m| m.trim());

                        println!("\nRAW MESSAGE:");
                        println!("{}", message);
                        for message in messages {
                            let parsed_message = parse_message(message);

                            match parsed_message.command {
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
                                Command::PRIVMSG{ message, tags, channel, .. } => {
                                    let name = match tags {
                                        Some(tags) => {
                                            let display_name = match tags.get("display-name") {
                                                Some(display_name) => {
                                                    if let Tag::DisplayName(name) = display_name {
                                                        name.to_string()
                                                    } else {
                                                        "".to_string()
                                                    }
                                                },
                                                None => "".to_string(),
                                            };

                                            display_name
                                        },
                                        None => "".to_string(),
                                    };
                                    println!("{}: {}", name, message);

                                    if message.contains("catJAM") {
                                        let response = create_privmsg(&channel, "catJAM");
                                        ws.send(ws::Message::Text(response.into())).await.unwrap();
                                    } else if message.contains("widepeepoHappy") {
                                        let response = create_privmsg(&channel, "widepeepoHappy");
                                        ws.send(ws::Message::Text(response.into())).await.unwrap();
                                    }
                                },
                                Command::PING => {
                                    println!("PING received. Sending PONG");
                                    ws.send(ws::Message::Text("PONG :tmi.twitch.tv".into())).await.unwrap();
                                    println!("PONG sent");
                                },
                                _ => println!("UNSUPPORTED MESSAGE: {:?}", parsed_message),
                            }
                        }
                    },

                    Ok(ws::Frame::Ping(_)) => {
                        // respond to ping probes
                        ws.send(ws::Message::Pong(Bytes::new())).await.unwrap();
                    },

                    _ => {},
                }
            },

            else => break,
        }
    }
}
