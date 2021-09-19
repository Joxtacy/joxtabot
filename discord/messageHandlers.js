"use strict";
exports.__esModule = true;
exports.socketEventHandler = void 0;
var axios_1 = require("axios");
var discord_js_1 = require("discord.js");
// import { opcodeHandlers } from "./gatewayOpcodeHandlers";
var socketEventHandler = function (socket) {
    return {
        socketOpenHandler: function (event) {
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
        },
        socketCloseHandler: function (event) {
            console.log("socket closed");
        },
        socketMessageHandler: function (message) {
            var messageData = JSON.parse(message.data);
            // console.log("received message", JSON.parse(message.data));
            if (messageData.op == 10) {
                // const { op10Handler } = opcodeHandlers(socket);
                // op10Handler(messageData.s, messageData.d.heartbeat_interval);
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
                // sessionId = messageData.d.session_id;
                console.log("a new player enters the game", messageData.d.user.username);
            }
            if (messageData.t === "INTERACTION_CREATE") {
                console.log("We got an interaction");
                var _a = messageData.d, data = _a.data, interactionId = _a.id, applicationId = _a.application_id, interactionToken = _a.token;
                console.log("interaction data", data);
                var deferred = {
                    type: 5
                };
                var interactionsResponseUrl = "https://discord.com/api/v8/interactions/" + interactionId + "/" + interactionToken + "/callback";
                axios_1["default"]
                    .post(interactionsResponseUrl, deferred)
                    .then(function (res) { return console.log("interaction response success"); })["catch"](function (error) {
                    return console.error("interaction oopsie", error);
                });
                console.log("data", data);
                var animalUrl = "";
                if (data.options[0].value === "animal_dog") {
                    animalUrl = "https://api.thedogapi.com/v1/images/search";
                }
                else if (data.options[0].value === "animal_cat") {
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
        }
    };
};
exports.socketEventHandler = socketEventHandler;
