/// Representation of the message command.
use std::collections::HashMap;

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
