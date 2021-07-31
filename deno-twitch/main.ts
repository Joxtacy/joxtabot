import { readAll } from "https://deno.land/std@0.101.0/io/mod.ts";
import "https://deno.land/x/dotenv@v2.0.0/load.ts";
import { startBot } from "https://deno.land/x/discordeno@12.0.1/mod.ts";

startBot({
    token: Deno.env.get("DISCORD_BOT_TOKEN") || "",
    intents: ["Guilds", "GuildMessages"],
    eventHandlers: {
        ready: () => {
            console.log("[DISCORD] Connected");
        },
        messageCreate: (msg) => {
            console.log("[DISCORD] Message received", msg);
        },
    },
});

import Application from "./server.ts";

const prod = Deno.env.get("PROD");
// Adds the environment variables in .env to Deno.env
prod || config({ export: true, safe: true });

const port = Number(Deno.env.get("PORT"));

const app = new Application();

const sendOnlineNotfication = async (event: any) => {
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

    const response = await fetch("https://api.courier.com/send", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
            Authorization: `Bearer ${Deno.env.get("COURIER_PROD_API_KEY")}`,
        },
        body: JSON.stringify({
            event: "TWITCH_ONLINE",
            recipient: "CHANNEL_GENERAL",
            profile: {
                discord: {
                    channel_id: Deno.env.get("DISCORD_CHANNEL_ID"),
                },
            },
            data: {
                stream_title: title,
                stream_game: gameName,
            },
        }),
    });
    const { messageId } = await response.json();
    console.log(
        `Online notification for ${event.broadcaster_user_name} sent. Message ID: ${messageId}.`
    );
};

app.use(async (req) => {
    const url = new URL(req.url, "http://localhost:3003");
    console.info(`Request: ${req.method} ${req.url} ${url.searchParams}`);
});

app.get("/", (req) => {
    req.respond({ status: 200, body: "Hello World!" });
});

app.get("/hello", (req) => {
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
                sendOnlineNotfication(event);
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
