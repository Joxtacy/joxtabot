import { sendMessage } from "./deps.ts";
import { TwitchEventsubEvent } from "./twitch-types.ts";

const joxtacyIsLiveChannelId = BigInt(Deno.env.get("DISCORD_CHANNEL_ID") || 0);

export const sendOnlineNotification = async (event: TwitchEventsubEvent) => {
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
