import {
    hmac,
    serve,
    HTTPOptions,
    ServerRequest,
} from "./deps.ts";

const twitchSigningSecret = Deno.env.get("TWITCH_SIGNING_SECRET");

type PostHandler = (req: ServerRequest) => void;
type GetHandler = (req: ServerRequest) => void;

interface FetchEvent {
    request: Request;
    respondWith: (response: Response) => void;
}

class Application {
    private middlewares: Array<(req: ServerRequest) => Promise<void>>;
    private getters: Map<string, GetHandler>;
    private posters: Map<string, PostHandler>;

    constructor() {
        this.getters = new Map();
        this.posters = new Map();
        this.middlewares = [];
    }

    use(handler: (req: ServerRequest) => Promise<void>) {
        this.middlewares.push(handler);
    }

    get(path: string, handler: GetHandler) {
        this.getters.set(path, handler);
    }

    post(path: string, handler: PostHandler) {
        this.posters.set(path, handler);
    }

    async listen(options: HTTPOptions) {
        const server = serve(options);
        for await (const req of server) {
            const { method, url, headers } = req;
            let [path] = url.split("?");
            const host = headers.get("host");

            if (host && url.includes(host)) {
                path = url.split(host)[1].split("?")[0];
            }
            /*
            for await (const middleware of this.middlewares) {
                await middleware(req);
            }
            */
            console.log(
                `[SERVER] Received request - method: ${method}, url: ${url}, path: ${path}, host: ${host}`
            );
            switch (method) {
                case "GET": {
                    if (this.getters.has(path)) {
                        const func = this.getters.get(path) as GetHandler;
                        func(req);
                    } else {
                        req.respond({ status: 404 });
                    }
                    break;
                }
                case "POST": {
                    if (this.posters.has(path)) {
                        const func = this.posters.get(path) as PostHandler;
                        func(req);
                    } else {
                        req.respond({ status: 404 });
                    }
                    break;
                }
                default: {
                    req.respond({ status: 405 });
                }
            }
        }
    }

    verifyTwitchSignature = (req: ServerRequest, body: string) => {
        const messageId = req.headers.get(
            "Twitch-Eventsub-Message-Id"
        ) as string;
        const timestamp = req.headers.get("Twitch-Eventsub-Message-Timestamp");
        const messageSignature = req.headers.get(
            "Twitch-Eventsub-Message-Signature"
        );
        const time = Math.floor(new Date().getTime() / 1000);
        console.log(`Message ${messageId} Signature: `, messageSignature);

        if (Math.abs(time - Number(timestamp)) > 600) {
            // needs to be < 10 minutes
            console.log(
                `Verification Failed: timestamp > 10 minutes. Message Id: ${messageId}.`
            );
            throw new Error("Ignore this request.");
        }

        if (!twitchSigningSecret) {
            console.log(`Twitch signing secret is empty.`);
            throw new Error("Twitch signing secret is empty.");
        }

        const computedSignature =
            "sha256=" +
            hmac(
                "sha256",
                twitchSigningSecret,
                messageId + timestamp + body,
                "utf8",
                "hex"
            );
        console.log(
            `Message ${messageId} Computed Signature: `,
            computedSignature
        );

        if (messageSignature !== computedSignature) {
            return false;
        } else {
            console.log("Verification successful");
            return true;
        }
    };
}

export default Application;
