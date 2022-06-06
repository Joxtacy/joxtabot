use twitch_irc_parser::*;

#[test]
fn reconnect() {
    let message = ":tmi.twitch.tv RECONNECT";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        command: Command::RECONNECT,
    };

    assert_eq!(actual, expected);
}
