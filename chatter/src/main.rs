use std::{collections::BTreeMap, env};

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use async_trait::async_trait;
use fred::{
    interfaces::{ClientLike, KeysInterface, RedisJsonInterface},
    types::{Builder, RedisConfig, SetOptions},
    util::NONE,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Badge {
    name: String,
    version: String,
    icon_url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Emote {
    id: String,
    char_range: (usize, usize),
    code: String,
    url_template: String,
    format: String,
}

#[derive(Debug, Serialize)]
struct RabbitMessage {
    message: String,
    sender: String,
    color: Option<String>,
    badges: Vec<Badge>,
    emotes: Vec<Emote>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwitchBadgeResponse {
    data: Vec<TwitchBadge>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwitchBadge {
    set_id: String,
    versions: Vec<TwitchBadgeVersion>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwitchBadgeVersion {
    id: String,
    image_url_1x: String,
    image_url_2x: String,
    image_url_4x: String,
    title: String,
    description: String,
    click_action: Option<String>,
    click_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwitchEmoteResponse {
    data: Vec<TwitchEmote>,
    template: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwitchEmote {
    id: String,
    name: String,
    images: TwitchImage,
    format: Vec<String>,
    scale: Vec<String>,
    theme_mode: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TwitchImage {
    url_1x: String,
    url_2x: String,
    url_4x: String,
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

    let postgres_pool = PgPool::connect(&env::var("POSTGRES_URL")?).await?;
    sqlx::migrate!("./migrations").run(&postgres_pool).await?;

    let redis_host = env::var("REDIS_URL")?;
    let redis_config = RedisConfig::from_url(&redis_host)?;
    let redis_pool = Builder::from_config(redis_config).build_pool(5)?;
    redis_pool.init().await?;

    struct AccessTokenResponse {
        access_token: String,
    }
    tracing::info!("Fetching access token");
    let access_token = sqlx::query_as!(
        AccessTokenResponse,
        r#"
SELECT access_token FROM tokens
WHERE name = 'twitch_chat';
            "#,
    )
    .fetch_one(&postgres_pool)
    .await
    .unwrap();

    let token_storage = PostgresTokenStorage {
        pool: postgres_pool,
    };

    let rabbit_host = env::var("RABBIT_HOST").unwrap_or("localhost".to_string());
    let client_id = env::var("CLIENT_ID").unwrap_or("".to_string());
    let client_secret = env::var("CLIENT_SECRET").unwrap_or("".to_string());

    tracing::info!("Create Twitch IRC client");
    let credentials =
        RefreshingLoginCredentials::init(client_id.clone(), client_secret, token_storage);
    let config = ClientConfig::new_simple(credentials);
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        RefreshingLoginCredentials<PostgresTokenStorage>,
    >::new(config);

    let global_badges: Option<Value> = redis_pool
        .json_get("global_badges", NONE, NONE, NONE, "$")
        .await
        .unwrap_or(None);

    let reqwest_client = reqwest::Client::new();
    if let None = global_badges {
        tracing::info!("Sending request to get global badges");
        let response = reqwest_client
            .get("https://api.twitch.tv/helix/chat/badges/global")
            .bearer_auth(&access_token.access_token)
            .header("Client-Id", &client_id)
            .send()
            .await
            .unwrap()
            .json::<TwitchBadgeResponse>()
            .await
            .unwrap();

        let _: () = redis_pool
            .json_set(
                "global_badges",
                "$",
                serde_json::to_string(&response.data).unwrap(),
                Some(SetOptions::NX),
            )
            .await
            .unwrap();
        // to set expire on a key
        // let _: () = redis_pool.expire("global_badges", 300).await.unwrap();
    }

    let channel_badges: Option<Value> = redis_pool
        .json_get("channel_badges", NONE, NONE, NONE, "$")
        .await
        .unwrap_or(None);

    if let None = channel_badges {
        tracing::info!("Sending request to get channel badges");
        let response = reqwest_client
            .get("https://api.twitch.tv/helix/chat/badges")
            .query(&[("broadcaster_id", "54605357")])
            .bearer_auth(&access_token.access_token)
            .header("Client-Id", &client_id)
            .send()
            .await
            .unwrap()
            .json::<TwitchBadgeResponse>()
            .await
            .unwrap();

        let _: () = redis_pool
            .json_set(
                "channel_badges",
                "$",
                serde_json::to_string(&response.data).unwrap(),
                Some(SetOptions::NX),
            )
            .await
            .unwrap();
        // to set expire on a key
        // let _: () = redis_pool.expire("channel_badges", 300).await.unwrap();
    }

    let global_emotes: Option<Value> = redis_pool
        .json_get("global_emotes", NONE, NONE, NONE, "$")
        .await
        .unwrap_or(None);

    if let None = global_emotes {
        let response = reqwest_client
            .get("https://api.twitch.tv/helix/chat/emotes/global")
            .bearer_auth(&access_token.access_token)
            .header("Client-Id", &client_id)
            .send()
            .await
            .unwrap()
            .json::<TwitchEmoteResponse>()
            .await
            .unwrap();

        let _: () = redis_pool
            .set("emote_template", response.template, None, None, false)
            .await
            .unwrap();

        let _: () = redis_pool
            .json_set(
                "global_emotes",
                "$",
                serde_json::to_string(&response.data).unwrap(),
                Some(SetOptions::NX),
            )
            .await
            .unwrap();
        // to set expire on a key
        // let _: () = redis_pool.expire("global_emotes", 300).await.unwrap();
    }

    let channel_emotes: Option<Value> = redis_pool
        .json_get("channel_emotes", NONE, NONE, NONE, "$")
        .await
        .unwrap_or(None);

    if let None = channel_emotes {
        let response = reqwest_client
            .get("https://api.twitch.tv/helix/chat/emotes")
            .query(&[("broadcaster_id", "54605357")])
            .bearer_auth(&access_token.access_token)
            .header("Client-Id", &client_id)
            .send()
            .await
            .unwrap()
            .json::<TwitchEmoteResponse>()
            .await
            .unwrap();

        let _: () = redis_pool
            .set("emote_template", response.template, None, None, false)
            .await
            .unwrap();

        let _: () = redis_pool
            .json_set(
                "channel_emotes",
                "$",
                serde_json::to_string(&response.data).unwrap(),
                Some(SetOptions::NX),
            )
            .await
            .unwrap();
        // to set expire on a key
        // let _: () = redis_pool.expire("channel_emotes", 300).await.unwrap();
    }

    // open a connection to RabbitMQ server
    tracing::info!("Connect to RabbitMQ");
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
            tracing::debug!("Received message: {:?}", message);
            match message {
                ServerMessage::Privmsg(msg) => {
                    let global_badges: Value = redis_pool
                        .json_get("global_badges", NONE, NONE, NONE, "$")
                        .await
                        .unwrap();
                    let global_badges: Vec<Vec<TwitchBadge>> =
                        serde_json::from_value(global_badges).unwrap();

                    let channel_badges: Value = redis_pool
                        .json_get("channel_badges", NONE, NONE, NONE, "$")
                        .await
                        .unwrap();
                    let channel_badges: Vec<Vec<TwitchBadge>> =
                        serde_json::from_value(channel_badges).unwrap();

                    let emote_template: String = redis_pool
                        .get("emote_template")
                        .await
                        .expect("Should have the emote_template set");

                    let global_emotes: Value = redis_pool
                        .json_get("global_emotes", NONE, NONE, NONE, "$")
                        .await
                        .unwrap();
                    let global_emotes: Vec<Vec<TwitchEmote>> =
                        serde_json::from_value(global_emotes).unwrap();

                    let channel_emotes: Value = redis_pool
                        .json_get("channel_emotes", NONE, NONE, NONE, "$")
                        .await
                        .unwrap();
                    let channel_emotes: Vec<Vec<TwitchEmote>> =
                        serde_json::from_value(channel_emotes).unwrap();

                    let all_emotes: Vec<&TwitchEmote> = channel_emotes
                        .iter()
                        .flatten()
                        .chain(global_emotes.iter().flatten())
                        .collect();

                    let my_badges = msg
                        .badges
                        .iter()
                        .map(|badge| (badge.name.clone(), badge))
                        .collect::<BTreeMap<_, _>>();

                    let my_global_badges: Vec<&TwitchBadge> = global_badges
                        .iter()
                        .flatten()
                        .filter(|global_badge| {
                            my_badges
                                .keys()
                                .collect::<Vec<_>>()
                                .contains(&&global_badge.set_id)
                        })
                        .collect();

                    // Maps global subscriber badges to match the correct version of the channel
                    // badge.
                    let mapped_global_badges: Vec<Badge> = my_global_badges
                        .iter()
                        .map(|global_badge| {
                            let my_badge = my_badges
                                .get(&global_badge.set_id)
                                .expect("Should have the badge we're looking for");

                            let badge_version = if global_badge.set_id == "subscriber" {
                                let subscriber_badge = channel_badges
                                    .iter()
                                    .flatten()
                                    .find(|badge| badge.set_id == "subscriber")
                                    .expect("Should have subscriber badge if we made it here");
                                subscriber_badge
                                    .versions
                                    .iter()
                                    .find(|version| version.id == my_badge.version)
                                    .expect("Should have a matching version")
                            } else {
                                global_badge
                                    .versions
                                    .iter()
                                    .find(|version| version.id == my_badge.version)
                                    .expect("Should have a matching version")
                            };
                            Badge {
                                name: global_badge.set_id.clone(),
                                version: badge_version.id.clone(),
                                icon_url: badge_version.image_url_1x.clone(),
                            }
                        })
                        .collect();

                    let emotes: Vec<Emote> = msg
                        .emotes
                        .iter()
                        .map(|message_emote| {
                            let format = if let Some(emote) =
                                all_emotes.iter().find(|emote| emote.id == message_emote.id)
                            {
                                emote
                                    .format
                                    .iter()
                                    .find(|format| *format == "animated")
                                    .unwrap_or(&String::from("static"))
                                    .to_owned()
                            } else {
                                "static".to_owned()
                            };
                            Emote {
                                id: message_emote.id.clone(),
                                char_range: (
                                    message_emote.char_range.start,
                                    message_emote.char_range.end,
                                ),
                                code: message_emote.code.clone(),
                                url_template: emote_template.clone(),
                                format,
                            }
                        })
                        .collect();

                    let message = RabbitMessage {
                        message: msg.message_text,
                        sender: msg.sender.name,
                        color: msg.name_color.map(|c| c.to_string()),
                        badges: mapped_global_badges,
                        emotes,
                    };
                    let message = serde_json::to_string::<RabbitMessage>(&message)
                        .unwrap()
                        .into_bytes();

                    channel_clone
                        .basic_publish(
                            BasicProperties::default(),
                            message,
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
