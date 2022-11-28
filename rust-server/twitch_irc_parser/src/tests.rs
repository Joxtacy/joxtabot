use pretty_assertions::assert_eq;

use super::*;

use utils::*;

fn create_tags() -> HashMap<String, Tag> {
    let mut tags = HashMap::new();

    tags.insert(
        String::from("badges"),
        Tag::Badges(vec![
            Badge::Staff(1),
            Badge::Broadcaster(1),
            Badge::Turbo(1),
        ]),
    );
    tags.insert(String::from("color"), Tag::Color(String::from("#FF0000")));
    tags.insert(
        String::from("display-name"),
        Tag::DisplayName(String::from("PetsgomOO")),
    );
    tags.insert(String::from("emote-only"), Tag::EmoteOnly(true));
    tags.insert(
        String::from("emotes"),
        Tag::Emotes(vec![Emote {
            id: String::from("33"),
            positions: vec![TextPosition {
                start_index: 0,
                end_index: 7,
            }],
        }]),
    );
    tags.insert(String::from("flags"), Tag::Unknown);
    tags.insert(
        String::from("id"),
        Tag::Id(String::from("c285c9ed-8b1b-4702-ae1c-c64d76cc74ef")),
    );
    tags.insert(String::from("mod"), Tag::Mod(false));
    tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("81046256")),
    );
    tags.insert(String::from("subscriber"), Tag::Subscriber(false));
    tags.insert(String::from("turbo"), Tag::Turbo(false));
    tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1550868292494")),
    );
    tags.insert(
        String::from("user-id"),
        Tag::UserId(String::from("81046256")),
    );
    tags.insert(String::from("user-type"), Tag::UserType(UserType::Staff));

    tags
}

#[test]
fn test_parse_tags() {
    let message = "badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff";

    let actual = parse_tags(message);

    let expected_tags = create_tags();

    assert_eq!(actual, expected_tags);
}

#[test]
fn test_parse_source() {
    let message = "petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv";

    let actual_source = parse_source(message);

    let expected_source = Source {
        nick: Some(String::from("petsgomoo")),
        host: String::from("petsgomoo@petsgomoo.tmi.twitch.tv"),
    };

    assert_eq!(actual_source, expected_source);
}

#[test]
fn test_parse_command() {
    // PRIVMSG
    {
        let message = "PRIVMSG #lovingt3s :herp derp";

        let result = parse_command(message, "");

        let expected_command = Command::PRIVMSG {
            channel: String::from("lovingt3s"),
            message: String::from("herp derp"),
            bot_command: None,
            tags: None,
        };

        assert_eq!(result, expected_command);
    }

    // PING
    {
        let message = "PING :tmi.twitch.tv";

        let result = parse_command(message, "");

        let expected_command = Command::PING;

        assert_eq!(result, expected_command);
    }
}

#[test]
fn test_parse_bot_command() {
    // With a bot command
    {
        let message = "!dilly dally wally";

        let result = parse_bot_command(message);

        let expected_bot_command = BotCommand {
            command: String::from("dilly"),
            parameters: vec![String::from("dally"), String::from("wally")],
        };

        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected_bot_command);
    }

    // With empty string
    {
        let message = "";

        let result = parse_bot_command(message);

        assert!(result.is_none());
    }

    // With a "!" and space after
    {
        let message = "! not a bot command";

        let result = parse_bot_command(message);

        assert!(result.is_none());
    }
}
