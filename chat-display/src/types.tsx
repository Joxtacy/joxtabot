/**
 * Message coming from RabbitMQ
 */
export interface RabbitMessage {
	message: string;
	sender: string;
	color?: string;
	badges: Badge[];
	emotes: Emote[];
}

/**
 * Badge information on a Twitch message
 */
export interface Badge {
	name: string;
	iconUrl: string;
	version: string;
}

/**
 * Emote information on a Twitch message
 */
export interface Emote {
	id: string;
	code: string;
	charRange: number[];
	urlTemplate: string;
}
