"use strict";
exports.__esModule = true;
// const Discord = require("discord.js");
// const client = new Discord.Client();
var WebSocket = require("ws");
var axios_1 = require("axios");
var node_fetch_1 = require("node-fetch");
var discord_js_1 = require("discord.js");
var client = new discord_js_1.Client();
var sessionId = "";
axios_1["default"]
    .get("https://discord.com/api/gateway/bot", {
    headers: { Authorization: "Bot " + process.env.DISCORD_TOKEN }
})
    .then(function (response) {
    var data = response.data;
    console.log("gateway", data);
    var gatewayUrl = data.url;
    var socket = new WebSocket(gatewayUrl + "?v=9&encoding=json");
    socket.addEventListener("open", function (event) {
        console.log("Socket is open");
        socket.send(JSON.stringify({
            op: 2,
            d: {
                token: process.env.DISCORD_TOKEN,
                intents: 513,
                properties: {
                    $os: "mac",
                    $browser: "Joxtabot",
                    $device: "Joxtabot"
                }
            }
        }));
    });
    socket.addEventListener("close", function (event) {
        return console.log("socket closed");
    });
    socket.addEventListener("message", function (message) {
        var messageData = JSON.parse(message.data);
        // console.log("received message", JSON.parse(message.data));
        if (messageData.op == 10) {
            setTimeout(function () {
                var data = {
                    op: 1,
                    d: messageData.s
                };
                socket.send(JSON.stringify(data));
                setInterval(function () {
                    var data = {
                        op: 1,
                        d: messageData.s
                    };
                    socket.send(JSON.stringify(data));
                }, messageData.d.heartbeat_interval);
            }, Math.random() * messageData.d.heartbeat_interval);
        }
        if (messageData.t === "READY") {
            sessionId = messageData.d.session_id;
            console.log("a new player enters the game", messageData.d.user.username);
        }
        if (messageData.t === "INTERACTION_CREATE") {
            console.log("We got an interaction");
            var _a = messageData.d, data_1 = _a.data, interactionId = _a.id, applicationId = _a.application_id, interactionToken = _a.token;
            console.log("interaction data", data_1);
            var deferred = {
                type: 5
            };
            var interactionsResponseUrl = "https://discord.com/api/v8/interactions/" + interactionId + "/" + interactionToken + "/callback";
            axios_1["default"]
                .post(interactionsResponseUrl, deferred)
                .then(function (res) { return console.log("interaction response success"); })["catch"](function (error) {
                return console.error("interaction oopsie", error);
            });
            console.log("data", data_1);
            var animalUrl = "";
            if (data_1.options[0].value === "animal_dog") {
                animalUrl = "https://api.thedogapi.com/v1/images/search";
            }
            else if (data_1.options[0].value === "animal_cat") {
                animalUrl = "https://api.thecatapi.com/v1/images/search";
            }
            var updateInteractionUrl_1 = "https://discord.com/api/v8/webhooks/" + applicationId + "/" + interactionToken + "/messages/@original";
            axios_1["default"]
                .get(animalUrl)
                .then(function (response) {
                var animal = response.data[0];
                var url = animal.url;
                var embed = new discord_js_1.MessageEmbed().setImage(url);
                var data = {
                    tts: false,
                    content: "Here's your requested animal",
                    embeds: [embed],
                    allowed_mentions: { parse: [] }
                };
                axios_1["default"]
                    .patch(updateInteractionUrl_1, data)
                    .then(function (response) { return console.log("update"); })["catch"](function (error) { return console.log("update error"); });
            })["catch"](function (error) {
                axios_1["default"]
                    .patch(updateInteractionUrl_1, {
                    content: "Couldn't find an animal picture... 😭"
                })
                    .then(function (response) { return console.log("update"); })["catch"](function (error) { return console.log("update error"); });
            });
        }
    });
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
client.once("ready", function () {
    console.info("READY!");
});
var messageHandler = function (message) {
    if (message.author.bot)
        return;
    if (!(message.channel instanceof discord_js_1.TextChannel))
        return;
    if (message.content.startsWith("!censor")) {
        var censoredContent = message.content
            .replace("heck", "h**k")
            .replace("!censor", "")
            .trim();
        message["delete"]({
            reason: "Not allowed to use the word 'heck' in this server."
        });
        message.reply(censoredContent);
        return;
    }
    switch (message.content.trim()) {
        case "!cat": {
            node_fetch_1["default"]("https://api.thecatapi.com/v1/images/search")
                .then(function (res) { return res.json(); })
                .then(function (result) {
                var cat = result[0];
                var url = cat.url;
                var embed = new discord_js_1.MessageEmbed().setImage(url);
                message.reply({ embed: embed });
            });
            return;
        }
        case "!dog": {
            node_fetch_1["default"]("https://api.thedogapi.com/v1/images/search")
                .then(function (res) { return res.json(); })
                .then(function (result) {
                var dog = result[0];
                var url = dog.url;
                var embed = new discord_js_1.MessageEmbed().setImage(url);
                message.reply({ embed: embed });
            });
            return;
        }
        case "!ping": {
            message.channel.send("Pong. 🏓");
            return;
        }
        case "!avatar": {
            message.reply("Your avatar: <" + message.author.displayAvatarURL({
                format: "png",
                dynamic: true
            }) + ">");
            return;
        }
        case ":smokeyCute:": {
            message.channel.send(message.content.trim());
            return;
        }
        case "how to embed": {
            var embed = new discord_js_1.MessageEmbed()
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
        var changed = false;
        var words = message.content.split(" ");
        var buttWordIndex = Math.floor(Math.random() * words.length);
        var buttWord = words[buttWordIndex];
        if (buttWord.length <= 4) {
            words[buttWordIndex] = "butt";
            changed = true;
        }
        changed && message.channel.send(words.join(" "));
    }
};
client.on("message", messageHandler);
client.login(process.env.DISCORD_TOKEN);
