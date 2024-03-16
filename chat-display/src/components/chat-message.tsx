import type { FC } from "hono/jsx";

const ChatMessage: FC<{ message: string; sender: string; }> = ({
	message,
	sender,
}) => (
	<div
		class="chat-message"
		_="init wait 15s then add .hidden then settle then remove me"
	>
		<span class="sender">{sender}</span>
		<span class="divider">: </span>
		<span class="message">{message}</span>
	</div>
);

export default ChatMessage;
