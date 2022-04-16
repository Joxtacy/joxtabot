export const getStreamInfo = async (userId: string | number) => {
  const result = await fetch(
    `https://api.twitch.tv/helix/streams?user_id=${userId}`,
    {
      headers: {
        Authorization: `Bearer ${
          Deno.env.get(
            "TWITCH_APP_ACCESS_TOKEN",
          )
        }`,
        "Client-Id": `${Deno.env.get("TWITCH_CLIENT_ID")}`,
      },
    },
  );
  const { data } = await result.json();
  const title: string = data[0]?.title;
  const gameName: string = data[0]?.game_name;

  if (title === undefined && gameName === undefined) {
    throw new Error("Stream is not online");
  }

  return {
    title,
    gameName,
  };
};

type Seconds = number;

interface TwitchOauthTokenResponse {
  access_token: string;
  expires_in: Seconds;
  scope: Array<string>;
  token_type: "bearer";
}

/*
 * Fetches a new Bearer token for the Twitch Bot
 */
export const getAuthToken = async () => {
  const url = "https://id.twitch.tv/oauth2/token";

  const params = new URLSearchParams();
  params.append("client_id", Deno.env.get("TWITCH_CLIENT_ID") || "");
  params.append("client_secret", Deno.env.get("TWITCH_CLIENT_SECRET") || "");
  params.append("grant_type", "client_credentials");
  params.append("scope", "channel:manage:redemptions");

  const init = {
    method: "POST",
  };

  const request = new Request(`${url}?${params.toString()}`, init);
  const response = await fetch(request);
  const json = await response.json() as TwitchOauthTokenResponse;
  return json;
};

// Insomnia:
// OAuth client credentials flow: take the "access_token" from the response
// and use it as a Bearer token to make requests
