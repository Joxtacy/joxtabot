#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_has_command_component() {
        let message = "PING :tmi.twitch.tv";
        let result = parse_message(message);

        let actual_command = result.command;
        let expected_command = Commands::PING;
        assert_eq!(actual_command, expected_command);
    }

    #[test]
    fn test_parse_message() {
        // TODO: Add test with tags
        let message =
            ":lovingt3s!lovingt3s@lovingt3s.tmi.twitch.tv PRIVMSG #lovingt3s :!dilly dally";

        let result = parse_message(message);

        let expected_command = Commands::PRIVMSG(String::from("#lovingt3s"));

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

        let result = parse_message(message);

        // TODO: Fix this test
        panic!("IMPLEMENT ME!");
    }

    #[test]
    fn test_parse_tags() {
        let message = "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame";

        let actual = parse_tags(message);

        let expected_tags = Tags {
            badges: vec![Badges::STAFF(1), Badges::BROADCASTER(1), Badges::TURBO(1)],
            color: String::from("#FF0000"),
            display_name: String::from("PetsgomOO"),
            emote_only: true,
            emotes: vec![Emote {
                id: 33,
                positions: vec![TextPosition {
                    start_index: 0,
                    end_index: 7,
                }],
            }],
            emote_sets: vec![],
            followers_only: false,
            id: String::from("c285c9ed-8b1b-4702-ae1c-c64d76cc74ef"),
            r#mod: false,
            room_id: String::from("81046256"),
            subscriber: false,
            turbo: false,
            tmi_sent_ts: String::from("1550868292494"),
            user_id: String::from("81046256"),
            user_type: UserType::Staff,
        };

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

            let expected_command = Commands::PRIVMSG(String::from("#lovingt3s"));

            assert_eq!(result, expected_command);
        }

        // PING
        {
            let message = "PING :tmi.twitch.tv";

            let result = parse_command(message);

            let expected_command = Commands::PING;

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
pub enum Commands {
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
pub enum Badges {
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
pub struct Tags {
    badges: Vec<Badges>,
    color: String,
    display_name: String, // display-name
    emote_only: bool,     // emote-only
    followers_only: bool, // followers-only
    emotes: Vec<Emote>,
    emote_sets: Vec<usize>,
    id: String,
    r#mod: bool,     // mod
    room_id: String, // room-id
    subscriber: bool,
    turbo: bool,
    tmi_sent_ts: String, // tmi-sent-ts
    user_id: String,     // user-id
    user_type: UserType, // user-type
}

#[derive(PartialEq, Debug)]
pub struct Parameters {
    parameters: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub struct ParsedTwitchMessage {
    pub tags: Option<Tags>,
    pub source: Option<Source>,
    pub command: Commands,
    pub parameters: Option<Parameters>,
    pub bot_command: Option<BotCommand>,
}

pub fn parse_message(message: &str) -> ParsedTwitchMessage {
    let mut idx = 0;

    // Get the tags component

    if &message[..idx + 1] == "@" {
        // Message has tags
        let end_index = message
            .find(' ')
            .expect("There should be more to a message with badges");

        let raw_tags = &message[1..end_index];

        idx = end_index + 1;
    }

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
        tags: None,
        bot_command,
        command,
        parameters,
        source,
    }
}

fn parse_badges(raw_badges: &str) -> Vec<Badges> {
    let mut badges: Vec<Badges> = vec![];

    let split_badges = raw_badges.split(',');

    for raw_badge in split_badges {
        let mut badge_parts = raw_badge.split('/');
        let badge_name = badge_parts.next().expect("Badge name should exist");
        let badge_version = badge_parts.next().expect("Badge version should exist");
        let badge_version = badge_version.parse::<usize>();

        let badge_version = badge_version.unwrap_or(0);
        let badge = match badge_name {
            "admin" => Badges::ADMIN(badge_version),
            "bits" => Badges::BITS(badge_version),
            "broadcaster" => Badges::BROADCASTER(badge_version),
            "moderator" => Badges::MODERATOR(badge_version),
            "staff" => Badges::STAFF(badge_version),
            "subscriber" => Badges::SUBSCRIBER(badge_version),
            "turbo" => Badges::TURBO(badge_version),
            "vip" => Badges::VIP(badge_version),
            _ => Badges::UNKNOWN,
        };
        badges.push(badge);
    }

    println!("BADGES: {:?}", badges);

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

    println!("EMOTES: {:?}", emotes);
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

    println!("EMOTE-SETS: {:?}", emote_sets);
    emote_sets
}

fn parse_tags(raw_tags: &str) -> Tags {
    let raw_tags = &raw_tags[1..];
    let parsed_tags = raw_tags.split(';');

    for parsed_tag in parsed_tags {
        println!("parsed_tag: {}", parsed_tag);
        let mut split_tag = parsed_tag.split('=');

        let tag_key = split_tag.next().expect("Should contain at least a key");
        let tag_value = split_tag.next();

        match tag_key {
            "badges" | "badge-info" => match tag_value {
                Some(value) => {
                    parse_badges(value);
                }
                None => (),
            },
            "color" => match tag_value {
                Some(value) => {
                    let color = value;
                    println!("COLOR: {}", color);
                }
                None => (),
            },
            "display-name" => match tag_value {
                Some(value) => {
                    let display_name = value;
                    println!("DISPLAY-NAME: {}", display_name);
                }
                None => (),
            },
            "emote-only" => match tag_value {
                Some(value) => {
                    let emote_only = match value {
                        "1" => true,
                        _ => false,
                    };
                    println!("EMOTE-ONLY: {}", emote_only);
                }
                None => (),
            },
            "emotes" => match tag_value {
                Some(value) => {
                    parse_emotes(value);
                }
                None => (),
            },
            "emote-sets" => match tag_value {
                Some(value) => {
                    parse_emote_sets(value);
                }
                None => (),
            },
            "id" => match tag_value {
                Some(value) => {
                    let id = value;
                    println!("ID: {}", id);
                }
                None => (),
            },
            "mod" => match tag_value {
                Some(value) => {
                    let r#mod = match value {
                        "1" => true,
                        _ => false,
                    };
                    println!("MOD: {}", r#mod);
                }
                None => (),
            },
            "room-id" => match tag_value {
                Some(value) => {
                    let room_id = value.parse::<usize>().expect("Should be an integer");
                    println!("ROOM-ID: {}", room_id);
                }
                None => (),
            },
            "subscriber" => match tag_value {
                Some(value) => {
                    let subscriber = value.parse::<usize>().expect("Should be an integer");
                    println!("SUBSCRIBER: {}", subscriber);
                }
                None => (),
            },
            "turbo" => match tag_value {
                Some(value) => {
                    let turbo = value.parse::<usize>().expect("Should be an integer");
                    println!("TURBO: {}", turbo);
                }
                None => (),
            },
            "tmi-sent-ts" => match tag_value {
                Some(value) => {
                    let tmi_sent_ts = value;
                    println!("TMI-SENT-TS: {}", tmi_sent_ts);
                }
                None => (),
            },
            "user-id" => match tag_value {
                Some(value) => {
                    let user_id = value;
                    println!("USER-ID: {}", user_id);
                }
                None => (),
            },
            "user-type" => match tag_value {
                Some(value) => {
                    let user_type = match value {
                        "admin" => UserType::Admin,
                        "global_mod" => UserType::GlobalMod,
                        "staff" => UserType::Staff,
                        _ => UserType::Normal,
                    };
                    println!("USER-TYPE: {:?}", user_type);
                }
                None => (),
            },
            _ => (),
        }
    }

    Tags {
        badges: vec![],
        color: String::from("#b000b5"),
        display_name: String::from("Display-Name"),
        emote_only: false,
        emote_sets: vec![],
        followers_only: false,
        emotes: vec![],
        id: String::from("id"),
        r#mod: false,
        room_id: String::from("room-id"),
        subscriber: false,
        turbo: false,
        tmi_sent_ts: String::from("The UNIX timestamp"),
        user_id: String::from("user-id"),
        user_type: UserType::Normal,
    }
}

fn parse_source(raw_source: &str) -> Source {
    let split_source = raw_source.split('!');

    let the_vec = split_source.collect::<Vec<&str>>();

    if the_vec.len() == 1 {
        Source {
            nick: None,
            host: String::from(the_vec[0]),
        }
    } else {
        Source {
            nick: Some(String::from(the_vec[0])),
            host: String::from(the_vec[1]),
        }
    }
}

fn parse_parameters(raw_parameters: &str) -> Parameters {
    let split_params = raw_parameters.split(" ");
    let parameters: Vec<String> = split_params.map(|param| String::from(param)).collect();
    Parameters { parameters }
}

#[derive(Debug)]
struct CommandCommand {
    command: String,
    channel: Option<String>,
}

fn parse_command(raw_command: &str) -> Commands {
    let mut command_parts = raw_command.split(' ');

    let command = command_parts.next().expect("This should be the command");

    match command {
        "PING" => Commands::PING,
        "PRIVMSG" => {
            let channel = command_parts.next().expect("This should exist");
            Commands::PRIVMSG(channel.to_string())
        }
        _ => Commands::UNSUPPORTED,
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
