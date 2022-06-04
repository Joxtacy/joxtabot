use std::collections::HashMap;
use twitch_irc_parser::*;

#[test]
fn no_tags() {
    let message = ":tmi.twitch.tv ROOMSTATE #bar";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        bot_command: None,
        parameters: None,
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        command: Command::ROOMSTATE(String::from("#bar")),
        tags: HashMap::new(),
    };

    assert_eq!(actual, expected);
}

#[test]
fn with_tags() {
    let message =
        "@emote-only=0;followers-only=-1;r9k=0;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #dallas";

    let actual = parse_message(message);

    let mut expected_tags = HashMap::new();
    expected_tags.insert(String::from("emote-only"), Tag::EmoteOnly(false));
    expected_tags.insert(String::from("followers-only"), Tag::FollowersOnly(-1));
    expected_tags.insert(String::from("r9k"), Tag::R9K(false));
    expected_tags.insert(String::from("slow"), Tag::Slow(0));
    expected_tags.insert(String::from("subs-only"), Tag::SubsOnly(false));

    let expected = ParsedTwitchMessage {
        bot_command: None,
        parameters: None,
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        command: Command::ROOMSTATE(String::from("#dallas")),
        tags: expected_tags,
    };

    assert_eq!(actual, expected);
}
