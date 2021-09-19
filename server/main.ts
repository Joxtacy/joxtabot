import { listenAndServe } from "https://deno.land/std@0.107.0/http/server.ts";

listenAndServe(":8000", async (request) => {
    const url = new URL(request.url);
    // const urlSearchParams = new URLSearchParams(url.search);

    const body = await json(request.body);
    console.log("body", body);

    switch (url.pathname) {
        case "/herp": {
            return new Response("derp\n");
        }
        case "/hello": {
            return new Response("Hello Nerd!\n");
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
