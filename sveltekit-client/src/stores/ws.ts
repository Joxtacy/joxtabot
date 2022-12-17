import { writable } from "svelte/store";

const store = writable({ messages: [] });

export const connect = (url = "ws://localhost:8000/ws") => {
  const ws = new WebSocket(url);

  let interval: NodeJS.Timer;
  ws.addEventListener("open", () => {
    console.log("[WS] Socket connected");
    interval = setInterval(() => {
      ws.send("[PING]");
    }, 30_000);
  });

  ws.addEventListener("message", ({ data }) => {
    store.update((state) => ({
      ...state,
      messages: [data, ...state.messages],
    }));
  });

  ws.addEventListener("close", () => {
    // TODO: Handle close
    clearInterval(interval);
  });

  ws.addEventListener("error", () => {
    // TODO: Handle error
  });
};

export default store;
