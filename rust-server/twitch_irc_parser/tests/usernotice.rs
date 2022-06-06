use std::collections::HashMap;
use twitch_irc_parser::*;

#[test]
fn without_tags() {
    let message = ":tmi.twitch.tv USERNOTICE #dallas :Great stream -- keep it up!";

    let actual = parse_message(message);

    let expected = ParsedTwitchMessage {
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        command: Command::USERNOTICE {
            channel: String::from("dallas"),
            message: Some(String::from("Great stream -- keep it up!")),
            tags: None,
        },
    };

    assert_eq!(actual, expected);
}

// The following example shows a `USERNOTICE` message with tags after ronni resubscribed to the dallas channel.
#[test]
fn ronni_resubscribe() {
    let message = r"@badge-info=;badges=staff/1,broadcaster/1,turbo/1;color=#008000;display-name=ronni;emotes=;id=db25007f-7a18-43eb-9379-80131e44d633;login=ronni;mod=0;msg-id=resub;msg-param-cumulative-months=6;msg-param-streak-months=2;msg-param-should-share-streak=1;msg-param-sub-plan=Prime;msg-param-sub-plan-name=Prime;room-id=12345678;subscriber=1;system-msg=ronni\shas\ssubscribed\sfor\s6\smonths!;tmi-sent-ts=1507246572675;turbo=1;user-id=87654321;user-type=staff :tmi.twitch.tv USERNOTICE #dallas :Great stream -- keep it up!";

    let actual = parse_message(message);

    let mut expected_tags = HashMap::new();
    expected_tags.insert(String::from("badge-info"), Tag::BadgeInfo(0));
    expected_tags.insert(
        String::from("badges"),
        Tag::Badges(vec![
            Badge::Staff(1),
            Badge::Broadcaster(1),
            Badge::Turbo(1),
        ]),
    );
    expected_tags.insert(String::from("color"), Tag::Color(String::from("#008000")));
    expected_tags.insert(
        String::from("display-name"),
        Tag::DisplayName(String::from("ronni")),
    );
    expected_tags.insert(String::from("emotes"), Tag::Emotes(vec![]));
    expected_tags.insert(
        String::from("id"),
        Tag::Id(String::from("db25007f-7a18-43eb-9379-80131e44d633")),
    );
    expected_tags.insert(String::from("login"), Tag::Login(String::from("ronni")));
    expected_tags.insert(String::from("mod"), Tag::Mod(false));
    expected_tags.insert(String::from("msg-id"), Tag::MsgId(String::from("resub")));
    expected_tags.insert(
        String::from("msg-param-cumulative-months"),
        Tag::MsgParamCumulativeMonths(6),
    );
    expected_tags.insert(
        String::from("msg-param-streak-months"),
        Tag::MsgParamStreakMonths(2),
    );
    expected_tags.insert(
        String::from("msg-param-should-share-streak"),
        Tag::MsgParamShouldShareStreak(true),
    );
    expected_tags.insert(
        String::from("msg-param-sub-plan"),
        Tag::MsgParamSubPlan(String::from("Prime")),
    );
    expected_tags.insert(
        String::from("msg-param-sub-plan-name"),
        Tag::MsgParamSubPlanName(String::from("Prime")),
    );
    expected_tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("12345678")),
    );
    expected_tags.insert(String::from("subscriber"), Tag::Subscriber(true));
    expected_tags.insert(
        String::from("system-msg"),
        Tag::SystemMsg(String::from(r"ronni\shas\ssubscribed\sfor\s6\smonths!")),
    );
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1507246572675")),
    );
    expected_tags.insert(String::from("turbo"), Tag::Turbo(true));
    expected_tags.insert(
        String::from("user-id"),
        Tag::UserId(String::from("87654321")),
    );
    expected_tags.insert(String::from("user-type"), Tag::UserType(UserType::Staff));
    let expected = ParsedTwitchMessage {
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        command: Command::USERNOTICE {
            channel: String::from("forstycup"),
            message: Some(String::from("Great stream -- keep it up!")),
            tags: Some(expected_tags),
        },
    };

    assert_eq!(actual, expected);
}

