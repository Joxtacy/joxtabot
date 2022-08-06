use super::*;

pub fn parse_badges(raw_badges: &str) -> Vec<Badge> {
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

pub fn parse_emotes(raw_emotes: &str) -> Vec<Emote> {
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

pub fn parse_emote_sets(raw_emote_sets: &str) -> Vec<usize> {
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

pub fn parse_tags(raw_tags: &str) -> HashMap<String, Tag> {
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
                    let emote_only = matches!(value, "1");
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
                    let first_msg = matches!(value, "1");
                    Tag::FirstMsg(first_msg)
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
                    let r#mod = matches!(value, "1");
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
                    let subscriber = matches!(value, "1");
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
                    let turbo = matches!(value, "1");
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

pub fn parse_source(raw_source: &str) -> Source {
    let mut split_source = raw_source.split('!');

    let first = split_source.next().expect("Should have at least one part");
    let second = split_source.next();

    if let Some(s) = second {
        Source {
            nick: Some(first.to_string()),
            host: s.to_string(),
        }
    } else {
        Source {
            nick: None,
            host: first.to_string(),
        }
    }
}

pub fn parse_command(raw_command: &str, raw_tags: &str) -> Command {
    let parameters_index = raw_command.find(':').unwrap_or(raw_command.len());
    let channel_index = raw_command.find('#').unwrap_or(parameters_index);

    let channel = if channel_index != parameters_index {
        raw_command[channel_index + 1..parameters_index].trim()
    } else {
        ""
    };

    let parameters = if parameters_index != raw_command.len() {
        raw_command[parameters_index + 1..].trim()
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
        "JOIN" => Command::JOIN(channel.to_string()),
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
        _ => {
            let mut split = command.split(' ');
            let number = split.next();
            match number {
                Some(number) => {
                    let number = number.trim().parse::<usize>();
                    match number {
                        Ok(number) => Command::NUMBER(number, Some(parameters.to_string())),
                        Err(_error) => Command::UNSUPPORTED,
                    }
                }
                None => Command::UNSUPPORTED,
            }
        }
    }
}

pub fn parse_bot_command(raw_bot_command: &str) -> Option<BotCommand> {
    // Example: "dilly dally wally"
    // Command: "dilly"
    // Parameters: ["dally", "wally"]
    let bot_command = raw_bot_command.trim();

    if bot_command.is_empty() {
        return None;
    }

    let first_char = &bot_command[..1];

    if first_char != "!" {
        // Not a bot command if not "!" first
        return None;
    }

    let bot_command = &bot_command[1..];

    if bot_command.is_empty() {
        // "!" is not a command
        return None;
    }

    if &bot_command[..1] == " " {
        // "! command" is not a command
        return None;
    }

    let have_params = bot_command.find(' ').is_some();

    if have_params {
        let mut params = bot_command.split(' ');
        let command = params.next().expect("There should be a str here").to_string();
        let parameters: Vec<String> = params.map(String::from).collect();
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
