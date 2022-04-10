import { hmac } from "https://deno.land/x/hmac@v2.0.1/mod.ts";

export const verifySignature = (headers: Headers, body: string) => {
  const signingSecret = Deno.env.get("TWITCH_SIGNING_SECRET");
  const messageId = headers.get("Twitch-Eventsub-Message-Id") ?? "";
  const timestamp = headers.get("Twitch-Eventsub-Message-Timestamp");
  const signature = headers.get("Twitch-Eventsub-Message-Signature");

  if (!timestamp) {
    throw new Error("[TWITCH] Verification failed: Timestamp missing");
  }

  const time = Date.now();
  const timestampRequest = new Date(timestamp).getTime();

  if (Math.abs(time - timestampRequest) > 600000) {
    throw new Error("[TWITCH] Verification failed: Timeout");
  }

  if (!signingSecret) {
    throw new Error("[TWITCH] Verification failed: Signing Secret Missing");
  }

  const computedSignature = `sha256=${
    hmac(
      "sha256",
      signingSecret,
      messageId + timestamp + body,
      "utf8",
      "hex",
    )
  }`;

  return signature === computedSignature;
};
