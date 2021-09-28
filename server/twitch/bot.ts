class TwitchBot {
    private socket: WebSocket;
    private ready: Promise<void>;

    constructor(
        private channel = "joxtacy",
        private nick = "joxtabot",
        private url = "wss://irc-ws.chat.twitch.tv:443"
    ) {
        this.socket = new WebSocket(url);
        this.ready = new Promise((resolve) => {
            this.socket.onopen = this.initTwitchConnection.bind(this, resolve);
        });
        this.socket.addEventListener("message", (event) => {
            console.group("[TWITCHBOT] Message Received");
            console.log(event.data);
            console.groupEnd();
        });

        this.socket.addEventListener("message", (event) => {
            if (event.data.includes("PING :tmi.twitch.tv")) {
                console.log("[TWITCHBOT] Sending PONG");
                this.socket.send("PONG :tmi.twitch.tv");
            }
        });
    }

    private initTwitchConnection(resolve: () => void, _event: Event) {
        resolve();
        this.socket.send(`PASS ${Deno.env.get("TWITCH_IRC_BOT_OAUTH")}`);
        this.socket.send(`NICK ${this.nick}`);
        this.socket.send(`JOIN #${this.channel}`);
        this.socket.send("CAP REQ :twitch.tv/membership");
        this.socket.send("CAP REQ :twitch.tv/tags twitch.tv/commands");
    }

    async sendPrivMsg(message: string) {
        await this.ready;
        this.socket.send(`PRIVMSG #${this.channel} :${message}`);
    }

    addMessageListener(listener: (event: MessageEvent<unknown>) => unknown) {
        this.socket.addEventListener("message", listener);
    }
}

export default TwitchBot;

/*
const ws = new WebSocket("wss://irc-ws.chat.twitch.tv:443");

ws.addEventListener("message", (message) => {
    console.log("[TWITCH WS] Received message:\n", message.data);
    parseTwitchIrcMessage(message.data);
    if (message.data.includes("PING :tmi.twitch.tv")) {
        ws.send("PONG :tmi.twitch.tv");
    } else if (message.data.includes("widepeepoHappy")) {
        ws.send("PRIVMSG #joxtacy :widepeepoHappy");
    }
    // Example ban ws.send("PRIVMSG #joxtacy :/timeout notjoxtacy 10 because why not?");
});

const parseTwitchIrcMessage = (message: string) => {
    const trimmedMessage = message.trim();

    const privmsgRegex =
        /\:[a-zA-Z_\d]*![a-zA-Z_\d]*@[a-zA-Z_\d]*\.tmi\.twitch\.tv\ PRIVMSG/;
    const pingRegex = /^PING :tmi.twitch.tv$/;

    if (privmsgRegex.test(trimmedMessage)) {
        console.info("[TWITCH WS] Found PRIVMSG");
        // private message. Example:
        // :<user>!<user>@<user>.tmi.twitch.tv PRIVMSG #<channel> :This is a sample message
        // :annishark!annishark@annishark.tmi.twitch.tv PRIVMSG #joxtacy :what are you counting
    } else if (pingRegex.test(trimmedMessage)) {
        console.info("[TWITCH WS] Found PING");
        // PING from twitch. Example
        // PING :tmi.twitch.tv
    }
};
*/
