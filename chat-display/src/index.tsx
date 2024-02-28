import { Hono } from 'hono'
import { serveStatic } from 'hono/bun'
import { streamSSE } from 'hono/streaming'
import { logger } from 'hono/logger'
import amqplib from "amqplib";
import type { FC } from 'hono/jsx'
import { showRoutes } from 'hono/dev';

let id = 0;

const rabbitHost = process.env.RABBIT_HOST || 'localhost';
const connection = await amqplib.connect(`amqp://guest:guest@${rabbitHost}`);
const channel = await connection.createChannel();
await channel.assertQueue("chat");

const Main: FC = () => {
  return (
    <html lang="en">
      <head>
        <title>Chatter</title>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
        {/* Server Sent Events plugin */}
        <script src="https://unpkg.com/htmx.org/dist/ext/sse.js"></script>
        {/* _hyperscript */}
        <script src="https://unpkg.com/hyperscript.org@0.9.12"></script>

        <link rel="preconnect" href="https://fonts.googleapis.com" />
        {/* @ts-ignore */}
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Roboto&display=swap" rel="stylesheet" />

        <link href="public/style.css" rel="stylesheet" />
      </head>
      <body>
        <div class="chat-container"
          hx-ext="sse,class-tools,remove-me"
          sse-connect="/chat"
          sse-swap="chat"
          hx-swap="beforeend">
        </div>
      </body>
    </html>
  )
}

const ChatMessage: FC<{ message: string }> = ({ message }) => <div
  class="chat-message"
  _="init wait 10s then add .hidden then settle then remove me"
>
  {message}
</div>;

type Variables = {
  channel: amqplib.Channel
}

const app = new Hono<{ Variables: Variables }>()

app.onError((err, c) => {
  console.error(`${err}`)
  return c.text('Custom Error Message', 500)
})

app.use(async (c, next) => {
  c.set("channel", channel);
  await next()
})

app.use(logger())

app.use('/public/*', serveStatic({ root: './' }))
app.use('/favicon.ico', serveStatic({ path: './public/favicon.ico' }))
app.get('/', (c) => {
  return c.html(<Main />)
})
app.get("/chat", async (c) => {
  return streamSSE(c, async (stream) => {
    let connected = true

    const channel = c.get("channel")
    const consumerTag = await channel.consume("chat", async (message: any) => {
      console.log("Message received:", message?.content.toString());
      await stream.writeSSE({
        event: "chat",
        data: ChatMessage({ message: message?.content.toString() }).toString()
      });
    }, { noAck: true });
    console.log("consumerTag", consumerTag.consumerTag);

    c.req.raw.signal.addEventListener("abort", () => {
      console.log("Client disconnected");
      channel.cancel(consumerTag.consumerTag);
      connected = false;
    });

    while (connected) {
      await stream.writeSSE({
        event: "chat",
        data: ChatMessage({ message: `id: ${id++}` }).toString(),
        // id: String(id),
      });
      await stream.sleep(1000);
    }
  });
});

showRoutes(app)

export default {
  port: 3000,
  fetch: app.fetch,
}
