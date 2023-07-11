use log::{debug, error, info, warn};
use reqwest::{Client, StatusCode};
use serde::Serialize;

const TARGET: &str = "DISCORD_UTILS";

pub struct Discord {
    /// Bearer token used for authentication of the Discord API
    token: String,
    /// Reqwest client
    client: reqwest::Client,
}

impl Discord {
    const BASE_URL: &str = "https://discord.com/api/v10";

    /// Creates a new Discord instance
    pub fn new(token: &str, client: Client) -> Self {
        Self {
            token: token.to_string(),
            client,
        }
    }

    /// Creates a new message in the provided `channel_id`.
    ///
    /// Reference: https://discord.com/developers/docs/resources/channel#create-message
    pub async fn create_message(&self, channel_id: u64, message: &str) -> Result<String, String> {
        info!(
            target: TARGET,
            "Creating new message in channel: {}", channel_id
        );
        let url = format!("{}/channels/{}/messages", Discord::BASE_URL, channel_id);
        // let url = "https://example.com/".to_string();

        #[derive(Debug, Serialize)]
        struct CreateMessage {
            content: String,
        }
        let data = CreateMessage {
            content: message.to_string(),
        };

        debug!(
            target: TARGET,
            "Sending request to create new message in channel: {}", channel_id
        );
        let resp = self
            .client
            .post(url)
            .header("Authorization", format!("Bot {}", self.token))
            .json(&data)
            .send()
            .await;

        match resp {
            Ok(response) => {
                let status_code = response.status();
                if StatusCode::is_success(&status_code) {
                    debug!(target: TARGET, "Successfully created message");
                    Ok("Message created successfully".to_string())
                } else {
                    let response_text = response.text().await.unwrap_or_else(|err| err.to_string());
                    warn!(
                        target: TARGET,
                        "Failed creating message. Reason: {}", response_text
                    );
                    Err(format!(
                        "Failed to create message. Reason: {}",
                        response_text
                    ))
                }
            }
            Err(error) => {
                error!(
                    target: TARGET,
                    "Error when sending request. Reason: {:?}", error
                );
                Err(format!("Failed to create message. Reason: {:?}", error))
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//
//     use crate::Discord;
//
//     #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
//     async fn request_success() {
//         let mut server = mockito::Server::new();
//         let target = server.url();
//         let channel_id = 69;
//
//         let mock = server
//             .mock("POST", "/")
//             .with_status(200)
//             .with_body("world")
//             .create();
//
//         println!("target: {}", target);
//         let client = reqwest::Client::builder()
//             .proxy(reqwest::Proxy::custom(move |url| {
//                 println!("url: {}", url.path());
//                 println!("url: {}", url);
//                 println!("url: {:?}", url.host_str());
//                 Some(target.clone())
//             }))
//             // .proxy(reqwest::Proxy::https(url).unwrap())
//             .build()
//             .unwrap();
//
//         let discord = Discord::new("token", client);
//
//         let res = discord.create_message(channel_id, "hello").await;
//
//         assert_eq!(res.is_ok(), true);
//
//         mock.assert();
//     }
// }
