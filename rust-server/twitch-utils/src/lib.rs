use log::{debug, error, info, warn};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    #[serde(rename = "type")]
    pub stream_type: String,
    pub title: String,
    pub viewer_count: u32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub tag_ids: Vec<String>,
    pub is_mature: bool,
}

#[derive(Deserialize, Debug)]
pub struct StreamInfo {
    pub data: Vec<Data>,
}

/// Fetches the current stream information for the `user_id` provided.
///
/// Reference: `https://dev.twitch.tv/docs/api/reference#get-streams`
pub async fn get_stream_info(
    token: String,
    client_id: String,
    user_id: u64,
) -> Result<StreamInfo, String> {
    info!(target: "TWITCH_UTILS", "Getting stream info for user: {}", user_id);
    let url = format!("https://api.twitch.tv/helix/streams?user_id={}", user_id);
    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .bearer_auth(token)
        .header("Client-Id", client_id)
        .send()
        .await;

    match resp {
        Ok(response) => {
            debug!(target: "TWITCH_UTILS", "Got response: {:?}", response);
            let status_code = response.status();
            if StatusCode::is_success(&status_code) {
                debug!(target: "TWITCH_UTILS", "Got stream info");
                let res = response.json::<StreamInfo>().await;
                match res {
                    Ok(stream_info) => Ok(stream_info),
                    Err(e) => {
                        warn!(target: "TWITCH_UTILS", "Failed parsing response body. Reason: {}", e.to_string());
                        Err(format!("Failed to get stream info. Reason: {:?}", e))
                    }
                }
            } else {
                let response_text = response.text().await.unwrap_or_else(|err| err.to_string());
                warn!(target: "TWITCH_UTILS", "Failed getting stream info. Reason: {}", response_text);
                Err(format!(
                    "Failed to get stream info. Reason: {}",
                    response_text
                ))
            }
        }
        Err(error) => {
            error!(target: "TWITCH_UTILS", "Error when sending request. Reason: {:?}", error);
            Err(format!("Failed to get stream info. Reason: {:?}", error))
        }
    }
}
