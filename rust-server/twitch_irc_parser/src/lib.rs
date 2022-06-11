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
}

/// Representation of the message command.
#[derive(PartialEq, Debug)]
pub enum Command {
    /// This is the standard `JOIN` message that you receive when a user joins the chat room.
    ///
    /// #Prototype
    /// `:<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>`
    JOIN(String),

    /// This is the standard `PART` message that you receive when a user leaves a chat room.
    ///
    /// #Prototype
    /// `:<user>!<user>@<user>.tmi.twitch.tv PART #<channel>`
    PART(String),

    /// Sent to indicate the outcome of an action like banning a user.
    ///
    /// The Twitch IRC server sends this message when:
    ///
    /// * A moderator (or bot with moderator privileges) sends a message with pretty much any of the [chat commands](https://dev.twitch.tv/docs/irc/chat-commands). For example, /emoteonly, /subscribers, /ban or /host.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv NOTICE #<channel> :<message>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>channel</code></td>
    ///       <td>The channel (chat room) where the action occurred.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>message</code></td>
    ///       <td>A message that describes the outcome of the action.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    NOTICE {
        channel: String,
        message: String,
        tags: Option<HashMap<String, Tag>>,
    },

    /// Sent when a moderator (or bot with moderator privileges) removes all messages from the chat room or removes all messages for the specified user.
    ///
    /// The Twitch IRC server sends this message when:
    /// * A moderator enters the /clear command in the chat room or a bot sends the /clear [chat command](https://dev.twitch.tv/docs/irc/chat-commands) message .
    /// * A moderator enters the /ban or /timeout command in the chat room or a bot sends the /ban or /timeout chat command message.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv CLEARCHAT #<channel> :<user>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>channel</code></td>
    ///       <td>The name of the channel (chat room) where the messages were removed from.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>user</code></td>
    ///       <td>Optional: The login name of the user whose messages were removed from the chat room because they were banned or put in a timeout.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    CLEARCHAT {
        channel: String,
        user: Option<String>,
        tags: Option<HashMap<String, Tag>>,
    },

    /// Sent when a bot with moderator privileges deletes a single message from the chat room.
    ///
    /// The Twitch IRC server sends this message when:
    ///
    /// * The bot sends a /delete chat command message.
    /// # Prototype
    /// `:tmi.twitch.tv CLEARMSG #<channel> :<message>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>channel</code></td>
    ///       <td>The name of the channel (chat room) where the message were removed from.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>message</code></td>
    ///       <td>The chat message that was removed.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    CLEARMSG {
        channel: String,
        message: String,
        tags: Option<HashMap<String, Tag>>,
    },

    /// Sent when a channel starts or stops hosting viewers from another channel.
    ///
    /// The Twitch IRC server sends this message when:
    ///
    /// A moderator enters the /host or /unhost command in the chat room or a bot with moderator privileges sends a /host or /unhost chat command message.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv HOSTTARGET #<hosting-channel> :[-|<channel>] <number-of-viewers>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>-</code></td>
    ///       <td>The channel is no longer hosting viewers.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>channel</code></td>
    ///       <td>The channel being hosted.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>hosting-channel</code></td>
    ///       <td>The channel that's hosting the viewers.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>number-of-viewers</code></td>
    ///       <td>The number of viewers from <channel> that are watching the broadcast.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    HOSTTARGET {
        channel: String,
        hosting_channel: String,
        number_of_viewers: usize,
    },

    /// A PRIVMSG type message. The `String` is the channel name.
    PRIVMSG {
        channel: String,
        message: String,
        bot_command: Option<BotCommand>,
        tags: Option<HashMap<String, Tag>>,
    },

    /// A PING type message.
    PING,
    // TODO: What is the `bool`?
    /// A CAP type message.
    CAP(bool),

    /// Sent after the bot successfully authenticates (by sending the PASS/NICK commands) with the server.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv GLOBALUSERSTATE`
    GLOBALUSERSTATE { tags: Option<HashMap<String, Tag>> },

    /// Sent when events like someone subscribing to the channel occurs.
    ///
    /// The Twitch IRC server sends this message when:
    /// * A user subscribes to the channel, re-subscribes to the channel, or gifts a subscription to another user.
    /// * Another broadcaster raids the channel. Raid is a Twitch feature that lets broadcasters send their viewers to another channel to help support and grow other members in the community. [Learn more](https://help.twitch.tv/s/article/how-to-use-raids)
    /// * A viewer milestone is celebrated such as a new viewer chatting for the first time.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv USERNOTICE #<channel> :[<message>]`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>channel</code></td>
    ///       <td>The name of the channel that the event occurred in.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>message</code></td>
    ///       <td>Optional. The chat message that describes the event.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    USERNOTICE {
        channel: String,
        message: Option<String>,
        tags: Option<HashMap<String, Tag>>,
    },

    /// Sent when the bot joins a channel or sends a `PRIVMSG` message.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv USERSTATE #<channel>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>channel</code></td>
    ///       <td>The name of the channel that the bot joined or sent a PRIVMSG in.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    USERSTATE {
        channel: String,
        tags: Option<HashMap<String, Tag>>,
    },

    /// Sent when the bot joins a channel or when the channel’s chat settings change.
    ///
    /// The Twitch IRC server sends this message when:
    ///
    /// * The bot joins a channel
    /// * A moderator (or bot with moderator privileges) sends a message with one of the following [chat command](https://dev.twitch.tv/docs/irc/chat-commands) messages:
    ///     * /emoteonly(off)
    ///     * /followers(off)
    ///     * /slow(off)
    ///     * /subscribers(off)
    ///     * /uniquechat(off)
    ///
    /// # Prototype
    /// `:tmi.twitch.tv ROOMSTATE #<channel>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>channel</code></td>
    ///       <td>The name of the channel (chat room) that the room state information applies to.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    ROOMSTATE {
        channel: String,
        tags: Option<HashMap<String, Tag>>,
    },

    /// Sent when the Twitch IRC server needs to terminate the connection for maintenance reasons. This gives your bot a chance to perform minimal clean up and save state before the server terminates the connection. The amount of time between receiving the message and the server closing the connection is indeterminate.
    ///
    /// The normal course of action is to reconnect to the Twitch IRC server and rejoin the channels you were previously joined to prior to the server terminating the connection.
    ///
    /// # Prototype
    /// `:tmi.twitch.tv RECONNECT`
    RECONNECT,

    ///Sent when a `WHISPER` message is directed specifically to your bot. Your bot will never receive whispers sent to other users.
    ///
    /// # Prototype
    /// `:<to-user>!<to-user>@<to-user>.tmi.twitch.tv WHISPER <from-user> :<message>`
    ///
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th><strong>Parameter</strong></th>
    ///       <th><strong>Description</strong></th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>from-user</code></td>
    ///       <td>The user that's sending the whisper message.</td>
    ///     </tr>
    ///     </tr>
    ///       <td><code>to-user</code></td>
    ///       <td>The user that's receiving the whisper message.</td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    // TODO: Needs to be implemented properly, and maybe separately.
    WHISPER {
        from_user: String,
        message: String,
        to_user: String,
    },

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

    /// Not sure what this one represents.
    /// TODO: Find out what this badge mean
    Premium(usize),

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
// TODO: Rewrite with focus on host. Nick is often not available.
#[derive(PartialEq, Debug)]
pub struct Source {
    pub nick: Option<String>,
    pub host: String,
}

