use pretty_assertions::assert_eq;
use twitch_irc_parser::*;

#[test]
fn reconnect() {
    let message = ":tmi.twitch.tv RECONNECT";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        source: Some(Source {
            host: String::from("tmi.twitch.tv"),
            nick: None,
        }),
        command: Command::RECONNECT,
    };

    assert_eq!(actual, expected);
}
