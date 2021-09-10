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
    "broadcaster_user_login": string;
    "broadcaster_user_name": string;
    "user_id": string;
    "user_login": string;
    "user_name": string;
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

export type TwitchEventsubSubscriptionType = "channel.channel_points_custom_reward_redemption.add" | "stream.online";
