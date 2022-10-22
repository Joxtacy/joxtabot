use pretty_assertions::assert_eq;
use std::collections::HashMap;

use twitch_irc_parser::*;

#[test]
fn message_deleted() {
    let message = ":tmi.twitch.tv NOTICE #bar :The message from foo is now deleted.";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        command: Command::NOTICE {
            channel: String::from("bar"),
            message: String::from("The message from foo is now deleted."),
            tags: None,
        },
        source: Some(Source {
            host: String::from("tmi.twitch.tv"),
            nick: None,
        }),
    };

    assert_eq!(actual, expected);
}

#[test]
fn user_banned() {
    let message = ":tmi.twitch.tv NOTICE #bar :foo is now banned from this channel.";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        command: Command::NOTICE {
            channel: String::from("bar"),
            message: String::from("foo is now banned from this channel."),
            tags: None,
        },
        source: Some(Source {
            host: String::from("tmi.twitch.tv"),
            nick: None,
        }),
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
        command: Command::NOTICE {
            channel: String::from("bar"),
            message: String::from("Your settings prevent you from sending this whisper."),
            tags: Some(expected_tags),
        },
        source: Some(Source {
            host: String::from("tmi.twitch.tv"),
            nick: None,
        }),
    };

    assert_eq!(actual, expected);
}
