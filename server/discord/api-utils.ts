type Snowflake = BigInt;

interface UserObject {
    id: Snowflake;
}

interface MessageObject {
    id: Snowflake;
    channel_id: Snowflake;
    guild_id?: Snowflake;
    author: UserObject;
    content: string;
}

export const createMessage = async (
    channelId: BigInt,
    messageContent: string
): Promise<MessageObject> => {
    try {
        console.log("Sending message to Discord");
        const result = await fetch(
            `https://discord.com/api/v8/channels/${channelId}/messages`,
            {
                method: "POST",
                headers: {
                    Authorization: `Bot ${Deno.env.get("DISCORD_BOT_TOKEN")}`,
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    content: messageContent,
                }),
            }
        );
        const data = (await result.json()) as MessageObject;
        console.log("Message sent to Discord", data);
        return data;
    } catch (error) {
        console.error("Error sending message to Discord:", error);
        throw new Error("Error sending message to Discord");
    }
};
