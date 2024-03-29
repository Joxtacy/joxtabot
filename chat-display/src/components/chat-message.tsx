import type { FC } from "hono/jsx";
import { RabbitMessage } from "../types";

const ChatMessage: FC<RabbitMessage> = ({
	message,
	sender,
	color,
	badges,
}) => (
	<div
		class="chat-message"
		_="init wait 15s then add .hidden then settle then remove me"
	>
		{badges?.map(({ name, iconUrl }) => (
			<img src={iconUrl} alt={name} />
		))}
		<span class="sender" style={{ color }}>{sender}</span>
		<span class="divider">: </span>
		<span class="message">{message}</span>
	</div>
);

export default ChatMessage;
