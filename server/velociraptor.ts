export default {
    envFile: ".env",
    scripts: {
        "start-server": {
            desc: "Run Discord Joxtabot with deployctl",
            cmd: "deployctl run --watch main.ts",
            allow: ["net", "env", "read"],
        },
        start: {
            desc: "Run Discord Joxtabot with deno",
            cmd: "deno run --watch main.ts",
            allow: ["net", "env", "read"],
        },
        fmt: {
            desc: "Format code",
            cmd: "deno fmt --config deno.tsconfig.json",
        },
    },
};