import { getStreamInfo } from "../twitch/api-utils.ts";
import { createMessage } from "./api-utils.ts";

const joxtacyIsLiveChannelId = BigInt(
    Deno.env.get("DISCORD_JOXTACY_IS_LIVE_CHANNELID") || 0
);

const testingJoxtabotChannelId = BigInt(
    Deno.env.get("DISCORD_TESTING_JOXTABOT_CHANNELID") || 0
);

export const sendOnlineNotification = async (
    event: Record<string, unknown>
) => {
    let title = "Some good title";
    let gameName = "Some cool game";
    try {
        const streamInfo = await getStreamInfo(54605357);
        title = streamInfo.title;
        gameName = streamInfo.gameName;
    } catch (_error) {
        // noop
        console.warn("[TWITCH] Could not get stream info");
    }

    const messageContent = `
**Hi @everyone! I am live!**
> Playing: ${gameName}
> Title: ${title}
https://twitch.tv/joxtacy
`;

    console.log(
        `Sending online message to Discord. channelId: ${joxtacyIsLiveChannelId}, messageContent: ${messageContent}`
    );

    const discordMessage = await createMessage(
        joxtacyIsLiveChannelId,
        messageContent
    );

    console.log(
        `Stream online notification for ${event.broadcaster_user_name} sent. Message ID: ${discordMessage.id}`
    );
};
