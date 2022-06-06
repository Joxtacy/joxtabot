use twitch_irc_parser::*;

#[test]
fn reconnect() {
    let message = ":tmi.twitch.tv RECONNECT";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        source: Some(Source::new(String::from("tmi.twitch.tv"), None)),
        command: Command::RECONNECT,
    };

    assert_eq!(actual, expected);
}
