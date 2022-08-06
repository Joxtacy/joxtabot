use actix_web::{
    http::{StatusCode, header::HeaderValue},
    post,
    web::Bytes,
    HttpRequest, HttpResponseBuilder, HttpResponse,
};
use hmac::{Mac, Hmac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use hex;

// Redeem reward example.
// {
//     "subscription": {
//         "id": "62a04e8f-5c47-acfa-6895-274d679abf85",
//&         "status": "enabled",
//         "type": "channel.channel_points_custom_reward_redemption.add",
//         "version": "1",
//         "condition": {
//             "broadcaster_user_id": "54605357"
//         },
//         "transport": {
//             "method": "webhook",
//             "callback": "null"
//         },
//         "created_at": "2022-07-13T04:03:22.577859Z",
//         "cost": 0
//     },
//     "event": {
//         "id": "62a04e8f-5c47-acfa-6895-274d679abf85",
//         "broadcaster_user_id": "54605357",
//         "broadcaster_user_login": "testBroadcaster",
//         "broadcaster_user_name": "testBroadcaster",
//         "user_id": "91240126",
//         "user_login": "testFromUser",
//         "user_name": "testFromUser",
//         "user_input": "Test Input From CLI",
//         "status": "unfulfilled",
//         "reward": {
//             "id": "c3f5c149-65e9-4bc0-2271-723f41731542",
//             "title": "Test Reward from CLI",
//             "cost": 150,
//             "prompt": "Redeem Your Test Reward from CLI"
//         },
//         "redeemed_at": "2022-07-13T04:03:22.577859Z"
//     }
// }

// Stream up example
// {
//     "subscription": {
//         "id": "8840d0b3-8488-f599-3f18-fc273347a6d3",
//         "status": "enabled",
//         "type": "stream.online",
//         "version": "1",
//         "condition": {
//             "broadcaster_user_id": "54605357"
//         },
//         "transport": {
//             "method": "webhook",
//             "callback": "null"
//         },
//         "created_at": "2022-07-13T04:05:05.579848Z",
//         "cost": 0
//     },
//     "event": {
//         "id": "4878643",
//         "broadcaster_user_id": "54605357",
//         "broadcaster_user_login": "testBroadcaster",
//         "broadcaster_user_name": "testBroadcaster",
//         "type": "live",
//         "started_at": "2022-07-13T04:05:05.579868Z"
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
struct Condition {
    broadcaster_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transport {
    method: String,
    callback: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Subscription {
    condition: Condition,
    cost: usize,
    created_at: String,
    id: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    message_type: String, // discriminator
    status: String,
    transport: Transport,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
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

#[post("/twitch/webhooks/callback")]
pub async fn twitch_webhook(req: HttpRequest, bytes: Bytes) -> HttpResponse {
// pub async fn twitch_webhook(req: HttpRequest, item: Json<TwitchMessage>) -> HttpResponse {
    let body = String::from_utf8(bytes.to_vec()).unwrap();

    let verified = verify_twitch_message(&req, &body);
    if !verified {
        return HttpResponseBuilder::new(StatusCode::NOT_ACCEPTABLE).finish();
    }

    let message = serde_json::from_str::<TwitchMessage>(&body).unwrap();

    // let message = item.into_inner();
    // println!("REQUEST! {:?}", req);

    // let headers = req.headers();
    // let twitch_message_type_header = headers.get("Twitch-Eventsub-Message-Type");
    // let twitch_message_id = headers.get("Twitch-Eventsub-Message-Id");
    // let twitch_message_timestamp = headers.get("Twitch-Eventsub-Message-Timestamp");
    // let twitch_message_signature = headers.get("Twitch-Eventsub-Message-Signature");
    // let twitch_subscription_type = headers.get("Twitch-Eventsub-Subscription-Type");
    // let twitch_subscription_version = headers.get("Twitch-Eventsub-Subscription-Version");
    // let twitch_message_retry = headers.get("Twitch-Eventsub-Message-Retry");

    // println!("subscription: {:?}", message.subscription);
    // println!("event: {:?}", message.event);

    HttpResponseBuilder::new(StatusCode::OK).json("I got you, fam".to_owned())
}


fn parse_header(header: Option<&HeaderValue>) -> String {
    if let Some(header) = header {
        header.to_str().unwrap_or("").to_owned()
    } else {
        "".to_owned()
    }
}

fn verify_twitch_message(req: &HttpRequest, body: &str) -> bool {
    let headers = req.headers();
    let twitch_message_id = headers.get("Twitch-Eventsub-Message-Id");
    let twitch_message_timestamp = headers.get("Twitch-Eventsub-Message-Timestamp");
    let twitch_message_signature = headers.get("Twitch-Eventsub-Message-Signature");

    let twitch_message_id = parse_header(twitch_message_id);
    let twitch_message_timestamp = parse_header(twitch_message_timestamp);
    let twitch_message_signature = parse_header(twitch_message_signature);

    let secret = "bajsballetelefonlur";
    let hmac_message = format!("{}{}{}", twitch_message_id, twitch_message_timestamp, body);

    type HmacSha256 = Hmac<Sha256>;

    let hmac_prefix = "sha256="; // Twitch signature starts with `sha256=`
    let split_strings = twitch_message_signature.split(hmac_prefix).into_iter().collect::<Vec<&str>>();

    // If split fails, that means it is not a valid signature
    if split_strings.len() < 2 {
        return false;
    }

    let decoded = hex::decode(split_strings[1]).unwrap_or_default();

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(hmac_message.as_bytes());

    mac.verify_slice(&decoded).is_ok()
}
