import { readAll } from "./deps.ts";

import { sendOnlineNotification } from "./discord.ts";

import {
    ChannelPointsCustomRewardRedemptionAdd,
    ChannelPointCustomRewardTitle,
    TwitchEventsubNotification,
    TwitchEventsubSubscriptionType,
    TwitchEventsubEvent
} from "./twitch-types.ts";

import Application from "./server.ts";

const port = Number(Deno.env.get("PORT"));

const app = new Application();

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
        const { event, subscription } = body as TwitchEventsubNotification;

        console.log(
            `Receiving ${subscription.type} request for ${event.broadcaster_user_name}:`,
            event
        );

        switch (subscription.type) {
            case TwitchEventsubSubscriptionType.STREAM_ONLINE: {
                console.info("Joxtacy went live! Send online notification to Discord.");
                sendOnlineNotification(event as TwitchEventsubEvent);
                break;
            }
            case TwitchEventsubSubscriptionType.CHANNEL_POINT_REDEMPTION_ADD: {
                const channelPointsRedemptionAdd = event as ChannelPointsCustomRewardRedemptionAdd;
                console.info(`Channel points redemption. Reward: ${channelPointsRedemptionAdd.reward.title}`);

                switch (channelPointsRedemptionAdd.reward.title) {
                    case ChannelPointCustomRewardTitle.PUSHUP_PLUS_1: {
                        // Increment database pushup number
                        break;
                    }
                    case ChannelPointCustomRewardTitle.SITUP_PLUS_1: {
                        // Increment database situp number
                        break;
                    }
                    case ChannelPointCustomRewardTitle.NICE: {
                        console.log(`Nice, ${event.user_name}. ????`);
                        break;
                    }
                    default: {
                        console.warn(`Unsupported custom reward: ${channelPointsRedemptionAdd.reward.title}`);
                    }
                }
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
