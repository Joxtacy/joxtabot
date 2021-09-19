// const Discord = require("discord.js");
// const client = new Discord.Client();
const WebSocket = require("ws");
import axios from "axios";
import fetch from "node-fetch";
import { io } from "socket.io-client";
import { Client, Message, MessageEmbed, TextChannel } from "discord.js";
import { socketEventHandler } from "./messageHandlers";
const client = new Client();

let sessionId = "";

axios
    .get("https://discord.com/api/gateway/bot", {
        headers: { Authorization: `Bot ${process.env.DISCORD_TOKEN}` },
    })
    .then((response) => {
        const { data } = response;
        console.log("gateway", data);
        const gatewayUrl = data.url;
        const socket = new WebSocket(`${gatewayUrl}?v=9&encoding=json`);

        const {
            socketOpenHandler,
            socketCloseHandler,
            socketMessageHandler
        } = socketEventHandler(socket);

        socket.addEventListener("open", socketOpenHandler);
        socket.addEventListener("close", socketCloseHandler);
        socket.addEventListener("message", socketMessageHandler);
    });
/*
fetch("https://discord.com/api/gateway/bot", {
    headers: {
        Authorization: `Bot ${process.env.DISCORD_TOKEN}`,
    },
})
    .then((res) => res.json())
    .then((json) => {
        console.log("gateway", json);
        const gatewayUrl = json.url;
        const socket = new WebSocket(`${gatewayUrl}?v=9&encoding=json`);

        socket.addEventListener("open", (event) => {
            console.log("Socket is open");
            socket.send(
                JSON.stringify({
                    op: 2,
                    d: {
                        token: process.env.DISCORD_TOKEN,
                        intents: 513,
                        properties: {
                            $os: "mac",
                            $browser: "Joxtabot",
                            $device: "Joxtabot",
                        },
                    },
                })
            );
        });
        socket.addEventListener("close", (event) =>
            console.log("socket closed")
        );
        socket.addEventListener("message", (message) => {
            const messageData = JSON.parse(message.data);
            // console.log("received message", JSON.parse(message.data));

            if (messageData.op == 10) {
                setTimeout(() => {
                    const data = {
                        op: 1,
                        d: messageData.s,
                    };
                    socket.send(JSON.stringify(data));
                    setInterval(() => {
                        const data = {
                            op: 1,
                            d: messageData.s,
                        };
                        socket.send(JSON.stringify(data));
                    }, messageData.d.heartbeat_interval);
                }, Math.random() * messageData.d.heartbeat_interval);
            }

            if (messageData.t === "READY") {
                sessionId = messageData.d.session_id;
                console.log(
                    "a new player enters the game",
                    messageData.d.user.username
                );
            }

            if (messageData.t === "INTERACTION_CREATE") {
                console.log("We got an interaction");
                const {
                    data,
                    id: interactionId,
                    application_id: applicationId,
                    token: interactionToken,
                } = messageData.d;

                console.log("interaction data", data);

                const deferred = {
                    type: 5,
                };
                const interactionsResponseUrl = `https://discord.com/api/v8/interactions/${interactionId}/${interactionToken}/callback`;

                axios
                    .post(interactionsResponseUrl, deferred)
                    .then((res) => console.log("interaction response success"))
                    .catch((error) =>
                        console.error("interaction oopsie", error)
                    );

                axios
                    .get("https://api.thedogapi.com/v1/images/search")
                    .then((response) => {
                        const dog = response.data[0];
                        const url = dog.url;
                        const embed = new MessageEmbed().setImage(url);
                        const data = {
                            tts: false,
                            content: "Here's your requested animal",
                            embeds: [embed],
                            allowed_mentions: { parse: [] },
                        };
                        const updateInteractionUrl = `https://discord.com/api/v8/webhooks/${applicationId}/${interactionToken}/messages/@original`;
                        axios
                            .patch(updateInteractionUrl, data)
                            .then((response) => console.log("update"))
                            .catch((error) => console.log("update error"));
                    });
            }
        });
    });
    */

client.once("ready", () => {
    console.info("READY!");
});

const messageHandler = (message: Message) => {
    if (message.author.bot) return;

    if (!(message.channel instanceof TextChannel)) return;

    if (message.content.startsWith("!censor")) {
        const censoredContent = message.content
            .replace("heck", "h**k")
            .replace("!censor", "")
            .trim();
        message.delete({
            reason: "Not allowed to use the word 'heck' in this server.",
        });
        message.reply(censoredContent);
        return;
    }

    switch (message.content.trim()) {
        case "!cat": {
            fetch("https://api.thecatapi.com/v1/images/search")
                .then((res) => res.json())
                .then((result) => {
                    const cat = result[0];
                    const url = cat.url;
                    const embed = new MessageEmbed().setImage(url);
                    message.reply({ embed });
                });
            return;
        }
        case "!dog": {
            fetch("https://api.thedogapi.com/v1/images/search")
                .then((res) => res.json())
                .then((result) => {
                    const dog = result[0];
                    const url = dog.url;
                    const embed = new MessageEmbed().setImage(url);
                    message.reply({ embed });
                });
            return;
        }
        case "!ping": {
            message.channel.send("Pong. 🏓");
            return;
        }
        case "!avatar": {
            message.reply(
                `Your avatar: <${message.author.displayAvatarURL({
                    format: "png",
                    dynamic: true,
                })}>`
            );
            return;
        }
        case ":smokeyCute:": {
            message.channel.send(message.content.trim());
            return;
        }
        case "how to embed": {
            const embed = new MessageEmbed()
                // Set the title of the field
                .setTitle("A slick little embed")
                // Set the color of the embed
                .setColor(0xff0000)
                // Set the main content of the embed
                .setDescription("Hello, this is a slick embed!");
            // Send the embed to the same channel as the message
            message.channel.send(embed);
            return;
        }
    }

    if (message.channel.name === "testing-joxtabot") {
        let changed = false;
        const words = message.content.split(" ");

        const buttWordIndex = Math.floor(Math.random() * words.length);
        const buttWord = words[buttWordIndex];

        if (buttWord.length <= 4) {
            words[buttWordIndex] = "butt";
            changed = true;
        }

        changed && message.channel.send(words.join(" "));
    }
};

client.on("message", messageHandler);

client.login(process.env.DISCORD_TOKEN);
