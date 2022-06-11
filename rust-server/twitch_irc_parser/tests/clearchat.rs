use std::collections::HashMap;
use twitch_irc_parser::*;

//
// The following example shows the message that the Twitch IRC server sent after dallas permanently banned ronni from the chat room and removed all of ronni’s messages.
//
#[test]
fn test_perma_ban_user_message() {
    let message = "@room-id=12345678;target-user-id=87654321;tmi-sent-ts=1642715756806 :tmi.twitch.tv CLEARCHAT #dallas :ronni";

    let actual = parse_message(message);

    let mut expected_tags: HashMap<String, Tag> = HashMap::new();
    expected_tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("12345678")),
    );
    expected_tags.insert(
        String::from("target-user-id"),
        Tag::TargetUserId(String::from("87654321")),
    );
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1642715756806")),
    );

    let expected_command = Command::CLEARCHAT {
        channel: String::from("dallas"),
        user: Some(String::from("ronni")),
        tags: Some(expected_tags),
    };

    let expected_source = Source {
        host: String::from("tmi.twitch.tv"),
        nick: None,
    };

    let expected = ParsedTwitchMessage {
        command: expected_command,
        source: Some(expected_source),
    };

    assert_eq!(actual, expected);
}

//
// The following example shows the message that the Twitch IRC server sent after dallas removed all messages from the chat room.
//
#[test]
fn test_clear_message_from_chat_room() {
    let message = "@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARCHAT #dallas";

    let actual = parse_message(message);

    let mut expected_tags: HashMap<String, Tag> = HashMap::new();
    expected_tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("12345678")),
    );
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1642715695392")),
    );

    let expected_command = Command::CLEARCHAT {
        channel: String::from("dallas"),
        user: None,
        tags: Some(expected_tags),
    };

    let expected_source = Source {
        host: String::from("tmi.twitch.tv"),
        nick: None,
    };

    let expected = ParsedTwitchMessage {
        command: expected_command,
        source: Some(expected_source),
    };

    assert_eq!(actual, expected);
}

//
// The following example shows the message that the Twitch IRC server sent after dallas put ronni in a timeout and removed all of ronni’s messages from the chat room.
//
#[test]
fn test_timeout_user() {
    let message = "@ban-duration=350;room-id=12345678;target-user-id=87654321;tmi-sent-ts=1642719320727 :tmi.twitch.tv CLEARCHAT #dallas :ronni";

    let actual = parse_message(message);

    let mut expected_tags: HashMap<String, Tag> = HashMap::new();
    expected_tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("12345678")),
    );
    expected_tags.insert(
        String::from("target-user-id"),
        Tag::TargetUserId(String::from("87654321")),
    );
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1642719320727")),
    );
    expected_tags.insert(String::from("ban-duration"), Tag::BanDuration(350));

    let expected_command = Command::CLEARCHAT {
        channel: String::from("dallas"),
        user: Some(String::from("ronni")),
        tags: Some(expected_tags),
    };

    let expected_source = Source {
        host: String::from("tmi.twitch.tv"),
        nick: None,
    };

    let expected = ParsedTwitchMessage {
        command: expected_command,
        source: Some(expected_source),
    };

    assert_eq!(actual, expected);
}
