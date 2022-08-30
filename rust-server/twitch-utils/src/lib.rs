use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub id: u64,
    pub user_id: u64,
    pub user_login: String,
    pub user_name: String,
    pub game_id: u64,
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
            let status_code = response.status();
            if StatusCode::is_success(&status_code) {
                // println!("RESULT: {}", response.text().await.unwrap());

                let res = response.json::<StreamInfo>().await;
                match res {
                    Ok(stream_info) => Ok(stream_info),
                    Err(e) => Err(format!("Failed to get stream info. Reason: {:?}", e)),
                }
            } else {
                Err(format!(
                    "Failed to get stream info. Reason: {:?}",
                    response.text().await
                ))
            }
        }
        Err(error) => Err(format!("Failed to get stream info. Reason: {:?}", error)),
    }
}
