import { assertEquals } from "https://deno.land/std@0.139.0/testing/asserts.ts";
import { parseMessage } from "../irc-message-parser.ts";

Deno.test("Original message is still the same", () => {
  const message = "PING :tmi.twitch.tv";
  const expected = "PING :tmi.twitch.tv";

  parseMessage(message);

  assertEquals(message, expected);
});

Deno.test("Parse PING Message", () => {
  const message = "PING :tmi.twitch.tv";

  const actual = parseMessage(message);

  const expected = {
    tags: null,
    source: null,
    command: {
      command: "PING",
    },
    parameters: "tmi.twitch.tv",
  };

  assertEquals(actual, expected);
});

Deno.test("Parse PRIVMSG Message", async (t) => {
  await t.step("Message WITH Tags", () => {
    const message =
      "@badges=staff/1,broadcaster/1,turbo/1;color=#FF0000;display-name=PetsgomOO;emote-only=1;emotes=33:0-7;flags=0-7:A.6/P.6,25-36:A.1/I.2;id=c285c9ed-8b1b-4702-ae1c-c64d76cc74ef;mod=0;room-id=81046256;subscriber=0;turbo=0;tmi-sent-ts=1550868292494;user-id=81046256;user-type=staff :petsgomoo!petsgomoo@petsgomoo.tmi.twitch.tv PRIVMSG #petsgomoo :DansGame";

    const actual = parseMessage(message);

    const expected = {
      tags: {
        badges: {
          staff: "1",
          broadcaster: "1",
          turbo: "1",
        },
        color: "#FF0000",
        "display-name": "PetsgomOO",
        "emote-only": "1",
        emotes: {
          33: [
            {
              startPosition: 0,
              endPosition: 7,
            },
          ],
        },
        id: "c285c9ed-8b1b-4702-ae1c-c64d76cc74ef",
        mod: "0",
        "room-id": "81046256",
        subscriber: "0",
        turbo: "0",
        "tmi-sent-ts": "1550868292494",
        "user-id": "81046256",
        "user-type": "staff",
      },
      source: {
        nick: "petsgomoo",
        host: "petsgomoo@petsgomoo.tmi.twitch.tv",
      },
      command: {
        command: "PRIVMSG",
        channel: "#petsgomoo",
      },
      parameters: "DansGame",
    };

    assertEquals(actual, expected);
  });

  await t.step("Message WITHOUT Tags", () => {
    const message =
      ":lovingt3s!lovingt3s@lovingt3s.tmi.twitch.tv PRIVMSG #lovingt3s :!dilly dally";

    const actual = parseMessage(message);

    const expected = {
      tags: null,
      source: {
        nick: "lovingt3s",
        host: "lovingt3s@lovingt3s.tmi.twitch.tv",
      },
      command: {
        command: "PRIVMSG",
        channel: "#lovingt3s",
        botCommand: "dilly",
        botCommandParams: "dally",
      },
      parameters: "!dilly dally",
    };

    assertEquals(actual, expected);
  });
});

Deno.test("Parse numeric message", () => {
  const message = ":joxtabot.tmi.twitch.tv 353 joxtabot = #joxtacy :joxtabot";

  const actual = parseMessage(message);

  const expected = null;

  assertEquals(actual, expected);
});
