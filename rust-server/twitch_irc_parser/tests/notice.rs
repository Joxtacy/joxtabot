use std::collections::HashMap;

use twitch_irc_parser::*;

#[test]
fn message_deleted() {
    let message = ":tmi.twitch.tv NOTICE #bar :The message from foo is now deleted.";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        tags: HashMap::new(),
        parameters: Some(vec![
            String::from("The"),
            String::from("message"),
            String::from("from"),
            String::from("foo"),
            String::from("is"),
            String::from("now"),
            String::from("deleted."),
        ]),
        command: Command::NOTICE(String::from("#bar")),
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        bot_command: None,
    };

    assert_eq!(actual, expected);
}

#[test]
fn user_banned() {
    let message = ":tmi.twitch.tv NOTICE #bar :foo is now banned from this channel.";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        tags: HashMap::new(),
        parameters: Some(vec![
            String::from("foo"),
            String::from("is"),
            String::from("now"),
            String::from("banned"),
            String::from("from"),
            String::from("this"),
            String::from("channel."),
        ]),
        command: Command::NOTICE(String::from("#bar")),
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        bot_command: None,
    };

    assert_eq!(actual, expected);
}

#[test]
fn unable_to_send_whisper() {
    let message = "@msg-id=whisper_restricted;target-user-id=12345678 :tmi.twitch.tv NOTICE #bar :Your settings prevent you from sending this whisper.";

    let actual = parse_message(message);

    let mut expected_tags = HashMap::new();
    expected_tags.insert(
        String::from("msg-id"),
        Tag::MsgId(String::from("whisper_restricted")),
    );
    expected_tags.insert(
        String::from("target-user-id"),
        Tag::TargetUserId(String::from("12345678")),
    );
    let expected = ParsedTwitchMessage {
        tags: expected_tags,
        parameters: Some(vec![
            String::from("Your"),
            String::from("settings"),
            String::from("prevent"),
            String::from("you"),
            String::from("from"),
            String::from("sending"),
            String::from("this"),
            String::from("whisper."),
        ]),
        command: Command::NOTICE(String::from("#bar")),
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        bot_command: None,
    };

    assert_eq!(actual, expected);
}
