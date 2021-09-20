import { listenAndServe } from "https://deno.land/std@0.107.0/http/server.ts";

const PORT = `:${Deno.env.get("PORT")}`;

console.info(`Server is up and running! Listening on port ${PORT}`);
await listenAndServe(PORT, async (request) => {
    const url = new URL(request.url);
    // const urlSearchParams = new URLSearchParams(url.search);

    const body = await json(request.body);
    console.log("body", body);

    switch (url.pathname) {
        case "/": {
            const html = `
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta http-equiv="X-UA-Compatible" content="IE=edge">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Joxtabot</title>
</head>
<body>
    <h1>Welcome to Joxtabot, fren!</h1>
    <h2>How are you doing today?</h2>
</body>
</html>
            `;
            return createResponse(html, {
                headers: new Headers({
                    "Content-Type": "text/html; charset=UTF-8",
                }),
            });
        }
        case "/herp": {
            return createResponse({ herp: "derp", hurr: "durr" });
        }
        case "/hello": {
            return createResponse("Hello fren!");
        }
        default: {
            return createResponse("This is not acceptable!", { status: 404 });
        }
    }
});

async function json(
    readableStream: ReadableStream<Uint8Array> | null
): Promise<unknown> {
    if (!readableStream) {
        return null;
    }
    const reader = readableStream.getReader();
    const newReadableStream = new ReadableStream({
        start(controller) {
            (function pump(): unknown {
                return reader?.read().then(({ done, value }) => {
                    if (done) {
                        controller.close();
                        return;
                    }
                    controller.enqueue(value);
                    return pump();
                });
            })();
        },
    });

    const response = new Response(newReadableStream);
    const blob = await response.blob();
    const arrBuf = await blob.arrayBuffer();

    return JSON.parse(new TextDecoder().decode(arrBuf));
}

function createResponse(
    data: string | Record<string, unknown>,
    init?: ResponseInit
): Response {
    if (typeof data === "string") {
        const dataWithNewline = data.endsWith("\n") ? data : data + "\n";
        return new Response(new TextEncoder().encode(dataWithNewline), init);
    }
    return new Response(new TextEncoder().encode(JSON.stringify(data)), init);
}