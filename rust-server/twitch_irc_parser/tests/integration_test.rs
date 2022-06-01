use std::collections::HashMap;
use twitch_irc_parser::*;

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
        Tag::Emotes(vec![Emote::new(33, vec![TextPosition::new(0, 7)])]),
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
fn message_with_tags() {
    let message = "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame";

    let actual = parse_message(message);

    let expected_tags = create_tags();
    let expected_source = Source::new(
        Some(String::from("petsgomoo")),
        String::from("petsgomoo@petsgomoo.tmi.twitch.tv"),
    );
    let expected_command = Command::PRIVMSG(String::from("#petsgomoo"));
    let expected_bot_command: Option<BotCommand> = None;
    let expected_parameters = vec![String::from("DansGame")];

    let expected_parsed_message = ParsedTwitchMessage {
        tags: expected_tags,
        source: Some(expected_source),
        command: expected_command,
        bot_command: expected_bot_command,
        parameters: Some(expected_parameters),
    };

    assert_eq!(actual, expected_parsed_message);
}

// TODO: Fix this test
#[test]
fn ping_has_command_component() {
    let message = "PING :tmi.twitch.tv";
    let result = parse_message(message);

    let actual_command = result.command;
    let expected_command = Command::PING;
    assert_eq!(actual_command, expected_command);
}

#[test]
fn test_parse_message() {
    let message = ":lovingt3s!lovingt3s@lovingt3s.tmi.twitch.tv PRIVMSG #lovingt3s :!dilly dally";

    let result = parse_message(message);

    let expected_command = Command::PRIVMSG(String::from("#lovingt3s"));

    let expected_bot_command = BotCommand::new(String::from("dilly"), vec![String::from("dally")]);

    let expected_parameters = vec![String::from("!dilly"), String::from("dally")];

    let expected_source = Source::new(
        Some(String::from("lovingt3s")),
        String::from("lovingt3s@lovingt3s.tmi.twitch.tv"),
    );

    let actual_command = result.command;
    let actual_bot_command = result.bot_command;
    let actual_parameters = result.parameters;
    let actual_source = result.source;

    assert_eq!(actual_command, expected_command);

    assert!(actual_bot_command.is_some());
    assert_eq!(actual_bot_command.unwrap(), expected_bot_command);

    assert!(actual_parameters.is_some());
    assert_eq!(actual_parameters.unwrap(), expected_parameters);

    assert!(actual_source.is_some());
    assert_eq!(actual_source.unwrap(), expected_source);
}
