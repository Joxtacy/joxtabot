export class SocketServer {
  constructor() {}

  openSocket(request: Request) {
    const { socket, response } = Deno.upgradeWebSocket(request);

    socket.addEventListener("error", (error) => {
      console.error("[SOCKET SERVER] Connection errored", error);
    });

    return response;
  }
}
