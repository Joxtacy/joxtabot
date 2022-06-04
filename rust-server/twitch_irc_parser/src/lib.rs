//! # Twitch IRC Parser
//!
//! `twitch_irc_parser` contains the utilities needed to parse a single Twitch Message.
//! It is based on the parser example found on the Twitch Developer site.
//! <https://dev.twitch.tv/docs/irc/example-parser>

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

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

            let expected_parameters = vec![String::from("!dilly"), String::from("dally")];

            assert_eq!(actual_parameters, expected_parameters);
        }

        // Empty string
        {
            let message = "";

            let actual_parameters = parse_parameters(message);

            let expected_parameters = vec![String::from("")];

            assert_eq!(actual_parameters, expected_parameters);
        }
    }
}

/// Representation of the message command.
// TODO: What are the values in the enums?
// TODO: Might change each command to have the full message information directly in it.
#[derive(PartialEq, Debug)]
pub enum Command {
    /// A JOIN type message. The `String` is the channel name.
    JOIN(String),
    /// A PART type message. The `String` is the channel name.
    PART(String),
    /// A NOTICE type message. The `String` is the channel name.
    NOTICE(String),
    /// A CLEARCHAT type message. The `String` is the channel name.
    CLEARCHAT(String),
    /// A CLEARMSG type message. The `String` is the channel name.
    CLEARMSG(String),
    /// A HOSTTARGET type message. The `String` is the channel name.
    HOSTTARGET(String),
    /// A PRIVMSG type message. The `String` is the channel name.
    PRIVMSG(String),
    /// A PING type message.
    PING,
    // TODO: What is the `bool`?
    /// A CAP type message.
    CAP(bool),
    /// A GLOBALUSERSTATE type message.
    GLOBALUSERSTATE,
    /// A USERSTATE type message. The `String` is the channel name.
    USERSTATE(String),
    /// A ROOMSTATE type message. The `String` is the channel name.
    ROOMSTATE(String),
    /// A RECONNECT type message.
    RECONNECT,
    // TODO: what is the `Option<String>`?
    /// A NUMBER type message. The `u32` is the message number.
    NUMBER(u32, Option<String>),
    /// Unsupported message type.
    UNSUPPORTED,
}

/// Representation of the different badges a user can have.
///
/// The `usize` represents the version of the badge. Most often `1`, but for the `Subscriber` badge it represents how long the user has been subscribed in months.
#[derive(PartialEq, Debug)]
pub enum Badge {
    /// An admin badge. The `usize` represents the version of the badge.
    Admin(usize),

    /// A bits badge. The `usize` represents the version of the badge.
    Bits(usize),

    /// A broadcaster badge. The `usize` represents the version of the badge.
    Broadcaster(usize),

    /// A moderator badge. The `usize` represents the version of the badge.
    Moderator(usize),

    /// A staff badge. The `usize` represents the version of the badge.
    Staff(usize),

    /// A subscriber badge. The `usize` represents the version of the badge.
    Subscriber(usize),

    /// A turbo badge. The `usize` represents the version of the badge.
    Turbo(usize),

    /// A vip badge. The `usize` represents the version of the badge.
    Vip(usize),

    /// Unknown. Used for badges that are not able to be parsed.
    Unknown,
}

/// Representation of the source of the message.
#[derive(PartialEq, Debug)]
pub struct Source {
    nick: Option<String>,
    host: String,
}

impl Source {
    /// Creates a new instance of the `Source` struct.
    ///
    /// # Examples
    /// ```
    /// let source = twitch_irc_parser::Source::new(
    ///     Some(String::from("joxtacy")),
    ///     String::from("joxtacy@joxtacy.tmi.twitch.tv")
    /// );
    /// ```
    pub fn new(nick: Option<String>, host: String) -> Source {
        Source { nick, host }
    }
}

/// Represents a bot command.
/// Bot commands are `PRIVMSG`s that begin with an exclamation point (`!`) directly followed by the command.
/// The parameters to the command will be a vector of the rest of the words in the message delimited by whitespaces.
#[derive(PartialEq, Debug)]
pub struct BotCommand {
    command: String,
    parameters: Vec<String>,
}

impl BotCommand {
    pub fn new(command: String, parameters: Vec<String>) -> BotCommand {
        BotCommand {
            command,
            parameters,
        }
    }
}

/// Represents the start and end index in a string.
#[derive(PartialEq, Debug)]
pub struct TextPosition {
    start_index: usize,
    end_index: usize,
}

impl TextPosition {
    pub fn new(start_index: usize, end_index: usize) -> TextPosition {
        TextPosition {
            start_index,
            end_index,
        }
    }
}

/// Represents an emotes id and where in the message this emote is.
#[derive(PartialEq, Debug)]
pub struct Emote {
    id: usize,
    positions: Vec<TextPosition>,
}

impl Emote {
    pub fn new(id: usize, positions: Vec<TextPosition>) -> Emote {
        Emote { id, positions }
    }
}

/// Represents what type of user sent the message.
#[derive(PartialEq, Debug)]
pub enum UserType {
    /// A normal user.
    Normal,// ""

    /// A Twitch administrator.
    Admin, // "admin"

    /// A global moderator.
    GlobalMod, // "global_mod"

    /// A Twitch employee.
    Staff, // "staff"
}

