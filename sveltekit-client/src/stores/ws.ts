import { writable } from "svelte/store";

const store = writable({ messages: [] });

export const connect = (url = "ws://localhost:8000/ws") => {
  const ws = new WebSocket(url);

  ws.addEventListener("open", () => {
    // TODO: Setup ping/pong
    console.log("[WS] Socket connected");
  });

  ws.addEventListener("message", ({ data }) => {
    store.update((state) => ({
      ...state,
      messages: [data, ...state.messages],
    }));
  });

  ws.addEventListener("close", () => {
    // TODO: Handle close
  });

  ws.addEventListener("error", () => {
    // TODO: Handle error
  });
};

export default store;
