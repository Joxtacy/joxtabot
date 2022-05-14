import { TwitchIrcMessageType } from "./types.ts";
import { parseMessage } from "./irc-message-parser.ts";

class TwitchBot {
  private socket: WebSocket;
  private ready: Promise<void>;

  constructor(
    private channel = "joxtacy",
    private nick = "joxtabot",
    private url = "wss://irc-ws.chat.twitch.tv:443",
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

    this.socket.addEventListener("message", this.handlePing);

    this.socket.addEventListener("message", ({ data }) => {
      const parsedMessage = parseMessage(data);

      if (Deno.env.get("JOXTABOT_DEBUG") !== "") {
        console.group("[TWITCHBOT] Parsing message");
        console.log("===================");
        console.log("Original Message:");
        console.log(data);
        console.log("===================");
        console.log("Parsed Message:");
        console.log(parsedMessage);
        console.log("===================");
        console.groupEnd();
      }

      if (parsedMessage?.command?.command === TwitchIrcMessageType.PRIVMSG) {
        const message = parsedMessage.parameters;
        if (message?.includes("widepeepoHappy")) {
          this.sendPrivMsg("widepeepoHappy");
        } else if (message?.includes("catJAM")) {
          this.sendPrivMsg("catJAM");
        }
      }
    });
  }

  private handlePing = (event: MessageEvent) => {
    if (event.data.includes("PING :tmi.twitch.tv")) {
      console.log("[TWITCHBOT] Sending PONG");
      this.socket.send("PONG :tmi.twitch.tv");
    }
  };

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

  async timeout(userName: string, seconds = 180, reason = "") {
    await this.sendPrivMsg(`/timeout ${userName} ${seconds} ${reason}`);
  }

  async emoteOnly(seconds = 120) {
    await this.sendPrivMsg("/emoteonly");
    setTimeout(async () => {
      await this.sendPrivMsg("/emoteonlyoff");
    }, seconds * 1000);
  }

  addMessageListener(listener: (event: MessageEvent<unknown>) => unknown) {
    this.socket.addEventListener("message", listener);
  }
}

export default TwitchBot;
