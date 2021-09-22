import { green, yellow } from "https://deno.land/std@0.108.0/fmt/colors.ts";
import { Application, Router } from "https://deno.land/x/oak@v9.0.1/mod.ts";
import { logger, timing, errorHandler, notFound } from "./middlewares.ts";
import { verifySignature } from "./twitch/utils.ts";

const PORT = Deno.env.get("PORT") || "8000";

const app = new Application();
const router = new Router();

app.use(logger);
app.use(timing);
app.use(errorHandler);

router.get("/", (context) => {
    context.response.type = "text/html";
    context.response.body = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Hello Oak Server</h1>
</body>
</html>
    `;
});

router.post("/twitch/webhooks/callback", async ({ request, response }) => {
    const body = await request.body().value; // as Twitch webhook something interface
    const verification = verifySignature(request.headers, JSON.stringify(body));

    if (verification) {
        response.status = 200;
        response.body = body.challenge;
    }
});

app.use(router.routes());
app.use(router.allowedMethods());
// Handling of 404 will kick in if no routes were matched, so this one must be placed last.
app.use(notFound);

app.addEventListener("listen", ({ secure, hostname, port }) => {
    const protocol = secure ? "https://" : "http://";
    const url = `${protocol}${hostname ?? "localhost"}:${port}`;
    console.log(`${yellow("Listening on:")} ${green(url)}`);
});

await app.listen({ port: Number(PORT) });
