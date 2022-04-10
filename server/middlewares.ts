import type { Context } from "https://deno.land/x/oak@v9.0.1/mod.ts";
import { isHttpError, Status } from "https://deno.land/x/oak@v9.0.1/mod.ts";

const X_RESPONSE_TIME = "X-Response-Time";

export const logger = async (
  { response, request }: Context,
  next: () => Promise<unknown>,
) => {
  await next();
  const rt = response.headers.get(X_RESPONSE_TIME);
  console.log(`${request.method} ${request.url} - ${rt}`);
};

export const timing = async (
  { response }: Context,
  next: () => Promise<unknown>,
) => {
  const start = Date.now();
  await next();
  const ms = Date.now() - start;
  response.headers.set(X_RESPONSE_TIME, `${ms}ms`);
};

export const notFound = ({ response }: Context) => {
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
  { response }: Context,
  next: () => Promise<unknown>,
) => {
  try {
    await next();
  } catch (error) {
    console.log("WE GOT AN ERROR", error);
    if (isHttpError(error)) {
      switch (error.status) {
        case Status.NotFound: {
          response.type = "text/html";
          response.body = `
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
