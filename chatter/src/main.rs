use std::env;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use async_trait::async_trait;
use sqlx::{prelude::FromRow, PgPool};
use tokio::time;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use twitch_irc::{
    login::{RefreshingLoginCredentials, TokenStorage, UserAccessToken},
    message::ServerMessage,
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

#[derive(Debug, FromRow)]
struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug)]
struct PostgresTokenStorage {
    pool: PgPool,
}

#[async_trait]
impl TokenStorage for PostgresTokenStorage {
    type LoadError = std::io::Error; // or some other error
    type UpdateError = std::io::Error;

    async fn load_token(&mut self) -> Result<UserAccessToken, Self::LoadError> {
        // Load the currently stored token from storage
        let rec: Token = sqlx::query_as!(
            Token,
            r#"
SELECT access_token, refresh_token, created_at, expires_at FROM tokens
WHERE name = 'twitch_chat';
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let token = UserAccessToken {
            access_token: rec.access_token,
            refresh_token: rec.refresh_token,
            created_at: rec.created_at,
            expires_at: rec.expires_at,
        };

        Ok(token)
    }

    async fn update_token(&mut self, token: &UserAccessToken) -> Result<(), Self::UpdateError> {
        // Called after the token was updated successfully, to save the new token.
        // After `update_token()` completes, the `load_token()` method should then return
        // that token for future invocations

        sqlx::query!(
            r#"
UPDATE tokens SET access_token = $1, refresh_token = $2, created_at = $3, expires_at = $4
WHERE "name" = 'twitch_chat'
RETURNING access_token, refresh_token, created_at, expires_at;
            "#,
            &token.access_token,
            &token.refresh_token,
            &token.created_at,
            token.expires_at
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

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

    let pool = PgPool::connect(&env::var("POSTGRES_URL")?).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let token_storage = PostgresTokenStorage { pool };

    let rabbit_host = env::var("RABBIT_HOST").unwrap_or("localhost".to_string());
    let client_id = env::var("CLIENT_ID").unwrap_or("".to_string());
    let client_secret = env::var("CLIENT_SECRET").unwrap_or("".to_string());

    let credentials = RefreshingLoginCredentials::init(client_id, client_secret, token_storage);
    let config = ClientConfig::new_simple(credentials);
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<PostgresTokenStorage>,
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
