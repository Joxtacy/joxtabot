use std::env;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use tokio::time;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    // construct a subscriber that prints formatted traces to stdout
    // global subscriber with log level according to RUST_LOG
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();
    let rabbit_host = env::var("RABBIT_HOST").unwrap_or("localhost".to_string());

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        String::from("joxtabot"),
        Some(String::from("thpg9474xi3g4sj6ograapccgfzdhz")),
    ));
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // open a connection to RabbitMQ server
    let connection = Connection::open(&OpenConnectionArguments::new(
        &rabbit_host,
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    // declare a durable queue
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named("chat"))
        .await
        .unwrap()
        .unwrap();

    let channel_clone = channel.clone();

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
            match message {
                ServerMessage::Privmsg(msg) => {
                    channel_clone
                        .basic_publish(
                            BasicProperties::default(),
                            format!("{}: {}", msg.sender.name, msg.message_text).into_bytes(),
                            BasicPublishArguments::new("", &queue_name),
                        )
                        .await
                        .unwrap();
                }
                _ => {}
            }
        }
    });
    client.join("joxtacy".to_string()).unwrap();
    client
        .say("joxtacy".to_string(), "henlo".to_string())
        .await
        .unwrap();

    // keep the `channel` and `connection` object from dropping before pub/sub is done.
    // channel/connection will be closed when drop
    time::sleep(time::Duration::from_secs(1)).await;

    // wait for join handle
    join_handle.await.unwrap();

    // explicitly close
    channel.close().await.unwrap();
    connection.close().await.unwrap();
}
