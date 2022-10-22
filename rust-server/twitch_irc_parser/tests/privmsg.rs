use pretty_assertions::assert_eq;
use std::collections::HashMap;

use twitch_irc_parser::*;

#[test]
fn simple_message() {
    let message = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        command: Command::PRIVMSG {
            channel: String::from("bar"),
            message: String::from("bleedPurple"),
            bot_command: None,
            tags: None,
        },
        source: Some(Source {
            host: String::from("foo@foo.tmi.twitch.tv"),
            nick: Some(String::from("foo")),
        }),
    };

    assert_eq!(actual, expected);
}

#[test]
fn message_with_tags() {
    let message = "@badge-info=;badges=broadcaster/1;client-nonce=28e05b1c83f1e916ca1710c44b014515;color=#0000FF;display-name=foofoo;emotes=62835:0-10;first-msg=0;flags=;id=f80a19d6-e35a-4273-82d0-cd87f614e767;mod=0;room-id=713936733;subscriber=0;tmi-sent-ts=1642696567751;turbo=0;user-id=713936733;user-type= :foofoo!foofoo@foofoo.tmi.twitch.tv PRIVMSG #bar :bleedPurple";

    let actual = parse_message(message);

    let mut expected_tags = HashMap::new();
    expected_tags.insert(String::from("badge-info"), Tag::BadgeInfo(0));
    expected_tags.insert(
        String::from("badges"),
        Tag::Badges(vec![Badge::Broadcaster(1)]),
    );
    expected_tags.insert(
        String::from("client-nonce"),
        Tag::ClientNonce(String::from("28e05b1c83f1e916ca1710c44b014515")),
    );
    expected_tags.insert(String::from("color"), Tag::Color(String::from("#0000FF")));
    expected_tags.insert(
        String::from("display-name"),
        Tag::DisplayName(String::from("foofoo")),
    );
    expected_tags.insert(
        String::from("emotes"),
        Tag::Emotes(vec![Emote {
            id: 62835,
            positions: vec![TextPosition {
                start_index: 0,
                end_index: 10,
            }],
        }]),
    );
    expected_tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("713936733")),
    );
    expected_tags.insert(
        String::from("id"),
        Tag::Id(String::from("f80a19d6-e35a-4273-82d0-cd87f614e767")),
    );
    expected_tags.insert(String::from("first-msg"), Tag::FirstMsg(false));
    expected_tags.insert(String::from("flags"), Tag::Unknown);
    expected_tags.insert(String::from("mod"), Tag::Mod(false));
    expected_tags.insert(String::from("subscriber"), Tag::Subscriber(false));
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1642696567751")),
    );
    expected_tags.insert(String::from("turbo"), Tag::Turbo(false));
    expected_tags.insert(
        String::from("user-id"),
        Tag::UserId(String::from("713936733")),
    );
    expected_tags.insert(String::from("user-type"), Tag::UserType(UserType::Normal));

    let expected = ParsedTwitchMessage {
        source: Some(Source {
            host: String::from("foofoo@foofoo.tmi.twitch.tv"),
            nick: Some(String::from("foofoo")),
        }),
        command: Command::PRIVMSG {
            channel: String::from("bar"),
            message: String::from("bleedPurple"),
            bot_command: None,
            tags: Some(expected_tags),
        },
    };

    assert_eq!(actual, expected);
}
