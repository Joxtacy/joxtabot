// import axios from "https://cdn.skypack.dev/axios@v0.21.1";
import { readAll } from "https://deno.land/std@0.101.0/io/mod.ts";
import { config } from "https://deno.land/x/dotenv@v2.0.0/mod.ts";

import Application from "./server.ts";

// Adds the environment variables in .env to Deno.env
config({ export: true, safe: true });

const port = Number(Deno.env.get("PORT"));

const app = new Application();

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
            `Receiving ${subscription.type} request for ${event.broadcaster_user_name}: `,
            event
        );

        switch (subscription.type) {
            case "stream.online": {
                console.info("Joxtacy went live!");
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
