use std::collections::HashMap;
use twitch_irc_parser::*;

//
// The following example shows the message that the Twitch IRC server sent after the moderator deleted ronni’s HeyGuys message from the dallas chat room.
//
#[test]
fn test_clear_single_message() {
    let message = "@login=ronni;room-id=;target-msg-id=abc-123-def;tmi-sent-ts=1642720582342 :tmi.twitch.tv CLEARMSG #dallas :HeyGuys";

    let actual = parse_message(message);

    let mut expected_tags: HashMap<String, Tag> = HashMap::new();
    expected_tags.insert(String::from("login"), Tag::Login(String::from("ronni")));
    expected_tags.insert(String::from("room-id"), Tag::RoomId(String::from("")));
    expected_tags.insert(
        String::from("target-msg-id"),
        Tag::TargetMsgId(String::from("abc-123-def")),
    );
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1642720582342")),
    );

    let expected_command = Command::CLEARMSG {
        channel: String::from("dallas"),
        message: String::from("HeyGuys"),
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
