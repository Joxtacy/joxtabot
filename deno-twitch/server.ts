import {
    serve,
    HTTPOptions,
    ServerRequest,
} from "https://deno.land/std@0.101.0/http/server.ts";
import { hmac } from "https://deno.land/x/hmac@v2.0.1/mod.ts";

const twitchSigningSecret = Deno.env.get("TWITCH_SIGNING_SECRET");

class Application {
    private getters: Map<string, (req: ServerRequest) => void>;
    private posters: Map<string, (req: ServerRequest) => void>;
    private middlewares: Array<(req: ServerRequest) => Promise<void>>;

    constructor() {
        this.getters = new Map();
        this.posters = new Map();
        this.middlewares = [];
    }

    use(handler: (req: ServerRequest) => Promise<void>) {
        this.middlewares.push(handler);
    }

    get(path: string, handler: (req: ServerRequest) => void) {
        this.getters.set(path, handler);
    }

    post(path: string, handler: (req: ServerRequest) => void) {
        this.posters.set(path, handler);
    }

    async listen(options: HTTPOptions) {
        const server = serve(options);
        for await (const req of server) {
            const { method, url } = req;
            const [path] = url.split("?");

            for await (const middleware of this.middlewares) {
                await middleware(req);
            }

            switch (method) {
                case "GET": {
                    if (this.getters.has(path)) {
                        const func = this.getters.get(path) as Function;
                        func(req);
                    } else {
                        req.respond({ status: 404 });
                    }
                    break;
                }
                case "POST": {
                    if (this.posters.has(path)) {
                        const func = this.posters.get(path) as Function;
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
