import type { FC } from "hono/jsx";
import { Emote, RabbitMessage } from "../types";

const replaceEmotes = (message: string, emotes: Emote[]) => {
	const reversed = emotes.toReversed();
	let components = [<Message part={message} />];
	let modMsg = message;
	for (const emote of reversed) {
		const start = emote.charRange[0];
		const end = emote.charRange[1];
		const code = emote.id;
		const url = emote.urlTemplate.replace("{{id}}", emote.id).replace("{{format}}", emote.format).replace("{{theme_mode}}", "dark").replace("{{scale}}", "1.0");
		const firstPart = modMsg.slice(0, start);
		const lastPart = modMsg.slice(end);
		const emoteComponent = <EmoteComponent src={url} alt={code} />;
		modMsg = firstPart;

		components = components.slice(1);
		components = [<Message part={firstPart} />, emoteComponent, <Message part={lastPart} />, ...components];
	}
	return components;
};

const Message: FC<{ part: string }> = ({ part }) => {
	return (
		// the "white-space: pre" keeps whitespaces at the beginning and end of the message
		<span style={{ "white-space": "pre" }}>{part}</span>
	)
};

const EmoteComponent: FC<{ src: string, alt: string }> = ({ src, alt }) => {
	return (
		<img src={src} alt={alt} />
	)
};

const ChatMessage: FC<RabbitMessage> = ({
	message,
	sender,
	color,
	badges,
	emotes,
}) => {
	const messageComponents = replaceEmotes(message, emotes);
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
			{messageComponents.map((component) => component)}
		</div>
	)
};

export default ChatMessage;
