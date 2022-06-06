use std::collections::HashMap;
use twitch_irc_parser::*;

#[test]
fn message_without_tags() {
    let message = ":tmi.twitch.tv GLOBALUSERSTATE";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        command: Command::GLOBALUSERSTATE { tags: None },
        source: Some(Source::new(String::from("tmi.twitch.tv"), None)),
    };

    assert_eq!(actual, expected);
}
#[test]
fn message_with_tags() {
    let message = "@badge-info=subscriber/8;badges=subscriber/6;color=#0D4200;display-name=dallas;emote-sets=0,33,50,237,793,2126,3517,4578,5569,9400,10337,12239;turbo=0;user-id=12345678;user-type=admin :tmi.twitch.tv GLOBALUSERSTATE";

    let actual = parse_message(message);

    let mut expected_tags: HashMap<String, Tag> = HashMap::new();
    expected_tags.insert(String::from("badge-info"), Tag::BadgeInfo(8));
    expected_tags.insert(
        String::from("badges"),
        Tag::Badges(vec![Badge::Subscriber(6)]),
    );
    expected_tags.insert(String::from("color"), Tag::Color(String::from("#0D4200")));
    expected_tags.insert(
        String::from("display-name"),
        Tag::DisplayName(String::from("dallas")),
    );
    expected_tags.insert(
        String::from("emote-sets"),
        Tag::EmoteSets(vec![
            0, 33, 50, 237, 793, 2126, 3517, 4578, 5569, 9400, 10337, 12239,
        ]),
    );
    expected_tags.insert(String::from("turbo"), Tag::Turbo(false));
    expected_tags.insert(
        String::from("user-id"),
        Tag::UserId(String::from("12345678")),
    );
    expected_tags.insert(String::from("user-type"), Tag::UserType(UserType::Admin));

    let expected_command = Command::GLOBALUSERSTATE {
        tags: Some(expected_tags),
    };

    let expected_source = Source::new(String::from("tmi.twitch.tv"), None);

    let expected = ParsedTwitchMessage {
        source: Some(expected_source),
        command: expected_command,
    };

    assert_eq!(actual, expected);
}
