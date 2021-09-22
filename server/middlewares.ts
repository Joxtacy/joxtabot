import type { Context } from "https://deno.land/x/oak@v9.0.1/mod.ts";
import { isHttpError, Status } from "https://deno.land/x/oak@v9.0.1/mod.ts";

export const logger = async (ctx: Context, next: () => Promise<unknown>) => {
    await next();
    const rt = ctx.response.headers.get("X-Response-Time");
    console.log(`${ctx.request.method} ${ctx.request.url} - ${rt}`);
};

export const timing = async (ctx: Context, next: () => Promise<unknown>) => {
    const start = Date.now();
    await next();
    const ms = Date.now() - start;
    ctx.response.headers.set("X-Response-Time", `${ms}ms`);
};

export const notFound = (ctx: Context) => {
    const { response } = ctx;
    response.status = 404;
    response.type = "text/html";
    response.body = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>☠️ Oh noes, the page was not found ☠️</h1>
</body>
</html>
`;
};

export const errorHandler = async (
    ctx: Context,
    next: () => Promise<unknown>
) => {
    try {
        await next();
    } catch (error) {
        console.log("WE GOT AN ERROR", error);
        if (isHttpError(error)) {
            switch (error.status) {
                case Status.NotFound: {
                    ctx.response.type = "text/html";
                    ctx.response.body = `
                    `;
                    break;
                }
                default: {
                    // handle other statuses
                }
            }
        } else {
            // rethrow if can't handle the error
            throw error;
        }
    }
};
