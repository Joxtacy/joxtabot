use std::collections::HashMap;

use twitch_irc_parser::*;

#[test]
fn starting_host() {
    let message = ":tmi.twitch.tv HOSTTARGET #abc :xyz 10";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        bot_command: None,
        command: Command::HOSTTARGET(String::from("#abc")),
        tags: HashMap::new(),
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        parameters: Some(vec![String::from("xyz"), String::from("10")]),
    };

    assert_eq!(actual, expected);
}

#[test]
fn ending_host() {
    let message = ":tmi.twitch.tv HOSTTARGET #abc :- 10";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        bot_command: None,
        command: Command::HOSTTARGET(String::from("#abc")),
        tags: HashMap::new(),
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        parameters: Some(vec![String::from("-"), String::from("10")]),
    };

    assert_eq!(actual, expected);
}