/// Represents a bot command.
/// Bot commands are `PRIVMSG`s that begin with an exclamation point (`!`) directly followed by the command.
/// The parameters to the command will be a vector of the rest of the words in the message delimited by whitespaces.
#[derive(PartialEq, Debug)]
pub struct BotCommand {
    pub command: String,
    pub parameters: Vec<String>,
}

/// Represents the start and end index in a string.
#[derive(PartialEq, Debug)]
pub struct TextPosition {
    pub start_index: usize,
    pub end_index: usize,
}

/// Represents an emotes id and where in the message this emote is.
#[derive(PartialEq, Debug)]
pub struct Emote {
    pub id: usize,
    pub positions: Vec<TextPosition>,
}

/// Represents what type of user sent the message.
#[derive(PartialEq, Debug)]
pub enum UserType {
    /// A normal user.
    Normal, // ""

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
    Badges(Vec<Badge>), // List of badges
    BanDuration(usize), // Duration in seconds

    ClientNonce(String),

    /// The color of the user’s name in the chat room. This is a hexadecimal RGB color code in the form, #<RGB>. This tag may be empty if it is never set.
    Color(String),

    /// The user’s display name, escaped as described in the [IRCv3 spec](https://ircv3.net/specs/core/message-tags-3.2.html). This tag may be empty if it is never set.
    DisplayName(String),

