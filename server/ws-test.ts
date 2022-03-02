const socket = new Promise((resolve, reject) => {
    const ws = new WebSocket("wss://bajsballe.com:443");
    // const ws = new WebSocket("wss://irc-ws.chat.twitch.tv:443");
    ws.addEventListener("close", (e) => {
        console.log("close", e);
        reject("rip it closed");
    });
    ws.addEventListener("open", (e) => {
        console.log("open", e);
        resolve(ws);
    });
});
console.log(WebSocket.CLOSED);
console.log(WebSocket.CLOSING);
console.log(WebSocket.CONNECTING);
console.log(WebSocket.OPEN);

socket
    .then((d) => {
        console.log("it resolved", d);
    })
    .catch((e) => {
        console.log("it rejected", e);
    });
