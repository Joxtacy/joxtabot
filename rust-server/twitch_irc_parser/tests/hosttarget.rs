use pretty_assertions::assert_eq;
use twitch_irc_parser::*;

#[test]
fn starting_host() {
    let message = ":tmi.twitch.tv HOSTTARGET #abc :xyz 10";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        command: Command::HOSTTARGET {
            channel: String::from("xyz"),
            hosting_channel: String::from("abc"),
            number_of_viewers: 10,
        },
        source: Some(Source {
            host: String::from("tmi.twitch.tv"),
            nick: None,
        }),
    };

    assert_eq!(actual, expected);
}

#[test]
fn ending_host() {
    let message = ":tmi.twitch.tv HOSTTARGET #abc :- 10";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        command: Command::HOSTTARGET {
            channel: String::from("-"),
            hosting_channel: String::from("abc"),
            number_of_viewers: 10,
        },
        source: Some(Source {
            host: String::from("tmi.twitch.tv"),
            nick: None,
        }),
    };

    assert_eq!(actual, expected);
}
