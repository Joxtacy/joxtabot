function noop() {}

interface TwitchBot {
    open: () => void;
}

interface TwitchBotOptions {
    channel: string;
    nick: string;
    url: string;
}
export default function TwitchBot(opts: TwitchBotOptions) {
    const $: TwitchBot = { open: () => {} };
    let ws;

    $.open = function () {
        ws = new WebSocket(opts.url);
    };
}
