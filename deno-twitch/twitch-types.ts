export interface TwitchEventsubNotification {
    subscription: TwitchEventsubSubscription;
    event: TwitchEventsubEvent;
}

export interface TwitchEventsubSubscription {
    id: string;
    status: "enabled" | "disabled",
    type: TwitchEventsubSubscriptionType;
    version: "1";
    cost: number;
    condition: {
        "broadcaster_user_id": string;
    };
    transport: {
        method: "webhook",
        callback: string;
    };
    "created_at": Date;
}

export interface TwitchEventsubEvent {
    id: string;
    "broadcaster_user_id": string;
    "broadcaster_user_login": string; // (lowercase)
    "broadcaster_user_name": string; // (Display name)
    "user_id": string;
    "user_login": string; // (lowercase)
    "user_name": string; // (Display name)
}

export interface ChannelPointsCustomRewardRedemptionAdd extends TwitchEventsubEvent {
    "redeemed_at": Date;
    "user_input": string;
    reward: ChannelPointCustomReward;
    status: "unfulfilled";
}

interface ChannelPointCustomReward {
    cost: number;
    id: string;
    prompt: string;
    title: ChannelPointCustomRewardTitle;
}

export enum ChannelPointCustomRewardTitle {
    PUSHUP_PLUS_1 = "+1 Pushup",
    SITUP_PLUS_1 = "+1 Situp",
    AD_1_MIN = "1 min ad",
    AD_2_MIN = "2 min ad",
    AD_3_MIN = "3 min ad",
    EMOTE_ONLY = "Emote-only Chat",
    FIRST = "First",
    HYDRATE = "Hydrate!",
    TALK_IN_JAPANESE = "Make me talk in Japanese",
    TALK_IN_SWEDISH = "Make me talk in Swedish",
    NICE = "Nice",
    POSTURE_CHECK = "Posture Check!",
    STRETCH = "Streeeeeeeeeetch",
    TIMEOUT = "Timeout",
    MINUS_420 = "-420",
}

export enum TwitchEventsubSubscriptionType {
    CHANNEL_POINT_REDEMPTION_ADD = "channel.channel_points_custom_reward_redemption.add",
    STREAM_ONLINE = "stream.online",
}
