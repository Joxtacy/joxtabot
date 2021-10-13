import type { TwitchIrcMessage } from "./twitch/types.ts";
import { TwitchIrcMessageType } from "./twitch/types.ts";

const initRegexp = /\:tmi.twitch.tv \d{3}/;
const usernameRegexp = /([a-z0-9_]{3,})!([a-z0-9_]{3,})@([a-z0-9_]{3,})/; // match[1]
const channelRegexp = /\#([a-z0-9_]{3,})/; // match[1]
const tagsRegExp = /([a-z\-]*)\=([a-zA-Z0-9\_\-\/\,\#]*)(\;|\ )+/g;
const messageTypeRegexp = new RegExp(
    Object.values(TwitchIrcMessageType)
        .filter((k) => typeof k === "string")
        .join("|")
        .replaceAll("*", "\\*")
);
const messageRegexp = /:(.*) :(.*)\r\n$/; // match[2] // won't work on multiple messages

export const parseTwitchIrcMessage = (message: string): TwitchIrcMessage => {
    // const messages = message.split("\r\n").slice(0, -1); // might use this later
    const initMessage = initRegexp.exec(message);
    const parsedMessage = messageRegexp.exec(message);
    const username = usernameRegexp.exec(message);
    const channel = channelRegexp.exec(message);
    const msgTyps = messageTypeRegexp.exec(message) as string[];
    const tags = new Map<string, string[]>();
    let matches;
    while ((matches = tagsRegExp.exec(message)) !== null) {
        const hits = matches[0];
        const [tag, values] = hits.split("=");
        tags.set(
            tag,
            values.replaceAll(";", "").replaceAll(" ", "").split(",")
        );
    }

    if (initMessage?.length) {
        return {
            type: TwitchIrcMessageType.INIT,
            message: "",
            metadata: {
                tags: null,
                channel: null,
                username: null,
            },
        };
    }

    const messageType =
        TwitchIrcMessageType[msgTyps[0] as keyof typeof TwitchIrcMessageType];

    return {
        type: messageType,
        message: parsedMessage && parsedMessage[2],
        metadata: {
            username: username && username[1],
            channel: channel && channel[1],
            tags: tags,
        },
    };
};
