use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use tokio::sync::{broadcast, mpsc::Sender};
use warp::ws::WebSocket;

/// Sets up the message handling for a WebSocket connection.
///
/// # Note
///
/// The `_shutdown_complete` is only there to notify to the server that this async function has
/// completed. It does that by simply going out of scope and being dropped.
pub async fn client_connected(
    websocket: WebSocket,
    mut ws_client_rx: broadcast::Receiver<String>,
    mut notify_shutdown: broadcast::Receiver<()>,
    _shutdown_complete: Sender<()>,
) {
    info!(target: "WS_SERVER", "User connected");

    let (mut tx, mut rx) = websocket.split();

    // Some tips might be found here: https://tms-dev-blog.com/build-basic-rust-websocket-server/
    loop {
        tokio::select! {
            msg = rx.next() => {
                match msg {
                    Some(msg) => debug!(target: "WS_SERVER", "Received message: {:?}", msg),
                    None => {
                        debug!(target: "WS_SERVER", "User disconnected");
                        break;
                    }
                }
            },
            msg = ws_client_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        debug!(target: "WS_SERVER", "Received message from broadcast: {}", msg);
                        let res = tx.send(warp::ws::Message::text(msg)).await;
                        if let Err(e) = res {
                            error!(target: "WS_SERVER",
                                "Could not send message to client. Reason: {:?}",
                                e
                                );
                            // If we end up here we exit out of the loop since the
                            // client is no longer connected.
                            break;
                        }
                    },
                    Err(e) => {
                        error!(target: "WS_SERVER", "Error while receiving message on broadcast channel: {:?}", e);
                    }
                }

            },
            _ = notify_shutdown.recv() => {
                // Notification received to shut down the websocket.
                let res = tx.close().await;
                match res {
                    Ok(_) => debug!(target: "WS_SERVER", "Closed Sink Successfully"),
                    Err(err) => error!(target: "WS_SERVER", "Failed to close Sink: {}", err)
                }
                debug!(target: "WS_SERVER", "Closed connection.");
                break;
            }
            else => {
                debug!(target: "WS_SERVER", "Else branch executed");
            }
        }
    }

    info!(target: "WS_SERVER", "Shutting down...");
}
