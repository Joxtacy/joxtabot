import type { FC } from "hono/jsx";
import { Emote, RabbitMessage } from "../types";

const replaceEmotes = (message: string, emotes: Emote[]) => {
	const reversed = emotes.toReversed();
	for (const emote of reversed) {
		const start = emote.charRange[0];
		const end = emote.charRange[1];
		const code = emote.id;
		const url = emote.urlTemplate.replace("{{id}}", emote.id).replace("{{format}}", emote.format).replace("{{theme_mode}}", "dark").replace("{{scale}}", "1.0");
		message = message.slice(0, start) + `<img src="${url}" alt="${code}" />` + message.slice(end);
	}
	return message;
}
const ChatMessage: FC<RabbitMessage> = ({
	message,
	sender,
	color,
	badges,
	emotes,
}) => {

	const msg = replaceEmotes(message, emotes);
	return (
		<div
			class="chat-message"
			_="init wait 15s then add .hidden then settle then remove me"
		>
			{badges?.map(({ name, iconUrl }) => (
				<img src={iconUrl} alt={name} />
			))}
			<span class="sender" style={{ color }}>{sender}</span>
			<span class="divider">:&#20;</span>
			<span class="message" dangerouslySetInnerHTML={{ __html: msg }}></span>
		</div>
	)
};

export default ChatMessage;