    EmoteOnly(bool), // True if emote only mode is on

    FirstMsg(bool),

    /// An integer value that determines whether only followers can post messages in the chat room. The value indicates how long, in minutes, the user must have followed the broadcaster before posting chat messages. If the value is -1, the chat room is not restricted to followers only.
    FollowersOnly(i32),
    Emotes(Vec<Emote>),
    EmoteSets(Vec<usize>), // List of emote sets

    /// An ID that uniquely identifies this message.
    Id(String),

    /// The login name of the user whose action generated the message.
    Login(String),

    /// A Boolean value that determines whether the user is a moderator. Is true (1) if the user is a moderator; otherwise, false (0).
    Mod(bool),

    /// An ID that you can use to programmatically determine the action’s outcome. For a list of possible IDs, see [NOTICE Message IDs](https://dev.twitch.tv/docs/irc/msg-id)
    MsgId(String),

    /// Included only with `sub` and `resub` notices.
    ///
    /// The total number of months the user has subscribed. This is the same as `msg-param-months` but sent for different types of user notices.
    MsgParamCumulativeMonths(usize),

    /// Included only with `subgift` notices.
    ///
    /// The total number of months the user has subscribed. This is the same as `msg-param-cumulative-months` but sent for different types of user notices.
    MsgParamMonths(usize),

    /// Included only with `subgift` notices.
    ///
    /// The display name of the subscription gift recipient.
    MsgParamRecipientDisplayName(String),

    /// Included only with `subgift` notices.
    ///
    /// The user ID of the subscription gift recipient.
    MsgParamRecipientId(String),

    /// Included only with `subgift` notices.
    ///
    /// The user name of the subscription gift recipient.
    MsgParamRecipientName(String),

    /// Included only with `sub` and `resub` notices.
    ///
    /// The number of consecutive months the user has subscribed. This is zero (0) if `msg-param-should-share-streak` is 0.
    MsgParamStreakMonths(usize),

    /// Included only with `sub` and `resub` notices.
    ///
    /// A Boolean value that indicates whether the user wants their streaks shared.
    MsgParamShouldShareStreak(bool),

    /// Included only with `sub`, `resub` and `subgift` notices.
    ///
    /// The type of subscription plan being used. Possible values are:
    /// * Prime — Amazon Prime subscription
    /// * 1000 — First level of paid subscription
    /// * 2000 — Second level of paid subscription
    /// * 3000 — Third level of paid subscription
    MsgParamSubPlan(String),

    /// Included only with `sub`, `resub` and `subgift` notices.
    ///
    /// The display name of the subscription plan. This may be a default name or one created by the channel owner.
    MsgParamSubPlanName(String),

    /// A Boolean value that determines whether a user’s messages must be unique. Applies only to messages with more than 9 characters. Is true (1) if users must post unique messages; otherwise, false (0).
    R9K(bool),
    RoomId(String), // Id of the chat room
    /// An integer value that determines how long, in seconds, users must wait between sending messages.
    Slow(usize),
    Subscriber(bool), // True if the user is a subscriber
    /// A Boolean value that determines whether only subscribers and moderators can chat in the chat room. Is true (1) if only subscribers and moderators can chat; otherwise, false (0).
    SubsOnly(bool),
    SystemMsg(String),
    TargetMsgId(String),  // Id of the message the command is relating to
    TargetUserId(String), //  Id of the user the command is relating to
    Turbo(bool),          // True if the user has Turbo
    TmiSentTs(String),
    UserId(String),     // Id of the user
    UserType(UserType), // Type of the user
    Unknown,
}

/// Represents a vector of strings that has been separated by whitespaces.
pub type Parameters = Vec<String>;

/// Represents a message parsed into an easy to work with struct.
#[derive(PartialEq, Debug)]
pub struct ParsedTwitchMessage {
    pub source: Option<Source>,
    pub command: Command,
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

