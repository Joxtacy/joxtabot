export default {
    envFile: ".env",
    scripts: {
        "start-server": {
            desc: "Run Discord Joxtabot with deployctl",
            cmd: "deployctl run --watch main.ts",
            allow: ["net", "env", "read", "write"],
        },
        dev: {
            desc: "Start Joxtabot in dev mode",
            cmd: "deno run --watch main.ts",
            allow: ["net", "env", "read", "write"],
        },
        start: {
            desc: "Start Joxtabot",
            cmd: "deno run main.ts",
            allow: ["net", "env", "read", "write"],
        },
        fmt: {
            desc: "Format code",
            cmd: "deno fmt --config deno.tsconfig.json",
        },
        test: {
            desc: "Run tests",
            cmd: "deno test --jobs",
            unstable: true,
        },
    },
};
