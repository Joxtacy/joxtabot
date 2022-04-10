class SocketHandler {
  private sockets: Set<WebSocket> = new Set();

  register(socket: WebSocket) {
    this.sockets.add(socket);
  }

  unregister(socket: WebSocket) {
    this.sockets.delete(socket);
  }

  sendAll(message: string | ArrayBufferLike | Blob | ArrayBufferView) {
    this.sockets.forEach((socket) => socket.send(message));
  }
}

const socketHandler = new SocketHandler();

export default socketHandler;
