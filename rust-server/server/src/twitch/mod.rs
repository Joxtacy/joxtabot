use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use warp::http::{HeaderMap, HeaderValue};

use crate::websocket::client_utils::TwitchCommand;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationChallenge {
    pub challenge: String,
    subscription: Subscription,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RevokedSubscription {
    pub subscription: Subscription,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    broadcaster_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transport {
    method: String,
    callback: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    condition: Condition,
    cost: usize,
    created_at: String,
    id: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    message_type: String, // discriminator
    pub status: String,
    transport: Transport,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    broadcaster_user_id: String,
    broadcaster_user_login: String,
    broadcaster_user_name: String,
    #[serde(rename(deserialize = "type"), default)]
    event_type: String,
    id: String,
    #[serde(default)]
    redeemed_at: String,
    #[serde(default)]
    reward: Reward,
    #[serde(default)]
    started_at: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    user_id: String,
    #[serde(default)]
    user_input: String,
    #[serde(default)]
    user_login: String,
    #[serde(default)]
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchMessage {
    subscription: Subscription,
    event: Event,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Reward {
    id: String,
    title: String,
    cost: usize,
    prompt: String,
}

/// The value of the `Twitch-Eventsub-Message-Type` header
/// when receiving a notification
pub const NOTIFICATION_TYPE: &str = "notification";
/// The value of the `Twitch-Eventsub-Message-Type` header
/// when new webhook subscription is created
pub const WEBHOOK_CALLBACK_VERIFICATION_TYPE: &str = "webhook_callback_verification";
/// The value of the `Twitch-Eventsub-Message-Type` header
/// when the webhook has been revoked
pub const SUBSCRIPTION_REVOKED_TYPE: &str = "revocation";

/// Handles the webhook message
pub fn handle_webhook_message(message: TwitchMessage) -> TwitchCommand {
    let message_type = message.subscription.message_type;

    match &message_type[..] {
        "stream.online" => TwitchCommand::StreamOnline,
        "channel.channel_points_custom_reward_redemption.add" => {
            let reward_title = message.event.reward.title;

            match &reward_title[..] {
                "First" => TwitchCommand::First(message.event.user_name),
                "Timeout" => {
                    let user_name = message.event.user_name;
                    TwitchCommand::Timeout {
                        timeout: 120,
                        user: user_name,
                    }
                }
                "-420" => TwitchCommand::FourTwenty,
                "ded" => TwitchCommand::Ded,
                "Nice" => TwitchCommand::Nice,
                "+1 Pushup" => TwitchCommand::Pushup(1),
                "+1 Situp" => TwitchCommand::Situp(1),
                "Emote-only Chat" => TwitchCommand::EmoteOnly,
                _ => {
                    println!("[TWITCH] Reward not supported: {}", reward_title);
                    TwitchCommand::UnsupportedMessage
                }
            }
        }
        _ => {
            println!("Unknown message type: {}", message_type);
            TwitchCommand::UnsupportedMessage
        }
    }
}

pub fn parse_twitch_request_header(header: Option<&HeaderValue>) -> String {
    if let Some(header) = header {
        header.to_str().unwrap_or("").to_owned()
    } else {
        "".to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TwitchTimestampError {
    /// Timestamp is too old.
    TooOld,
    /// Timestamp could not be parsed as it was not a valid timestamp.
    NotAValidTimestamp,
}

/// Verifies that the timestamp header value is not too old.
///
/// Current limit is 10 minutes.
pub fn verify_twitch_message_age(
    timestamp_header: Option<&HeaderValue>,
) -> Result<(), TwitchTimestampError> {
    let twitch_message_timestamp = parse_twitch_request_header(timestamp_header);

    let timestamp = chrono::DateTime::parse_from_rfc3339(&twitch_message_timestamp);

    if timestamp.is_err() {
        return Err(TwitchTimestampError::NotAValidTimestamp);
    }

    let timestamp = timestamp.expect("This is now `Ok` type");

    let now = chrono::Utc::now();
    let old_message_duration = chrono::Duration::minutes(10);

    if timestamp + old_message_duration < now {
        return Err(TwitchTimestampError::TooOld);
    }

    Ok(())
}

pub fn verify_twitch_message(headers: &HeaderMap, body: &str) -> bool {
    let twitch_message_id = headers.get("Twitch-Eventsub-Message-Id");
    let twitch_message_timestamp = headers.get("Twitch-Eventsub-Message-Timestamp");
    let twitch_message_signature = headers.get("Twitch-Eventsub-Message-Signature");

    let twitch_message_id = parse_twitch_request_header(twitch_message_id);
    let twitch_message_timestamp = parse_twitch_request_header(twitch_message_timestamp);
    let twitch_message_signature = parse_twitch_request_header(twitch_message_signature);

    let secret = "bajsballetelefonlur";
    let hmac_message = format!("{}{}{}", twitch_message_id, twitch_message_timestamp, body);

    type HmacSha256 = Hmac<Sha256>;

    let hmac_prefix = "sha256="; // Twitch signature starts with `sha256=`
    let split_strings = twitch_message_signature
        .split(hmac_prefix)
        .into_iter()
        .collect::<Vec<&str>>();

    // If split fails, that means it is not a valid signature
    if split_strings.len() < 2 {
        return false;
    }

    let decoded = hex::decode(split_strings[1]).unwrap_or_default();

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(hmac_message.as_bytes());

    mac.verify_slice(&decoded).is_ok()
}

#[cfg(test)]
mod tests {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use warp::http::{HeaderMap, HeaderValue};

    use super::{
        parse_twitch_request_header, verify_twitch_message, verify_twitch_message_age,
        TwitchTimestampError,
    };

    #[test]
    fn it_should_verify_twitch_message() {
        let now = chrono::Utc::now();

        let timestamp = now.to_rfc3339();
        let message_id = "message-id";
        let body = r#"{"subscription":{"id":"cfe495bf-a78e-6c47-2e66-f3ff62398c31","status":"enabled","type":"channel.channel_points_custom_reward_redemption.add","version":"1","condition":{"broadcaster_user_id":"98048478"},"transport":{"method":"webhook","callback":"null"},"created_at":"2022-10-22T02:52:54.58609Z","cost":0},"event":{"id":"cfe495bf-a78e-6c47-2e66-f3ff62398c31","broadcaster_user_id":"98048478","broadcaster_user_login":"testBroadcaster","broadcaster_user_name":"testBroadcaster","user_id":"73700748","user_login":"testFromUser","user_name":"testFromUser","user_input":"Test Input From CLI","status":"unfulfilled","reward":{"id":"923154d2-65f1-cc5d-7e5f-d131036daaa7","title":"Test Reward from CLI","cost":150,"prompt":"Redeem Your Test Reward from CLI"},"redeemed_at":"2022-10-22T02:52:54.58609Z"}}"#;

        let timestamp_header = HeaderValue::from_str(&timestamp).unwrap();
        let message_id_header = HeaderValue::from_str("message-id").unwrap();

        let hmac_message = format!("{}{}{}", message_id, &timestamp, body);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice("bajsballetelefonlur".as_bytes()).unwrap();
        mac.update(hmac_message.as_bytes());

        let bytes = mac.finalize().into_bytes();

        let signature = hex::encode(bytes);
        let signature = format!("sha256={}", signature);

        let signature_header = HeaderValue::from_str(&signature).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Twitch-Eventsub-Message-Id", message_id_header);
        headers.insert("Twitch-Eventsub-Message-Timestamp", timestamp_header);
        headers.insert("Twitch-Eventsub-Message-Signature", signature_header);

        let result = verify_twitch_message(&headers, body);

        assert!(result);
    }

    #[test]
    fn it_should_verify_the_age_of_the_message() {
        let now = chrono::Utc::now();
        let header_value = HeaderValue::from_str(&now.to_rfc3339()).unwrap();
        let res = verify_twitch_message_age(Some(&header_value));

        assert!(res.is_ok());
    }

    #[test]
    fn it_should_verify_that_the_message_is_too_old() {
        let now = chrono::Utc::now();
        let duration = chrono::Duration::minutes(20);
        let new_time = now - duration;
        let header_value = HeaderValue::from_str(&new_time.to_rfc3339()).unwrap();
        let res = verify_twitch_message_age(Some(&header_value));

        assert!(res.is_err());

        assert_eq!(res.unwrap_err(), TwitchTimestampError::TooOld);
    }

    #[test]
    fn it_should_verify_that_the_timestamp_is_not_valid() {
        let header_value = HeaderValue::from_str("Herp derp").unwrap();
        let res = verify_twitch_message_age(Some(&header_value));

        assert!(res.is_err());

        assert_eq!(res.unwrap_err(), TwitchTimestampError::NotAValidTimestamp);
    }
    #[test]
    fn it_should_parse_twitch_request_header_to_empty_string() {
        let result = parse_twitch_request_header(None);

        assert_eq!(result, "".to_string());
    }

    #[test]
    fn it_should_parse_twitch_request_header_to_string() {
        let header_value = HeaderValue::from_str("This is a valid header").unwrap();

        let result = parse_twitch_request_header(Some(&header_value));

        assert_eq!(result, "This is a valid header".to_string());
    }
}
