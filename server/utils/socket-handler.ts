type WebSocketMessage = string | ArrayBufferLike | Blob | ArrayBufferView;

class SocketHandler {
  private sockets: Set<WebSocket> = new Set();

  register(socket: WebSocket) {
    this.sockets.add(socket);
  }

  unregister(socket: WebSocket) {
    this.sockets.delete(socket);
  }

  sendAll(message: WebSocketMessage) {
    this.sockets.forEach((socket) => socket.send(message));
  }
}

const socketHandler = new SocketHandler();

export default socketHandler;
