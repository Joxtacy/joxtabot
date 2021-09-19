import axios from "axios";
import { MessageEmbed } from "discord.js";
// import { opcodeHandlers } from "./gatewayOpcodeHandlers";

export const socketEventHandler = (socket: WebSocket) => {
    return {
        socketOpenHandler: (event: Event) => {
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
        },
        socketCloseHandler: (event: Event) => {
            console.log("socket closed");
        },
        socketMessageHandler: (message: MessageEvent) => {
            const messageData = JSON.parse(message.data);
            // console.log("received message", JSON.parse(message.data));

            if (messageData.op == 10) {
                // const { op10Handler } = opcodeHandlers(socket);
                // op10Handler(messageData.s, messageData.d.heartbeat_interval);
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
                // sessionId = messageData.d.session_id;
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

                console.log("data", data);
                let animalUrl = "";
                if (data.options[0].value === "animal_dog") {
                    animalUrl = "https://api.thedogapi.com/v1/images/search";
                } else if (data.options[0].value === "animal_cat") {
                    animalUrl = "https://api.thecatapi.com/v1/images/search";
                }

                const updateInteractionUrl = `https://discord.com/api/v8/webhooks/${applicationId}/${interactionToken}/messages/@original`;
                axios
                    .get(animalUrl)
                    .then((response) => {
                        const animal = response.data[0];
                        const url = animal.url;
                        const embed = new MessageEmbed().setImage(url);
                        const data = {
                            tts: false,
                            content: "Here's your requested animal",
                            embeds: [embed],
                            allowed_mentions: { parse: [] },
                        };
                        axios
                            .patch(updateInteractionUrl, data)
                            .then((response) => console.log("update"))
                            .catch((error) => console.log("update error"));
                    })
                    .catch((error) => {
                        axios
                            .patch(updateInteractionUrl, {
                                content:
                                    "Couldn't find an animal picture... 😭",
                            })
                            .then((response) => console.log("update"))
                            .catch((error) => console.log("update error"));
                    });
            }
        },
    };
};