// The following example shows a `USERNOTICE` message with tags after tww2 gifts a subscription to Mr_Woodchuck in forstycup’s channel.
#[test]
fn tww2_gift_sub_to_mr_woodchuck() {
    let message = r"@badge-info=;badges=staff/1,premium/1;color=#0000FF;display-name=TWW2;emotes=;id=e9176cd8-5e22-4684-ad40-ce53c2561c5e;login=tww2;mod=0;msg-id=subgift;msg-param-months=1;msg-param-recipient-display-name=Mr_Woodchuck;msg-param-recipient-id=55554444;msg-param-recipient-name=mr_woodchuck;msg-param-sub-plan-name=House\sof\sNyoro~n;msg-param-sub-plan=1000;room-id=12345678;subscriber=0;system-msg=TWW2\sgifted\sa\sTier\s1\ssub\sto\sMr_Woodchuck!;tmi-sent-ts=1521159445153;turbo=0;user-id=87654321;user-type=staff :tmi.twitch.tv USERNOTICE #forstycup";

    let actual = parse_message(message);

    let mut expected_tags = HashMap::new();
    expected_tags.insert(String::from("badge-info"), Tag::BadgeInfo(0));
    expected_tags.insert(
        String::from("badges"),
        Tag::Badges(vec![Badge::Staff(1), Badge::Premium(1)]),
    );
    expected_tags.insert(String::from("color"), Tag::Color(String::from("#0000FF")));
    expected_tags.insert(
        String::from("display-name"),
        Tag::DisplayName(String::from("TWW2")),
    );
    expected_tags.insert(String::from("emotes"), Tag::Emotes(vec![]));
    expected_tags.insert(
        String::from("id"),
        Tag::Id(String::from("e9176cd8-5e22-4684-ad40-ce53c2561c5e")),
    );
    expected_tags.insert(String::from("login"), Tag::Login(String::from("tww2")));
    expected_tags.insert(String::from("mod"), Tag::Mod(false));
    expected_tags.insert(String::from("msg-id"), Tag::MsgId(String::from("subgift")));
    expected_tags.insert(String::from("msg-param-months"), Tag::MsgParamMonths(1));
    expected_tags.insert(
        String::from("msg-param-recipient-display-name"),
        Tag::MsgParamRecipientDisplayName(String::from("Mr_Woodchuck")),
    );
    expected_tags.insert(
        String::from("msg-param-recipient-id"),
        Tag::MsgParamRecipientId(String::from("55554444")),
    );
    expected_tags.insert(
        String::from("msg-param-recipient-name"),
        Tag::MsgParamRecipientName(String::from("mr_woodchuck")),
    );
    expected_tags.insert(
        String::from("msg-param-sub-plan-name"),
        Tag::MsgParamSubPlanName(String::from(r"House\sof\sNyoro~n")),
    );
    expected_tags.insert(
        String::from("msg-param-sub-plan"),
        Tag::MsgParamSubPlan(String::from("1000")),
    );
    expected_tags.insert(
        String::from("room-id"),
        Tag::RoomId(String::from("12345678")),
    );
    expected_tags.insert(String::from("subscriber"), Tag::Subscriber(false));
    expected_tags.insert(
        String::from("system-msg"),
        Tag::SystemMsg(String::from(
            r"TWW2\sgifted\sa\sTier\s1\ssub\sto\sMr_Woodchuck!",
        )),
    );
    expected_tags.insert(
        String::from("tmi-sent-ts"),
        Tag::TmiSentTs(String::from("1521159445153")),
    );
    expected_tags.insert(String::from("turbo"), Tag::Turbo(false));
    expected_tags.insert(
        String::from("user-id"),
        Tag::UserId(String::from("87654321")),
    );
    expected_tags.insert(String::from("user-type"), Tag::UserType(UserType::Staff));
    let expected = ParsedTwitchMessage {
        source: Some(Source::new(None, String::from("tmi.twitch.tv"))),
        command: Command::USERNOTICE {
            channel: String::from("forstycup"),
            tags: Some(expected_tags),
            message: None,
        },
    };

    assert_eq!(actual, expected);
}
