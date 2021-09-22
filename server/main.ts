import { listenAndServe } from "https://deno.land/std@0.107.0/http/server.ts";
import { hmac } from "https://deno.land/x/hmac@v2.0.1/mod.ts";

const PORT = `:${Deno.env.get("PORT")}`;

console.info(`Server is up and running! Listening on port ${PORT}`);
await listenAndServe(PORT, async (request) => {
    const url = new URL(request.url);
    // const urlSearchParams = new URLSearchParams(url.search);

    const body = await json(request.body);
    console.log("body", JSON.parse(body ?? "null"));

    switch (url.pathname) {
        case "/": {
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
    <h1>Welcome to Joxtabot, fren!</h1>
    <h2>How are you doing today?</h2>
</body>
</html>
            `;
            return createResponse(html, {
                headers: new Headers({
                    "Content-Type": "text/html; charset=UTF-8",
                }),
            });
        }
        case "/herp": {
            return createResponse({ herp: "derp", hurr: "durr" });
        }
        case "/hello": {
            return createResponse("Hello fren!");
        }
        case "/twitch/webhooks/callback": {
            const body = await json(request.body);

            let twitchVerified;
            try {
                twitchVerified = verifyTwitchSignature(request, body);
            } catch (error) {
                twitchVerified = false;
                console.error("Verification Failed horribly", error);
            }
            if (twitchVerified) {
                const messageType = request.headers.get(
                    "Twitch-Eventsub-Message-Type"
                );

                if (messageType === "webhook_callback_verification") {
                    console.log("[TWITCH] Verifying Webhook");
                    return createResponse(JSON.parse(body ?? "null").challenge);
                }

                const { event, subscription } = JSON.parse(body ?? "null");
                console.log(
                    `Receiving ${subscription.type} request for ${event.broadcaster_user_name}:`,
                    event
                );

                switch (subscription.type) {
                    case "stream.online": {
                        console.info(
                            "Joxtacy went live! Send online notification to Discord."
                        );
                        break;
                    }
                    case "channel.channel_points_custom_reward_redemption.add": {
                        const channelPointsRedemptionAdd = event;
                        console.info(
                            `Channel points redemption. Reward: ${channelPointsRedemptionAdd.reward.title}`
                        );

                        switch (channelPointsRedemptionAdd.reward.title) {
                            case "+1 Pushup": {
                                // Increment database pushup number
                                console.log("+1 Pushup");
                                break;
                            }
                            case "+1 Situp": {
                                // Increment database situp number
                                console.log("+1 Situp");
                                break;
                            }
                            case "Nice": {
                                console.log(`Nice, ${event.user_name}. 😏`);
                                break;
                            }
                            default: {
                                console.warn(
                                    `Unsupported custom reward: ${channelPointsRedemptionAdd.reward.title}`
                                );
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
            }

            return createResponse("This should be a proper response\n");
        }
        default: {
            return createResponse("This is not acceptable!", { status: 404 });
        }
    }
});

async function json(
    readableStream: ReadableStream<Uint8Array> | null
): Promise<string | null> {
    if (!readableStream) {
        return null;
    }
    const reader = readableStream.getReader();
    const newReadableStream = new ReadableStream({
        start(controller) {
            (function pump(): unknown {
                return reader?.read().then(({ done, value }) => {
                    if (done) {
                        controller.close();
                        return;
                    }
                    controller.enqueue(value);
                    return pump();
                });
            })();
        },
    });

    const response = new Response(newReadableStream);
    const blob = await response.blob();
    const arrBuf = await blob.arrayBuffer();

    return new TextDecoder().decode(arrBuf);
}

function createResponse(
    data: string | Record<string, unknown>,
    init?: ResponseInit
): Response {
    if (typeof data === "string") {
        const dataWithNewline = data.endsWith("\n") ? data : data + "\n";
        return new Response(new TextEncoder().encode(dataWithNewline), init);
    }
    return new Response(new TextEncoder().encode(JSON.stringify(data)), init);
}

function verifyTwitchSignature(request: Request, body: string | null) {
    const messageId = request.headers.get("Twitch-Eventsub-Message-Id") || "";
    const timestamp =
        request.headers.get("Twitch-Eventsub-Message-Timestamp") || "";
    const messageSignature = request.headers.get(
        "Twitch-Eventsub-Message-Signature"
    );

    const time = Math.floor(Date.now() / 1000);

    if (Math.abs(time - Number(timestamp)) > 600) {
        console.warn(
            `Twitch Verification Failed: tiemstamp > 10 minutes. MessageId: ${messageId}`
        );
        throw new Error("Twitch Verification Failed: Timestamp too old");
    }

    const twitchSigningSecret = Deno.env.get("TWITCH_SIGNING_SECRET");

    if (!twitchSigningSecret) {
        console.warn("Twitch Signing Secret is empty");
        throw new Error("Twitch Signing Secret is not set");
    }

    const computedSignature = `sha256=${hmac(
        "sha256",
        twitchSigningSecret,
        messageId + timestamp + body,
        "utf8",
        "hex"
    )}`;

    if (messageSignature !== computedSignature) {
        return false;
    }
    console.log("Twitch Verification Successful");
    return true;
}
