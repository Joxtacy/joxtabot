import { listenAndServe } from "https://deno.land/std@0.107.0/http/server.ts";

const PORT = `:${Deno.env.get("PORT")}`;

listenAndServe(PORT, async (request) => {
    console.info(`Server is up and running! Listening on port ${PORT}`);
    const url = new URL(request.url);
    // const urlSearchParams = new URLSearchParams(url.search);

    const body = await json(request.body);
    console.log("body", body);

    switch (url.pathname) {
        case "/herp": {
            return createResponse({ herp: "derp", hurr: "durr" });
        }
        case "/hello": {
            return createResponse("Hello fren!\n");
        }
        default: {
            return new Response("This is not acceptable!\n");
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
    status?: number
): Response {
    const init: ResponseInit = { status };
    if (typeof data === "string") {
        return new Response(new TextEncoder().encode(data), init);
    }
    return new Response(new TextEncoder().encode(JSON.stringify(data)), init);
}