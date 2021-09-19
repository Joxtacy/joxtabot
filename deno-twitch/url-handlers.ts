import { ServerRequest } from "https://deno.land/std@0.101.0/http/server.ts";
export const handleTwitchWebhooksCallback = async (req: ServerRequest) => {
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
            event,
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
                    subscription.type,
                );
            }
        }

        req.respond({ status: 200, body: "OK" });
    } else {
        req.respond({ status: 403, body: "Invalid signature" });
    }
};
