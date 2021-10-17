import { assertEquals } from "https://deno.land/std@0.109.0/testing/asserts.ts";
import { TwitchIrcMessageType, TwitchPrivmsgIrcTags } from "../twitch/types.ts";
import { parseTwitchIrcMessage } from "../parser-utils.ts";

Deno.test("parseTwitchIrcMessage", async (t) => {
    await t.step("Parse init message", () => {
        const message = ":tmi.twitch.tv 001 joxtabot :Welcome, GLHF!\r\n" +
            ":tmi.twitch.tv 002 joxtabot :Your host is tmi.twitch.tv\r\n" +
            ":tmi.twitch.tv 003 joxtabot :This server is rather new\r\n" +
            ":tmi.twitch.tv 004 joxtabot :-\r\n" +
            ":tmi.twitch.tv 375 joxtabot :-\r\n" +
            ":tmi.twitch.tv 372 joxtabot :You are in a maze of twisty passages, all alike.\r\n" +
            ":tmi.twitch.tv 376 joxtabot :>\r\n";
        const actual = parseTwitchIrcMessage(message);

        const expected = {
            type: TwitchIrcMessageType.INIT,
            message: null,
            metadata: {
                tags: null,
                username: null,
                channel: null,
            },
        };

        assertEquals(actual, expected);
    });

    await t.step("Parse NAMES message", () => {
        const message =
            ":joxtabot.tmi.twitch.tv 353 joxtabot = #joxtacy :joxtabot\r\n" +
            ":joxtabot.tmi.twitch.tv 366 joxtabot #joxtacy :End of /NAMES list\r\n";

        const actual = parseTwitchIrcMessage(message);

        const expected = {
            type: TwitchIrcMessageType.NAMES,
            message: null,
            metadata: {
                tags: null,
                username: null,
                channel: null,
            },
        };

        assertEquals(actual, expected);
    });

    await t.step("Parse PING message", () => {
        const message = "PING :tmi.twitch.tv\r\n";
        const actual = parseTwitchIrcMessage(message);

        const expected = {
            type: TwitchIrcMessageType.PING,
            message: null,
            metadata: {
                username: null,
                channel: null,
                tags: new Map(),
            },
        };

        assertEquals(actual, expected);
    });

    await t.step("Parse JOIN message", () => {
        const message =
            "joxtabot!joxtabot@joxtabot.tmi.twitch.tv JOIN #joxtacy\r\n";

        const actual = parseTwitchIrcMessage(message);

        const expected = {
            type: TwitchIrcMessageType.JOIN,
            message: null,
            metadata: {
                username: "joxtabot",
                channel: "joxtacy",
                tags: new Map(),
            },
        };

        assertEquals(actual, expected);
    });

    await t.step("Parse USERSTATE message", () => {
        const message =
            "@badge-info=;badges=moderator/1;color=;display-name=Joxtabot;emote-sets=0;mod=1;subscriber=0;user-type=mod :tmi.twitch.tv USERSTATE #joxtacy\r\n";

        const actual = parseTwitchIrcMessage(message);

        const tags = new Map<string, string[]>();
        tags.set(TwitchPrivmsgIrcTags.BADGE_INFO, [""]);
        tags.set(TwitchPrivmsgIrcTags.BADGES, ["moderator/1"]);
        tags.set(TwitchPrivmsgIrcTags.COLOR, [""]);
        tags.set(TwitchPrivmsgIrcTags.DISPLAY_NAME, ["Joxtabot"]);
        tags.set(TwitchPrivmsgIrcTags.EMOTE_SETS, ["0"]);
        tags.set(TwitchPrivmsgIrcTags.MOD, ["1"]);
        tags.set(TwitchPrivmsgIrcTags.SUBSCRIBER, ["0"]);
        tags.set(TwitchPrivmsgIrcTags.USER_TYPE, ["mod"]);

        const expected = {
            type: TwitchIrcMessageType.USERSTATE,
            message: null,
            metadata: {
                username: null,
                channel: "joxtacy",
                tags,
            },
        };
        assertEquals(actual, expected);
    });

    await t.step("Parse PRIVMSG message", () => {
        const message =
            "@badge-info=subscriber/4;badges=broadcaster/1,subscriber/3003,sub-gifter/5;client-nonce=6f576c3e52fd3f33f7668c9d9f9175f8;color=#FF0079;display-name=Joxtacy;emotes=;flags=;id=84c4a516-0fdd-4995-b733-055c7cf56cec;mod=0;room-id=54605357;subscriber=1;tmi-sent-ts=1633146684996;turbo=0;user-id=54605357;user-type= :joxtacy!joxtacy@joxtacy.tmi.twitch.tv PRIVMSG #joxtacy :Hej hej!\r\n";

        const actual = parseTwitchIrcMessage(message);

        const tags = new Map<string, string[]>();
        tags.set(TwitchPrivmsgIrcTags.BADGE_INFO, ["subscriber/4"]);
        tags.set(TwitchPrivmsgIrcTags.BADGES, [
            "broadcaster/1",
            "subscriber/3003",
            "sub-gifter/5",
        ]);
        tags.set(TwitchPrivmsgIrcTags.CLIENT_NONCE, [
            "6f576c3e52fd3f33f7668c9d9f9175f8",
        ]);
        tags.set(TwitchPrivmsgIrcTags.COLOR, ["#FF0079"]);
        tags.set(TwitchPrivmsgIrcTags.DISPLAY_NAME, ["Joxtacy"]);
        tags.set(TwitchPrivmsgIrcTags.EMOTES, [""]);
        tags.set(TwitchPrivmsgIrcTags.FLAGS, [""]);
        tags.set(TwitchPrivmsgIrcTags.ID, [
            "84c4a516-0fdd-4995-b733-055c7cf56cec",
        ]);
        tags.set(TwitchPrivmsgIrcTags.MOD, ["0"]);
        tags.set(TwitchPrivmsgIrcTags.ROOM_ID, ["54605357"]);
        tags.set(TwitchPrivmsgIrcTags.SUBSCRIBER, ["1"]);
        tags.set(TwitchPrivmsgIrcTags.TMI_SENT_TS, ["1633146684996"]);
        tags.set(TwitchPrivmsgIrcTags.TURBO, ["0"]);
        tags.set(TwitchPrivmsgIrcTags.USER_ID, ["54605357"]);
        tags.set(TwitchPrivmsgIrcTags.USER_TYPE, [""]);

        const expected = {
            type: TwitchIrcMessageType.PRIVMSG,
            message: "Hej hej!",
            metadata: {
                username: "joxtacy",
                channel: "joxtacy",
                tags,
            },
        };

        assertEquals(actual, expected);
    });
});