        idx = end_index + 1;
        &message[1..end_index]
    } else {
        ""
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

    // Get the command component
    // At this point it is just the rest of the message.

    let raw_command = &message[idx..];

    let command = parse_command(raw_command, tags);

    ParsedTwitchMessage { command, source }
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
            "premium" => Badge::Premium(badge_version),
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

    if raw_emotes.is_empty() {
        return emotes;
    }

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
    let mut tags: HashMap<String, Tag> = HashMap::new();

    if raw_tags.is_empty() {
        return tags;
    }

    let parsed_tags = raw_tags.split(';');

    for parsed_tag in parsed_tags {
        let mut split_tag = parsed_tag.split('=');

        let tag_key = split_tag.next().expect("Should contain at least a key");
        let tag_value = split_tag.next();

        let tag = match tag_key {
            "badge-info" => match tag_value {
                Some(value) => {
                    if value.is_empty() {
                        eprintln!("What to do when `badge-info` is empty? Returning Tag::BadgeInfo(0) for now.");
                        Tag::BadgeInfo(0)
                    } else {
                        let mut split = value.split('/');
                        split.next();
                        let subscriber_length = split
                            .next()
                            .expect("Should have subscriber length value")
                            .parse::<usize>()
                            .expect("Should be a number");
                        Tag::BadgeInfo(subscriber_length)
                    }
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
            "client-nonce" => match tag_value {
                Some(value) => Tag::ClientNonce(value.to_string()),
                None => {
                    eprintln!("Should have a client-nonce");
                    Tag::ClientNonce(String::from(""))
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
            "first-msg" => match tag_value {
                Some(value) => {
                    let fisrt_msg = match value {
                        "1" => true,
                        _ => false,
                    };
                    Tag::FirstMsg(fisrt_msg)
                }
                None => Tag::FirstMsg(false),
            },
            "followers-only" => match tag_value {
                Some(value) => {
                    let minutes = value.parse::<i32>().expect("Should be a number");
                    Tag::FollowersOnly(minutes)
                }
                None => {
                    eprintln!("Should have a time in minutes");
                    Tag::FollowersOnly(-1)
                }
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
            "msg-id" => match tag_value {
                Some(value) => Tag::MsgId(value.to_string()),
                None => {
                    eprintln!("Should have a NOTICE Message ID");
                    Tag::MsgId(String::from(""))
                }
            },
            "msg-param-cumulative-months" => match tag_value {
                Some(value) => {
                    let months = value.parse::<usize>().expect("Should have months");

                    Tag::MsgParamCumulativeMonths(months)
                }
                None => {
                    eprintln!("Should have a value");
                    Tag::MsgParamCumulativeMonths(0)
                }
            },
            "msg-param-months" => match tag_value {
                Some(value) => {
                    let months = value.parse::<usize>().expect("Should have months");

                    Tag::MsgParamMonths(months)
                }
                None => {
                    eprintln!("Should have a value");
                    Tag::MsgParamMonths(0)
                }
            },
            "msg-param-recipient-display-name" => match tag_value {
                Some(value) => Tag::MsgParamRecipientDisplayName(value.to_string()),
                None => {
                    eprintln!("Should have a value");
                    Tag::MsgParamRecipientDisplayName(String::from(""))
                }
            },
            "msg-param-recipient-id" => match tag_value {
                Some(value) => Tag::MsgParamRecipientId(value.to_string()),
                None => {
                    eprintln!("Should have a value");
                    Tag::MsgParamRecipientId(String::from(""))
                }
            },
            "msg-param-recipient-name" => match tag_value {
                Some(value) => Tag::MsgParamRecipientName(value.to_string()),
                None => {
                    eprintln!("Should have a value");
                    Tag::MsgParamRecipientName(String::from(""))
                }
            },
            "msg-param-streak-months" => match tag_value {
                Some(value) => {
                    let months = value.parse::<usize>().expect("Should have months");

                    Tag::MsgParamStreakMonths(months)
                }
                None => {
                    eprintln!("Should have a value");
                    Tag::MsgParamStreakMonths(0)
                }
            },
            "msg-param-should-share-streak" => match tag_value {
                Some(value) => {
                    let should_share = match value {
                        "1" => true,
                        "0" => false,
                        _ => {
                            eprintln!("Should be 1 or 0");
                            false
                        }
                    };
                    Tag::MsgParamShouldShareStreak(should_share)
                }
                None => Tag::MsgParamShouldShareStreak(false),
            },
            "msg-param-sub-plan" => match tag_value {
                Some(value) => Tag::MsgParamSubPlan(value.to_string()),
                None => {
                    eprintln!("Should have a msg-param-sub-plan");
                    Tag::MsgParamSubPlan(String::from(""))
                }
            },
            "msg-param-sub-plan-name" => match tag_value {
                Some(value) => Tag::MsgParamSubPlanName(value.to_string()),
                None => {
                    eprintln!("Should have a msg-param-sub-plan-name");
                    Tag::MsgParamSubPlanName(String::from(""))
                }
            },
            "r9k" => match tag_value {
                Some(value) => {
                    let r9k = match value {
                        "1" => true,
                        "0" => false,
                        _ => {
                            eprint!("Should only be 1 or 0");
                            false
                        }
                    };
                    Tag::R9K(r9k)
                }
                None => {
                    eprintln!("Should have a value");
                    Tag::R9K(false)
                }
            },
            "room-id" => match tag_value {
                Some(value) => Tag::RoomId(value.to_string()),
                None => {
                    eprintln!("Should have a room-id");
                    Tag::RoomId(String::from("0"))
                }
            },
            "slow" => match tag_value {
                Some(value) => {
                    let slow = value.parse::<usize>().expect("Should have a duration");

                    Tag::Slow(slow)
                }
                None => {
                    eprintln!("Should have a value");
                    Tag::Slow(0)
                }
            },
            "subs-only" => match tag_value {
                Some(value) => {
                    let subs_only = match value {
                        "1" => true,
                        "0" => false,
                        _ => {
                            eprint!("Should only be 1 or 0");
                            false
                        }
                    };
                    Tag::SubsOnly(subs_only)
                }
                None => {
                    eprintln!("Should have a value");
                    Tag::SubsOnly(false)
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
            "system-msg" => match tag_value {
                Some(value) => Tag::SystemMsg(value.to_string()),
                None => {
                    eprintln!("Should have a system-message");
                    Tag::SystemMsg(String::from(""))
                }
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

fn parse_command(raw_command: &str, raw_tags: &str) -> Command {
    let parameters_index = raw_command.find(':').unwrap_or(raw_command.len());
    let channel_index = raw_command.find('#').unwrap_or(parameters_index);

    let channel = if channel_index != parameters_index {
        let channel = raw_command[channel_index + 1..parameters_index].trim();
        channel
    } else {
        ""
    };

    let parameters = if parameters_index != raw_command.len() {
        let parameters = raw_command[parameters_index + 1..].trim();
        parameters
    } else {
        ""
    };

    let command = raw_command[..channel_index].trim();

    let tags = if raw_tags.is_empty() {
        None
    } else {
        Some(parse_tags(raw_tags))
    };

    match command {
        "CLEARCHAT" => Command::CLEARCHAT {
            channel: channel.to_string(),
            user: if parameters.is_empty() {
                None
            } else {
                Some(parameters.to_string())
            },
            tags,
        },
        "CLEARMSG" => Command::CLEARMSG {
            channel: channel.to_string(),
            message: parameters.to_string(),
            tags,
        },
        "GLOBALUSERSTATE" => Command::GLOBALUSERSTATE { tags },
        "HOSTTARGET" => {
            let mut params = parameters.split(' ');
            let hosted_channel = params.next().unwrap_or_else(|| {
                eprintln!("Should have a channel name or '-'");
                ""
            });
            let number_of_viewers = params
                .next()
                .unwrap_or_else(|| {
                    eprintln!("Should have amount of viewers");
                    "0"
                })
                .parse::<usize>()
                .unwrap_or_else(|_err| {
                    eprintln!("Number of viewers should be a `usize`");
                    0
                });
            Command::HOSTTARGET {
                hosting_channel: channel.to_string(),
                channel: hosted_channel.to_string(),
                number_of_viewers,
            }
        }
        "NOTICE" => Command::NOTICE {
            channel: channel.to_string(),
            message: parameters.to_string(),
            tags,
        },
        "PING" => Command::PING,
        "PRIVMSG" => {
            let bot_command = parse_bot_command(parameters);
            Command::PRIVMSG {
                channel: channel.to_string(),
                message: parameters.to_string(),
                bot_command,
                tags,
            }
        }
        "RECONNECT" => Command::RECONNECT,
        "ROOMSTATE" => Command::ROOMSTATE {
            channel: channel.to_string(),
            tags,
        },
        "USERNOTICE" => Command::USERNOTICE {
            channel: channel.to_string(),
            message: if parameters.is_empty() {
                None
            } else {
                Some(parameters.to_string())
            },
            tags,
        },
        "USERSTATE" => Command::USERSTATE {
            channel: channel.to_string(),
            tags,
        },
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
