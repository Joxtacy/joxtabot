import { ChatUserstate, client, Options } from "tmi.js";
import dotenv from "dotenv";
dotenv.config();

const opts: Options = {
    identity: {
        username: process.env.JOXTABOT_USERNAME,
        password: process.env.JOXTABOT_TOKEN,
    },
    channels: JSON.parse(process.env.JOXTABOT_CHANNELS || '["joxtacy"]'),
};

// Create a client with our options
const twitchClient = new client(opts);

// Register our event handlers (defined below)
twitchClient.on('message', onMessageHandler);
twitchClient.on('connected', onConnectedHandler);
twitchClient.on('redeem', onRedeemHandler); // Only works for rewards with text input

// Connect to Twitch:
twitchClient.connect();

// Called every time a message comes in
function onMessageHandler(channel: string, userstate: ChatUserstate, message: string, self: boolean) {
    if (self) {
        return;
    } // Ignore messages from the bot

    // Remove whitespace from chat message
    const commandName = message.trim();

    // If the command is known, let's execute it
    if (commandName === '!dice') {
        const num = rollDice();
        twitchClient.say(
            channel,
            `@${userstate.username}, you rolled a ${num}`
        );
        console.log(`* Executed ${commandName} command`);
    } else {
        console.log(`* Unknown command ${commandName}`);
    }
}

// Function called when the "dice" command is issued
function rollDice() {
    const sides = 6;
    return Math.floor(Math.random() * sides) + 1;
}

// Called every time the bot connects to Twitch chat
function onConnectedHandler(addr: string, port: number) {
    console.log(`* Connected to ${addr}:${port}`);
}

function onRedeemHandler(channel: string, username: string, rewardType: string, tags: ChatUserstate) {
    console.log("ARRRRGS", arguments);
    console.log("redeem channel", channel);
    console.log("redeem username", username);
    console.log("redeem rewardType", rewardType);
    console.log("redeem tags", tags);
}
