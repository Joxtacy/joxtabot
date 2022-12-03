use log::{debug, error, info, warn};
use reqwest::StatusCode;
use serde::Serialize;

const TARGET: &str = "DISCORD_UTILS";

pub struct Discord {
    token: String,
}

impl Discord {
    /// Creates a new message in the provided `channel_id`.
    ///
    /// Reference: https://discord.com/developers/docs/resources/channel#create-message
    pub async fn create_message(&self, channel_id: u64, message: &str) -> Result<String, String> {
        info!(
            target: TARGET,
            "Creating new message in channel: {}", channel_id
        );
        let url = format!(
            "https://discord.com/api/v10/channels/{}/messages",
            channel_id
        );
        let client = reqwest::Client::new();

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
        let resp = client
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
                error!(target: "TWITCH_UTILS", "Error when sending request. Reason: {:?}", error);
                Err(format!("Failed to create message. Reason: {:?}", error))
            }
        }
    }
}

pub struct DiscordBuilder {
    token: String,
}

impl DiscordBuilder {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub fn build(&self) -> Discord {
        Discord {
            token: self.token.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DiscordBuilder;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_builder() {
        let discord = DiscordBuilder::new("token").build();

        assert_eq!("token", discord.token);
    }
}
