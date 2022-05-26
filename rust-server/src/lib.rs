use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tags() -> HashMap<String, Tag> {
        let mut tags = HashMap::new();

        tags.insert(
            String::from("badges"),
            Tag::Badges(vec![
                Badge::STAFF(1),
                Badge::BROADCASTER(1),
                Badge::TURBO(1),
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
                id: 33,
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
    fn ping_has_command_component() {
        let message = "PING :tmi.twitch.tv";
        let result = parse_message(message);

        let actual_command = result.command;
        let expected_command = Command::PING;
        assert_eq!(actual_command, expected_command);
    }

    #[test]
    fn test_parse_message() {
        // TODO: Add test with tags
        let message =
            ":lovingt3s!lovingt3s@lovingt3s.tmi.twitch.tv PRIVMSG #lovingt3s :!dilly dally";

        let result = parse_message(message);

        let expected_command = Command::PRIVMSG(String::from("#lovingt3s"));

        let expected_bot_command = BotCommand {
            command: String::from("dilly"),
            parameters: vec![String::from("dally")],
        };

        let expected_parameters = Parameters {
            parameters: vec![String::from("!dilly"), String::from("dally")],
        };

        let expected_source = Source {
            nick: Some(String::from("lovingt3s")),
            host: String::from("lovingt3s@lovingt3s.tmi.twitch.tv"),
        };

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

    #[test]
    fn message_with_tags() {
        let message = "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame";

        let actual = parse_message(message);

        let expected_tags = create_tags();
        let expected_source = Source {
            nick: Some(String::from("petsgomoo")),
            host: String::from("petsgomoo@petsgomoo.tmi.twitch.tv"),
        };
        let expected_command = Command::PRIVMSG(String::from("#petsgomoo"));
        let expected_bot_command: Option<BotCommand> = None;
        let expected_parameters = Parameters {
            parameters: vec![String::from("DansGame")],
        };

        let expected_parsed_message = ParsedTwitchMessage {
            tags: expected_tags,
            source: Some(expected_source),
            command: expected_command,
            bot_command: expected_bot_command,
            parameters: Some(expected_parameters),
        };

        assert_eq!(actual, expected_parsed_message);
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
            let message = "PRIVMSG #lovingt3s";

            let result = parse_command(message);

            let expected_command = Command::PRIVMSG(String::from("#lovingt3s"));

            assert_eq!(result, expected_command);
        }

        // PING
        {
            let message = "PING :tmi.twitch.tv";

            let result = parse_command(message);

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

    #[test]
    fn test_parse_parameters() {
        // Non empty string
        {
            let message = "!dilly dally";

            let actual_parameters = parse_parameters(message);

            let expected_parameters = Parameters {
                parameters: vec![String::from("!dilly"), String::from("dally")],
            };

            assert_eq!(actual_parameters, expected_parameters);
        }

        // Empty string
        {
            let message = "";

            let actual_parameters = parse_parameters(message);

            let expected_parameters = Parameters {
                parameters: vec![String::from("")],
            };

            assert_eq!(actual_parameters, expected_parameters);
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    JOIN(String),
    PART(String),
    NOTICE(String),
    CLEARCHAT(String),
    HOSTTARGET(String),
    PRIVMSG(String),
    PING,
    CAP(bool),
    GLOBALUSERSTATE,
    USERSTATE(String),
    ROOMSTATE(String),
    RECONNECT,
    NUMBER(u32, Option<String>),
    UNSUPPORTED,
}

// BADGE(version)
#[derive(PartialEq, Debug)]
pub enum Badge {
    ADMIN(usize),
    BITS(usize),
    BROADCASTER(usize),
    MODERATOR(usize),
    STAFF(usize),
    SUBSCRIBER(usize),
    TURBO(usize),
    VIP(usize),
    UNKNOWN,
}

#[derive(PartialEq, Debug)]
pub struct Source {
    nick: Option<String>,
    host: String,
}

#[derive(PartialEq, Debug)]
pub struct BotCommand {
    command: String,
    parameters: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub struct TextPosition {
    start_index: usize,
    end_index: usize,
}

#[derive(PartialEq, Debug)]
pub struct Emote {
    id: usize,
    positions: Vec<TextPosition>,
}

#[derive(PartialEq, Debug)]
pub enum UserType {
    Normal,    // A normal user - ""
    Admin,     // A Twitch administrator - "admin"
    GlobalMod, // A global moderator - "global_mod"
    Staff,     // A Twitch employee - "staff"
}

#[derive(PartialEq, Debug)]
pub enum Tag {
    Badges(Vec<Badge>),
    Color(String),
    DisplayName(String),
    EmoteOnly(bool),
    FollowersOnly(bool),
    Emotes(Vec<Emote>),
    EmoteSets(Vec<usize>),
    Id(String),
    Mod(bool),
    RoomId(String),
    Subscriber(bool),
    Turbo(bool),
    TmiSentTs(String),
    UserId(String),
    UserType(UserType),
    Unknown,
}

#[derive(PartialEq, Debug)]
pub struct Parameters {
    parameters: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub struct ParsedTwitchMessage {
    pub tags: HashMap<String, Tag>,
    pub source: Option<Source>,
    pub command: Command,
    pub parameters: Option<Parameters>,
    pub bot_command: Option<BotCommand>,
}

pub fn parse_message(message: &str) -> ParsedTwitchMessage {
    let mut idx = 0;

    // Get the tags component

    let tags = if &message[..idx + 1] == "@" {
        // Message has tags
        let end_index = message
            .find(' ')
            .expect("There should be more to a message with badges");

        let raw_tags = &message[1..end_index];

        idx = end_index + 1;
        parse_tags(raw_tags)
    } else {
        HashMap::new()
    };

    let message = &message[idx..];

    // Get the source component

    let source = if &message[..1] == ":" {
        // Message source
        let end_index = message
            .find(' ')
            .expect("There should be more to a message with a source");

        let raw_sources = &message[1..end_index];

        idx = end_index + 1;

        Some(parse_source(raw_sources))
    } else {
        None
    };

    let message = &message[idx..];

    // Get the command component

    // Looking for the parameters part of the message.
    // But not all messages include the parameter list. In that case we default to the end of the string.
    let end_index = message.find(':').unwrap_or(message.len());

    let raw_command = &message[..end_index];

    let command = parse_command(raw_command);

    // Get the parameters component.

    let raw_parameters = if end_index != message.len() {
        idx = end_index + 1;
        Some(&message[idx..])
    } else {
        None
    };

    let parameters = match raw_parameters {
        Some(params) => Some(parse_parameters(params)),
        None => None,
    };

    let bot_command = match raw_parameters {
        Some(params) => parse_bot_command(params),
        None => None,
    };

    ParsedTwitchMessage {
        tags,
        bot_command,
        command,
        parameters,
        source,
    }
}

fn parse_badges(raw_badges: &str) -> Vec<Badge> {
    let mut badges: Vec<Badge> = vec![];

    let split_badges = raw_badges.split(',');

    for raw_badge in split_badges {
        let mut badge_parts = raw_badge.split('/');
        let badge_name = badge_parts.next().expect("Badge name should exist");
        let badge_version = badge_parts.next().expect("Badge version should exist");
        let badge_version = badge_version.parse::<usize>();

        let badge_version = badge_version.unwrap_or(0);
        let badge = match badge_name {
            "admin" => Badge::ADMIN(badge_version),
            "bits" => Badge::BITS(badge_version),
            "broadcaster" => Badge::BROADCASTER(badge_version),
            "moderator" => Badge::MODERATOR(badge_version),
            "staff" => Badge::STAFF(badge_version),
            "subscriber" => Badge::SUBSCRIBER(badge_version),
            "turbo" => Badge::TURBO(badge_version),
            "vip" => Badge::VIP(badge_version),
            _ => Badge::UNKNOWN,
        };
        badges.push(badge);
    }

    badges
}

fn parse_emotes(raw_emotes: &str) -> Vec<Emote> {
    let mut emotes: Vec<Emote> = vec![];

    let split_emotes = raw_emotes.split('/');
    for raw_emote in split_emotes {
        let mut emote_parts = raw_emote.split(':');

        let emote_id = emote_parts
            .next()
            .expect("Should have emote ID")
            .parse::<usize>()
            .expect("Should be an integer");

        let mut text_positions: Vec<TextPosition> = vec![];

        let positions = emote_parts
            .next()
            .expect("Should have at least one position")
            .split(',');

        for position in positions {
            let mut position_parts = position.split('-');
            let start_index = position_parts
                .next()
                .expect("Should have start index")
                .parse::<usize>()
                .expect("Should be an integer");
            let end_index = position_parts
                .next()
                .expect("Should have end index")
                .parse::<usize>()
                .expect("Should be an integer");
            let text_position = TextPosition {
                start_index,
                end_index,
            };
            text_positions.push(text_position);
        }

        let emote = Emote {
            id: emote_id,
            positions: text_positions,
        };
        emotes.push(emote);
    }

    emotes
}

fn parse_emote_sets(raw_emote_sets: &str) -> Vec<usize> {
    let mut emote_sets: Vec<usize> = vec![];

    let split_emote_sets = raw_emote_sets.split(',');
    for raw_emote_set in split_emote_sets {
        let emote_set = raw_emote_set
            .parse::<usize>()
            .expect("Should be an integer");
        emote_sets.push(emote_set);
    }

    emote_sets
}

fn parse_tags(raw_tags: &str) -> HashMap<String, Tag> {
    let parsed_tags = raw_tags.split(';');

    let mut tags: HashMap<String, Tag> = HashMap::new();

    for parsed_tag in parsed_tags {
        let mut split_tag = parsed_tag.split('=');

        let tag_key = split_tag.next().expect("Should contain at least a key");
        let tag_value = split_tag.next();

        let tag = match tag_key {
            "badges" | "badge-info" => match tag_value {
                Some(value) => {
                    let badges = parse_badges(value);
                    Tag::Badges(badges)
                }
                None => Tag::Badges(vec![]),
            },
            "color" => match tag_value {
                Some(value) => Tag::Color(value.to_string()),
                None => Tag::Color(String::from("")),
            },
            "display-name" => match tag_value {
                Some(value) => Tag::DisplayName(value.to_string()),
                None => Tag::DisplayName(String::from("")),
            },
            "emote-only" => match tag_value {
                Some(value) => {
                    let emote_only = match value {
                        "1" => true,
                        _ => false,
                    };
                    Tag::EmoteOnly(emote_only)
                }
                None => Tag::EmoteOnly(false),
            },
            "emotes" => match tag_value {
                Some(value) => {
                    let emotes = parse_emotes(value);
                    Tag::Emotes(emotes)
                }
                None => Tag::Emotes(vec![]),
            },
            "emote-sets" => match tag_value {
                Some(value) => {
                    let emote_sets = parse_emote_sets(value);
                    Tag::EmoteSets(emote_sets)
                }
                None => Tag::EmoteSets(vec![]),
            },
            "id" => match tag_value {
                Some(value) => Tag::Id(value.to_string()),
                None => Tag::Id(String::from("0")),
            },
            "mod" => match tag_value {
                Some(value) => {
                    let r#mod = match value {
                        "1" => true,
                        _ => false,
                    };
                    Tag::Mod(r#mod)
                }
                None => Tag::Mod(false),
            },
            "room-id" => match tag_value {
                Some(value) => Tag::RoomId(value.to_string()),
                None => Tag::RoomId(String::from("0")),
            },
            "subscriber" => match tag_value {
                Some(value) => {
                    let subscriber = match value {
                        "1" => true,
                        _ => false,
                    };
                    Tag::Subscriber(subscriber)
                }
                None => Tag::Subscriber(false),
            },
            "turbo" => match tag_value {
                Some(value) => {
                    let turbo = match value {
                        "1" => true,
                        _ => false,
                    };
                    Tag::Turbo(turbo)
                }
                None => Tag::Turbo(false),
            },
            "tmi-sent-ts" => match tag_value {
                Some(value) => Tag::TmiSentTs(value.to_string()),
                None => Tag::TmiSentTs(String::from("0")),
            },
            "user-id" => match tag_value {
                Some(value) => Tag::UserId(value.to_string()),
                None => Tag::UserId(String::from("0")),
            },
            "user-type" => match tag_value {
                Some(value) => {
                    let user_type = match value.trim() {
                        "admin" => UserType::Admin,
                        "global_mod" => UserType::GlobalMod,
                        "staff" => UserType::Staff,
                        _ => UserType::Normal,
                    };
                    Tag::UserType(user_type)
                }
                None => Tag::UserType(UserType::Normal),
            },
            _ => Tag::Unknown,
        };

        tags.insert(tag_key.to_string(), tag);
    }

    tags
}

fn parse_source(raw_source: &str) -> Source {
    let mut split_source = raw_source.split('!');

    let first = split_source.next().expect("Should have at least one part");
    let second = split_source.next();

    if second.is_some() {
        Source {
            nick: Some(first.to_string()),
            host: second.unwrap().to_string(),
        }
    } else {
        Source {
            nick: None,
            host: first.to_string(),
        }
    }
}

fn parse_parameters(raw_parameters: &str) -> Parameters {
    let split_params = raw_parameters.split(" ");
    let parameters: Vec<String> = split_params.map(|param| String::from(param)).collect();
    Parameters { parameters }
}

fn parse_command(raw_command: &str) -> Command {
    let mut command_parts = raw_command.split(' ');

    let command = command_parts.next().expect("This should be the command");

    match command {
        "PING" => Command::PING,
        "PRIVMSG" => {
            let channel = command_parts.next().expect("This should exist");
            Command::PRIVMSG(channel.to_string())
        }
        _ => Command::UNSUPPORTED,
    }
}

fn parse_bot_command(raw_bot_command: &str) -> Option<BotCommand> {
    // Example: "dilly dally wally"
    // Command: "dilly"
    // Parameters: ["dally", "wally"]
    let bot_command = raw_bot_command.trim();

    if bot_command.len() == 0 {
        return None;
    }

    let first_char = &bot_command[..1];

    if first_char != "!" {
        // Not a bot command if not "!" first
        return None;
    }

    let bot_command = &bot_command[1..];

    if bot_command.len() == 0 {
        // "!" is not a command
        return None;
    }

    if &bot_command[..1] == " " {
        // "! command" is not a command
        return None;
    }

    let have_params = bot_command.find(' ').is_some();

    if have_params {
        let mut derp = bot_command.split(' ');
        let command = derp.next().expect("There should be a str here").to_string();
        let parameters: Vec<String> = derp.map(|param| String::from(param)).collect();
        Some(BotCommand {
            command,
            parameters,
        })
    } else {
        Some(BotCommand {
            command: bot_command.to_string(),
            parameters: vec![],
        })
    }
}