/// Represents a tag in a message.
/// Tags are found directly after an at (`@`) symbol.
// TODO: What are the values in the enums?
#[derive(PartialEq, Debug)]
pub enum Tag {
    /// Currently only holds how long a user has been subscribed in months.
    /// `@badge-info=subscriber/8`
    BadgeInfo(usize),
    Badges(Vec<Badge>),  // List of badges
    BanDuration(usize),  // Duration in seconds
    Color(String),       // Hex color. Ex. #B000B5
    DisplayName(String), // Display name of the chatter
    EmoteOnly(bool),     // True if emote only mode is on
    FollowersOnly(bool), // True if follower only mode is on
    Emotes(Vec<Emote>),
    EmoteSets(Vec<usize>), // List of emote sets
    Id(String),            // Id of the message
    Login(String),         // The login of the user whos message is being cleared
    Mod(bool),             // True if the user is a moderator
    RoomId(String),        // Id of the chat room
    Subscriber(bool),      // True if the user is a subscriber
    TargetMsgId(String),   // Id of the message the command is relating to
    TargetUserId(String),  //  Id of the user the command is relating to
    Turbo(bool),           // True if the user has Turbo
    TmiSentTs(String),
    UserId(String),     // Id of the user
    UserType(UserType), // Type of the user
    Unknown,
}

/// Represents a vector of strings that has been separated by whitespaces.
type Parameters = Vec<String>;

/// Represents a message parsed into an easy to work with struct.
#[derive(PartialEq, Debug)]
pub struct ParsedTwitchMessage {
    pub tags: HashMap<String, Tag>,
    pub source: Option<Source>,
    pub command: Command,
    pub parameters: Option<Parameters>,
    pub bot_command: Option<BotCommand>,
}

/// Parses a message from a Twitch IRC Chat
///
/// # Panics
///
/// When the message lacks a source.
///
/// When the message starts with a '@' but has nothing after it.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use twitch_irc_parser::{parse_message, Command, ParsedTwitchMessage};
///
/// let message = "PING :tmi.twitch.tv";
/// let parsed = parse_message(message);
///
/// let expected = ParsedTwitchMessage {
///     tags: HashMap::new(),
///     source: None,
///     command: Command::PING,
///     bot_command: None,
///     parameters: Some(vec![String::from("tmi.twitch.tv")]),
/// };
///
/// assert_eq!(parsed, expected);
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
            "admin" => Badge::Admin(badge_version),
            "bits" => Badge::Bits(badge_version),
            "broadcaster" => Badge::Broadcaster(badge_version),
            "moderator" => Badge::Moderator(badge_version),
            "staff" => Badge::Staff(badge_version),
            "subscriber" => Badge::Subscriber(badge_version),
            "turbo" => Badge::Turbo(badge_version),
            "vip" => Badge::Vip(badge_version),
            _ => Badge::Unknown,
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
            "badge-info" => match tag_value {
                Some(value) => {
                    let mut split = value.split('/');
                    split.next();
                    let subscriber_length = split
                        .next()
                        .expect("Should have subscriber length value")
                        .parse::<usize>()
                        .expect("Should be a number");
                    Tag::BadgeInfo(subscriber_length)
                }
                None => Tag::BadgeInfo(0),
            },
            "badges" => match tag_value {
                Some(value) => {
                    let badges = parse_badges(value);
                    Tag::Badges(badges)
                }
                None => Tag::Badges(vec![]),
            },
            "ban-duration" => match tag_value {
                Some(value) => {
                    let duration = value.parse::<usize>().expect("Should have a duration");
                    Tag::BanDuration(duration)
                }
                None => {
                    eprintln!("Should have a ban-duration");
                    Tag::BanDuration(0)
                }
            },
            "color" => match tag_value {
                Some(value) => Tag::Color(value.to_string()),
                None => {
                    eprintln!("Should have a color");
                    Tag::Color(String::from(""))
                }
            },
            "display-name" => match tag_value {
                Some(value) => Tag::DisplayName(value.to_string()),
                None => {
                    eprintln!("Should have a display-name");
                    Tag::DisplayName(String::from(""))
                }
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
                None => {
                    eprintln!("Should have an id");
                    Tag::Id(String::from("0"))
                }
            },
            "login" => match tag_value {
                Some(value) => Tag::Login(value.to_string()),
                None => {
                    eprintln!("Should have a login");
                    Tag::Login(String::from(""))
                }
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
                None => {
                    eprintln!("Should have a room-id");
                    Tag::RoomId(String::from("0"))
                }
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
            "target-msg-id" => match tag_value {
                Some(value) => Tag::TargetMsgId(value.to_string()),
                None => {
                    eprintln!("Should have a target-msg-id");
                    Tag::TargetMsgId(String::from(""))
                }
            },
            "target-user-id" => match tag_value {
                Some(value) => Tag::TargetUserId(value.to_string()),
                None => {
                    eprintln!("Should have a target-user-id");
                    Tag::TargetUserId(String::from(""))
                }
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
                None => {
                    eprintln!("Should have a tmi-sent-ts");
                    Tag::TmiSentTs(String::from("0"))
                }
            },
            "user-id" => match tag_value {
                Some(value) => Tag::UserId(value.to_string()),
                None => {
                    eprintln!("Should have a user-id");
                    Tag::UserId(String::from("0"))
                }
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
                None => {
                    eprintln!("Should have a user-type");
                    Tag::UserType(UserType::Normal)
                }
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
    parameters
}

fn parse_command(raw_command: &str) -> Command {
    let mut command_parts = raw_command.split(' ');

    let command = command_parts.next().expect("This should be the command");

    match command {
        "CLEARCHAT" => {
            let channel = command_parts.next().expect("This should exist");
            Command::CLEARCHAT(channel.to_string())
        }
        "CLEARMSG" => {
            let channel = command_parts.next().expect("This should exist");
            Command::CLEARMSG(channel.to_string())
        }
        "GLOBALUSERSTATE" => Command::GLOBALUSERSTATE,
        "HOSTTARGET" => {
            let channel = command_parts.next().expect("This should exist");
            Command::HOSTTARGET(channel.to_string())
        }
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
