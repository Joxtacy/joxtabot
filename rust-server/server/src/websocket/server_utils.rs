use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use warp::ws::WebSocket;

pub async fn client_connected(websocket: WebSocket, mut ws_client_rx: broadcast::Receiver<String>) {
    println!("[WS SERVER] User connected");

    let (mut tx, mut rx) = websocket.split();

    // Some tips might be found here: https://tms-dev-blog.com/build-basic-rust-websocket-server/
    loop {
        tokio::select! {
            msg = rx.next() => {
                match msg {
                    Some(msg) => println!("[WS SERVER] Received message: {:?}", msg),
                    None => {
                        println!("[WS SERVER] User disconnected");
                        break;
                    }
                }
            },
            msg = ws_client_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        println!("[WS SERVER] Received message from broadcast: {}", msg);
                        let res = tx.send(warp::ws::Message::text(msg)).await;
                        if let Err(e) = res {
                            eprintln!(
                                "[WS SERVER] Could not send message to client. Reason: {:?}",
                                e
                                );
                            // If we end up here we exit out of the loop since the
                            // client is no longer connected.
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("[WS SERVER] Error while receiving message on broadcast channel: {:?}", e);
                    }
                }

            },
            else => {
                println!("[WS SERVER] Else branch executed");
            }
        }
    }
}
