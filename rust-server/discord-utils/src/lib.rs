use reqwest::StatusCode;
use serde::Serialize;

/// Creates a new message in the provided `channel_id`.
///
/// Reference: https://discord.com/developers/docs/resources/channel#create-message
pub async fn create_message(
    token: String,
    channel_id: u64,
    message: String,
) -> Result<String, String> {
    let url = format!(
        "https://discord.com/api/v10/channels/{}/messages",
        channel_id
    );
    let client = reqwest::Client::new();

    #[derive(Debug, Serialize)]
    struct CreateMessage {
        content: String,
    }
    let data = CreateMessage { content: message };
    let resp = client
        .post(url)
        .header("Authorization", format!("Bot {}", token))
        .json(&data)
        .send()
        .await;

    match resp {
        Ok(response) => {
            let status_code = response.status();
            if StatusCode::is_success(&status_code) {
                Ok("Message created successfully".to_string())
            } else {
                Err(format!(
                    "Failed to create message. Reason: {:?}",
                    response.text().await
                ))
            }
        }
        Err(error) => Err(format!("Failed to create message. Reason: {:?}", error)),
    }
}
