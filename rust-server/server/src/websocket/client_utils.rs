use crate::string_utils;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use twitch_irc_parser::{Command, ParsedTwitchMessage, Tag};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwitchCommand {
    Ded,
    EmoteOnly,
    First(String),
    FourTwenty,
    Nice,
    Privmsg { message: String },
    Pushup(u32),
    Situp(u32),
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

            if let Some(nick) = nick {
                println!("[WS CLIENT] {} joined #{}", nick, channel);
            }
        }
        Command::PART(channel) => {
            let nick = match message.source {
                Some(source) => source.nick,
                None => None,
            };

            if let Some(nick) = nick {
                println!("[WS CLIENT] {} left #{}", nick, channel);
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
