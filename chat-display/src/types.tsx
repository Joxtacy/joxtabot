/**
 * Message coming from RabbitMQ
 */
export interface RabbitMessage {
	message: string;
	sender: string;
	color?: string;
	badges: Badge[];
}

/**
 * Badge information on a Twitch message
 */
export interface Badge {
	name: string;
	iconUrl: string;
	version: string;
}
