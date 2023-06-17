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
    url: String,
    token: String,
    client_id: String,
    user_id: u64,
) -> Result<StreamInfo, String> {
    info!(target: "TWITCH_UTILS", "Getting stream info for user: {}", user_id);
    let url = format!("{}?user_id={}", url, user_id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_stream_info_success() {
        // Set up mock server
        let mock_token = "mock_token";
        let mock_client_id = "mock_client_id";
        let mock_user_id = 123;
        let mock_response_body = json!({
            "data": [
                {
                    "id": "123456",
                    "user_id": "123",
                    "user_login": "mock_user_login",
                    "user_name": "mock_user_name",
                    "game_id": "456",
                    "game_name": "mock_game_name",
                    "type": "live",
                    "title": "mock_title",
                    "viewer_count": 100,
                    "started_at": "2022-01-01T00:00:00Z",
                    "language": "en",
                    "thumbnail_url": "mock_thumbnail_url",
                    "tag_ids": ["789"],
                    "is_mature": false
                }
            ]
        });
        let mut server = Server::new();
        let mock_url = server.url();
        let mock_server = server
            .mock("GET", "/?user_id=123")
            .match_header("Authorization", format!("Bearer {}", mock_token).as_str())
            .match_header("Client-Id", mock_client_id)
            .with_status(200)
            .with_body(mock_response_body.to_string())
            .create();

        // Call function
        let stream_info = get_stream_info(
            mock_url,
            mock_token.to_string(),
            mock_client_id.to_string(),
            mock_user_id,
        )
        .await;

        // Assert
        assert!(stream_info.is_ok());
        let stream_info = stream_info.unwrap();
        assert_eq!(stream_info.data.len(), 1);
        let stream_data = &stream_info.data[0];
        assert_eq!(stream_data.id, "123456");
        assert_eq!(stream_data.user_id, "123");
        assert_eq!(stream_data.user_login, "mock_user_login");
        assert_eq!(stream_data.user_name, "mock_user_name");
        assert_eq!(stream_data.game_id, "456");
        assert_eq!(stream_data.game_name, "mock_game_name");
        assert_eq!(stream_data.stream_type, "live");
        assert_eq!(stream_data.title, "mock_title");
        assert_eq!(stream_data.viewer_count, 100);
        assert_eq!(stream_data.started_at, "2022-01-01T00:00:00Z");
        assert_eq!(stream_data.language, "en");
        assert_eq!(stream_data.thumbnail_url, "mock_thumbnail_url");
        assert_eq!(stream_data.tag_ids, vec!["789"]);
        assert_eq!(stream_data.is_mature, false);

        // Clean up
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_stream_info_failure() {
        // Set up mock server
        let mock_token = "mock_token";
        let mock_client_id = "mock_client_id";
        let mock_user_id = 123;
        let mut server = Server::new();
        let mock_url = server.url();

        let mock_server = server
            .mock("GET", "/?user_id=123")
            .match_header("Authorization", format!("Bearer {}", mock_token).as_str())
            .match_header("Client-Id", mock_client_id)
            .with_status(404)
            .with_body("Not Found (status: 404): Stream not found")
            .create();

        // Call function
        let stream_info = get_stream_info(
            mock_url,
            mock_token.to_string(),
            mock_client_id.to_string(),
            mock_user_id,
        )
        .await;

        // Assert
        assert!(stream_info.is_err());
        let err_msg = stream_info.err().unwrap();
        assert_eq!(
            err_msg,
            "Failed to get stream info. Reason: Not Found (status: 404): Stream not found"
        );

        // Clean up
        mock_server.assert();
    }
}
