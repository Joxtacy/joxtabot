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

// Insomnia:
// OAuth client credentials flow: take the "access_token" from the response
// and use it as a Bearer token to make requests
