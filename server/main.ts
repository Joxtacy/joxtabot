import { green, yellow } from "https://deno.land/std@0.108.0/fmt/colors.ts";
import { Application, Router } from "https://deno.land/x/oak@v9.0.1/mod.ts";
import { errorHandler, logger, notFound, timing } from "./middlewares.ts";
import { getRandomTimeoutReason } from "./twitch/timeout-utils.ts";
import { verifySignature } from "./twitch/utils.ts";
import { writeFirst } from "./obs-utils.ts";
import TwitchBot from "./twitch/bot.ts";
import { sendOnlineNotification } from "./discord/utils.ts";
import socketHandler from "./utils/socket-handler.ts";

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

router.get("/ws", async (context) => {
  if (context.isUpgradable) {
    const socket = await context.upgrade();

    socket.addEventListener("error", (error) => {
      console.error("[SOCKET SERVER] Someting went wrong:", error);
    });

    socket.addEventListener("open", (event) => {
      console.info(
        "[SOCKET SERVER] Connection opened:",
        new Date(event.timeStamp).toLocaleString(),
      );
    });

    socket.addEventListener("message", (message) => {
      console.info("[SOCKET SERVER] Message received:", message.data);
      socket.send(`You do be sending message, huh? ${message.data}`);
    });

    socket.addEventListener("close", () => {
      socketHandler.unregister(socket);
    });

    socketHandler.register(socket);
  }
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
        sendOnlineNotification(event);
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
          case "Timeout": {
            console.log(
              `[TWITCH] Timeout user: ${event.user_login}`,
            );
            twitchBot.timeout(
              event.user_login,
              180,
              getRandomTimeoutReason(),
            );
            break;
          }
          case "-420": {
            socketHandler.sendAll("420");
            break;
          }
          case "ded": {
            socketHandler.sendAll("Death");
            break;
          }
          case "Nice": {
            socketHandler.sendAll("Nice");
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
          case "Emote-only Chat": {
            // Start emote only chat. 2 min
            console.log("[TWITCH] Emote-only");
            twitchBot.emoteOnly(120);
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
