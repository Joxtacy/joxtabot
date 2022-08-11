//! # Twitch IRC Parser
//!
//! `twitch_irc_parser` contains the utilities needed to parse a single Twitch Message.
//! It is based on the parser example found on the Twitch Developer site.
//! <https://dev.twitch.tv/docs/irc/example-parser>

use std::collections::HashMap;

#[cfg(test)]
mod tests;

mod utils;

use utils::*;

mod types;

pub use types::*;

/// Parses a message from a Twitch IRC Chat
///
/// # Panics
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
///     source: None,
///     command: Command::PING,
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

/// Parses a string of messages from a Twitch IRC Chat
///
/// # Panics
///
/// When the string starts with a '@' but has nothing after it.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use twitch_irc_parser::{parse_messages, Command, ParsedTwitchMessage, Source};
///
/// let messages = "PING :tmi.twitch.tv\r\n:tmi.twitch.tv HOSTTARGET #abc :xyz 10"
///
/// let parsed = parse_messages(messages);
///
/// let expected = vec![
///     ParsedTwitchMessage {
///         source: None,
///         command: Command::PING,
///     },
///     ParsedTwitchMessage {
///         source: Some(Source {
///             host: String::from("tmi.twitch.tv"),
///             nick: None
///         }),
///         command: Command::HOSTTARGET {
///             channel: String::from("xyz"),
///             hosting_channel: String::from("abc"),
///             number_of_viewers: 10,
///         }
///     }
/// ];
///
/// assert_eq!(parsed, expected);
pub fn parse_messages(messages: &str) -> Vec<ParsedTwitchMessage> {
    let messages = messages.split("\r\n");

    messages.into_iter().map(parse_message).collect()
}
