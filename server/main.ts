import { green, yellow } from "https://deno.land/std@0.108.0/fmt/colors.ts";
import { Application, Router } from "https://deno.land/x/oak@v9.0.1/mod.ts";
import { errorHandler, logger, notFound, timing } from "./middlewares.ts";
import { verifySignature } from "./twitch/utils.ts";
import { writeFirst } from "./obs-utils.ts";
import TwitchBot from "./twitch/bot.ts";

const twitchBot = new TwitchBot();
twitchBot.sendPrivMsg("I am online, peeps! widepeepoHappy");

const PORT = Deno.env.get("PORT") || "8000";

const app = new Application();
const router = new Router();

app.use(logger);
app.use(timing);
app.use(errorHandler);

router.get("/", (context) => {
    context.response.type = "text/html";
    context.response.body = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Hello Oak Server</h1>
</body>
</html>
    `;
});

router.post("/twitch/webhooks/callback", async ({ request, response }) => {
    const body = await request.body().value; // as Twitch webhook something interface
    const verification = verifySignature(request.headers, JSON.stringify(body));

    if (verification) {
        const messageType = request.headers.get("Twitch-Eventsub-Message-Type");
        if (messageType === "webhook_callback_verification") {
            response.status = 200;
            response.body = body.challenge;
            return;
        }

        const { event, subscription } = body; // as TwitchEventsubMessage

        console.info(
            `Receiving ${subscription.type} request for ${event.broadcaster_user_name}:`,
            event,
        );

        switch (subscription.type) {
            case "stream.online": {
                console.info("[TWITCH] Stream is live!");
                writeFirst(""); // Reset 'First' when stream goes live
                break;
            }
            case "channel.channel_points_custom_reward_redemption.add": {
                const rewardTitle = event.reward?.title;

                switch (rewardTitle) {
                    case "First": {
                        console.log("[TWITCH] Write to file. First");
                        writeFirst(event.user_name);
                        break;
                    }
                    case "+1 Pushup": {
                        // Update file with amount of pushups
                        break;
                    }
                    case "+1 Situp": {
                        // Update file with amount of situps
                        break;
                    }
                    default: {
                        console.warn(
                            `[TWITCH] Reward not supported - ${rewardTitle}`,
                        );
                    }
                }
                break;
            }
            default: {
                console.warn(
                    `[TWITCH] Unknown subscription type - ${subscription.type}`,
                );
            }
        }

        // Default to respond with 200/OK
        response.status = 200;
        response.body = "OK";
        return;
    } else {
        // Twitch verification failed
        response.status = 500;
        response.body = "Verification failed";
        return;
    }
});

app.use(router.routes());
app.use(router.allowedMethods());
// Handling of 404 will kick in if no routes were matched, so this one must be placed last.
app.use(notFound);

app.addEventListener("listen", ({ secure, hostname, port }) => {
    const protocol = secure ? "https://" : "http://";
    const url = `${protocol}${hostname ?? "localhost"}:${port}`;
    console.log(`${yellow("Listening on:")} ${green(url)}`);
});

await app.listen({ port: Number(PORT) });
