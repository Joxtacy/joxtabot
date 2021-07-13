import type { DenonConfig } from "https://deno.land/x/denon/mod.ts";

const config: DenonConfig = {
    scripts: {
        start: {
            cmd: "deno run main.ts",
            desc: "Starts the Twitch bot locally",
            allow: ["net", "env", "read"],
        },
    },
};

export default config;
