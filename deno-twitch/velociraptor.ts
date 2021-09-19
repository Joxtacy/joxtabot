export default {
    envFile: ".env",
    scripts: {
        "start-server": {
            desc: "Run Joxtabot with deployctl",
            cmd: "deployctl run --watch main.ts",
            allow: ["net", "env", "read"],
        },
        start: {
            desc: "Run Joxtabot with deno",
            cmd: "deno run --watch main.ts",
            allow: ["net", "env", "read"],
        },
        fmt: {
            desc: "Format code",
            cmd: "deno fmt --config deno.tsconfig.json",
        },
        herp: {
            desc: "Just a test thingy",
            cmd: "deno run derp.ts",
            allow: ["env"],
        },
        derp: {
            desc: "Just a test thingy",
            cmd: "deployctl run derp.ts",
        },
    },
};
