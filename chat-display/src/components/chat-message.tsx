import type { FC } from "hono/jsx";
import { RabbitMessage } from "../types";

const ChatMessage: FC<RabbitMessage> = ({
	message,
	sender,
	color,
}) => (
	<div
		class="chat-message"
		_="init wait 15s then add .hidden then settle then remove me"
	>
		<span class="sender" style={{ color }}>{sender}</span>
		<span class="divider">: </span>
		<span class="message">{message}</span>
	</div>
);

export default ChatMessage;
