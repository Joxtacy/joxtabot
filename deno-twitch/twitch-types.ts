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
    title: string;
}

export enum ChannelPointCustomRewardTitle {
    "+1 Pushup",
    "+1 Situp",
    "1 min ad",
    "2 min ad",
    "3 min ad",
    "Emote-only Chat",
    "First",
    "Hydrate!",
    "Make me talk in Japanese",
    "Make me talk in Swedish",
    "Nice",
    "Posture Check!",
    "Streeeeeeeeeetch",
    "Timeout",
    // "-420", // Can't have numeric name
}

export enum TwitchEventsubSubscriptionType {
    "channel.channel_points_custom_reward_redemption.add",
    "stream.online"
}

