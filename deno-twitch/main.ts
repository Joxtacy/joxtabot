import { readAll } from "https://deno.land/std@0.106.0/io/mod.ts";
import { v4 } from "https://deno.land/std@0.106.0/uuid/mod.ts";
import {
    // DiscordenoMessage,
    sendMessage,
    // startBot,
    // ws,
} from "https://deno.land/x/discordeno@12.0.1/mod.ts";

const joxtacyIsLiveChannelId = BigInt(Deno.env.get("DISCORD_CHANNEL_ID") || 0);

/* Let's remove the online Discord bot for now. Can't get the websocket to stay open.
const joxtabotDiscordChannelId = BigInt(
    Deno.env.get("DISCORD_JOXTABOT_CHANNELID") || 0
);
const sessionUuid = crypto.randomUUID();
const joxtabotUpdatedMessage = `**Joxtabot updated**:${
    Deno.env.get("DENO_DEPLOYMENT_ID") || "localhost"
}, sessionId:${sessionUuid}`;

const closeDiscordBot = () => {
    ws.shards.forEach((shard) => {
        clearInterval(shard.heartbeat.intervalId);
        if (
            shard.ws.readyState === WebSocket.OPEN ||
            shard.ws.readyState === WebSocket.CONNECTING
        ) {
            ws.closeWS(shard.ws, 3061, "Cleaning up old connections");
            console.log(`[DISCORD] Disconnected shard ${shard.id}`);
        }
    });
};

const startDiscordBot = () => {
    startBot({
        token: Deno.env.get("DISCORD_BOT_TOKEN") || "",
        intents: ["Guilds", "GuildMessages"],
        eventHandlers: {
            ready: () => {
                console.log("[DISCORD] Connected");
                sendMessage(joxtabotDiscordChannelId, joxtabotUpdatedMessage);
            },
            messageCreate: (msg) => {
                if (shouldCloseConnection(msg)) {
                    closeDiscordBot();
                }
                console.log("[DISCORD] Message received", msg);
                if (msg.content === "!pling") {
                    msg.reply("You rang.");
                    // msg.channel?.send("Plong!");
                } else if (msg.content === "!joxtabot") {
                    msg.reply(
                        "I am a bot created by Joxtacy. At your service. 🙇‍♂️"
                    );
                }
            },
        },
    });
};
startDiscordBot();

const shouldCloseConnection = (msg: DiscordenoMessage) => {
    // Could add a check to see that it is Joxtabot that sends the message by looking at msg.authorId
    const isBot = msg.isBot;
    const hasCorrectChannelId = msg.channelId === joxtabotDiscordChannelId;
    const isUpdatedMessage = msg.content.includes("**Joxtabot updated**");
    const [, sessionId] = msg.content.split("sessionId:");
    const hasValidUuid = v4.validate(sessionId);
    const hasSameSessionId = hasValidUuid && sessionUuid === sessionId;

    return (
        isBot && hasCorrectChannelId && isUpdatedMessage && !hasSameSessionId
    );
};
*/

import Application from "./server.ts";

const port = Number(Deno.env.get("PORT"));

const app = new Application();

const sendOnlineNotification = async (event: any) => {
    let title = "Le title";
    let gameName = "Le game";
    try {
        const yep = await fetch(
            `https://api.twitch.tv/helix/streams?user_id=${54605357}`,
            {
                headers: {
                    Authorization: `Bearer ${Deno.env.get("TWITCH_APP_TOKEN")}`,
                    "Client-Id": `${Deno.env.get("TWITCH_CLIENT_ID")}`,
                },
            }
        );
        const { data } = await yep.json();
        title = data[0].title;
        gameName = data[0].game_name;
        console.log("Twitch stream info", data);
    } catch (_error) {
        // noop
    }

    const messageContent = `
**Hi everyone! I am live!**
> Playing: ${gameName}
> Title: ${title}
https://twitch.tv/joxtacy
`;

    const discordMessage = await sendMessage(
        joxtacyIsLiveChannelId,
        messageContent
    );
    console.log(
        `Stream online notification for ${event.broadcaster_user_name} sent. Message ID: ${discordMessage.id}`
    );
};

/*
app.use(async (ctx, next) => {
    console.log(`[LOGGER] At the start of the request: ${ctx}`);
    await next();
    console.log(`[LOGGER] At the end of the request: ${ctx}`);
});
*/

app.get("/", (req) => {
    console.log("[LOGGER] Received request to /");
    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Joxtabot</title>
</head>
<body>
    <h1>Welcome to Joxtabot</h1>
</body>
</html>
`;
    const headers = new Headers({
        "Content-Type": "text/html; charset=UTF-8",
    });
    req.respond({
        status: 200,
        body: html,
        headers,
    });
});

app.get("/hello", (req) => {
    // startDiscordBot(); // Restart the Discord bot
    console.log("[LOGGER] Received request to /hello");
    req.respond({ status: 200, body: "Hello Deno!" });
});

app.post("/twitch/webhooks/callback", async (req) => {
    const rawBody = await readAll(req.body);
    const decodedBody = new TextDecoder().decode(rawBody);
    const body = JSON.parse(decodedBody);

    const messageType = req.headers.get("Twitch-Eventsub-Message-Type");

    if (app.verifyTwitchSignature(req, decodedBody)) {
        if (messageType === "webhook_callback_verification") {
            console.info("Verifying Webhook");
            req.respond({ status: 200, body: body.challenge });
            return;
        }
        const { event, subscription } = body;

        console.log(
            `Receiving ${subscription.type} request for ${event.broadcaster_user_name}:`,
            event
        );

        switch (subscription.type) {
            case "stream.online": {
                console.info("Joxtacy went live!");
                sendOnlineNotification(event);
                break;
            }
            case "channel.channel_points_custom_reward_redemption.add": {
                console.info("Channel points redemption", {
                    subscription,
                    event,
                });

                /*
                const {
                    id, // reward id
                    title, // reward title
                    prompt, // reward description
                    cost, // reward cost
                } = event.reward;
                const {
                    user_id, // user id
                    user_login, // user login (lowercase)
                    user_name, // user name (display name)
                    user_input, // input for the reward
                } = event;
                */
                break;
            }
            default: {
                console.warn(
                    "Unsupported subscription type",
                    subscription.type
                );
            }
        }

        req.respond({ status: 200, body: "OK" });
    } else {
        req.respond({ status: 403, body: "Invalid signature" });
    }
});

app.listen({ port });
console.info(`Listening on http://localhost:${port}`);
