import amqplib from "amqplib";
import { Hono } from "hono";
import { serveStatic } from "hono/bun";
import { showRoutes } from "hono/dev";
import { logger } from "hono/logger";
import { streamSSE } from "hono/streaming";
import ChatMessage from "./components/chat-message";
import Main from "./components/main";
import type { RabbitMessage } from "./types";

let id = 0;

const rabbitHost = process.env.RABBIT_HOST || "localhost";
const connection = await amqplib.connect(`amqp://guest:guest@${rabbitHost}`);
const channel = await connection.createChannel();
await channel.assertQueue("chat");

type Variables = {
	channel: amqplib.Channel;
};

const app = new Hono<{ Variables: Variables }>();

app.onError((err, c) => {
	console.error(`${err}`);
	return c.text("Custom Error Message", 500);
});

app.use(async (c, next) => {
	c.set("channel", channel);
	await next();
});

app.use(logger());

app.use("/public/*", serveStatic({ root: "./" }));
app.use("/favicon.ico", serveStatic({ path: "./public/favicon.ico" }));
app.get("/", (c) => {
	return c.html(<Main />);
});
app.get("/chat", async (c) => {
	return streamSSE(c, async (stream) => {
		let connected = true;

		const channel = c.get("channel");
		const consumerTag = await channel.consume(
			"chat",
			async (message) => {
				const msg: RabbitMessage = JSON.parse(
					message?.content.toString() || '{"sender":null,"message":null}',
				);
				console.log("Message received:", msg);
				await stream.writeSSE({
					event: "chat",
					data: ChatMessage({ ...msg }).toString(),
				});
			},
			{ noAck: true },
		);
		console.log("consumerTag", consumerTag.consumerTag);

		c.req.raw.signal.addEventListener("abort", () => {
			console.log("Client disconnected");
			channel.cancel(consumerTag.consumerTag);
			connected = false;
		});

		while (connected) {
			await stream.writeSSE({
				event: "chat",
				data: ChatMessage({
					message: `id -> ${id++}`,
					sender: "system",
					badges: [],
				}).toString(),
				// id: String(id),
			});
			await stream.sleep(1000);
		}
	});
});

showRoutes(app);

export default {
	port: 3000,
	fetch: app.fetch,
};
