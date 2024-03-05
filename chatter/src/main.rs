use std::{env, fs};

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use async_trait::async_trait;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tokio::time;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use twitch_irc::{
    login::{RefreshingLoginCredentials, TokenStorage, UserAccessToken},
    message::ServerMessage,
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

#[derive(Debug, Serialize, Deserialize)]
struct CustomTokenStorage {
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[async_trait]
impl TokenStorage for CustomTokenStorage {
    type LoadError = std::io::Error; // or some other error
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        // Load the currently stored token from storage
        let contents = fs::read_to_string("token.json")?;
        let storage = serde_json::from_str::<CustomTokenStorage>(&contents)?;
        let expires_at = match &storage.expires_at {
            Some(expires_at) => {
                let expires_at = DateTime::parse_from_str(expires_at, "%+");
                let expires_at = expires_at.unwrap().to_utc();

                Some(expires_at)
            }
            None => None,
        };

        let created_at = DateTime::parse_from_str(&storage.created_at, "%+");
        let created_at = created_at.unwrap().to_utc();
        let token = UserAccessToken {
            access_token: storage.access_token.clone(),
            refresh_token: storage.refresh_token.clone(),
            created_at,
            expires_at,
        };

        Ok(token)
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        // Called after the token was updated successfully, to save the new token.
        // After `update_token()` completes, the `load_token()` method should then return
        // that token for future invocations
        let contents = serde_json::to_string(&token)?;
        fs::write("token.json", contents)?;

        Ok(())
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // construct a subscriber that prints formatted traces to stdout
    // global subscriber with log level according to RUST_LOG
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let rabbit_host = env::var("RABBIT_HOST").unwrap_or("localhost".to_string());
    let client_id = env::var("CLIENT_ID").unwrap_or("".to_string());
    let client_secret = env::var("CLIENT_SECRET").unwrap_or("".to_string());
    let storage = fs::read_to_string("token.json").unwrap();
    let storage = serde_json::from_str::<CustomTokenStorage>(&storage).unwrap();

    let credentials = RefreshingLoginCredentials::init(client_id, client_secret, storage);
    let config = ClientConfig::new_simple(credentials);
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<CustomTokenStorage>,
    >::new(config);

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
        .say("joxtacy".to_string(), "Hello, frens! joxtacHi".to_string())
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
    Ok(())
}
