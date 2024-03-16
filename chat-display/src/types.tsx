/**
 * Message coming from RabbitMQ
 */
export interface RabbitMessage {
	message: string;
	sender: string;
	color?: string;
}
