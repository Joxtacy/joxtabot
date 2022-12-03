use log::error;
use tokio::sync::broadcast;

pub fn broadcast_message<T>(tx: &broadcast::Sender<T>, msg: T)
where
    T: std::fmt::Debug,
{
    if let Err(e) = tx.send(msg) {
        error!(target: "WS_SERVER", "Could not send message to socket server: {:?}", e);
    }
}
