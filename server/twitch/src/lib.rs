use log::{debug, error, info, warn};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

const TARGET: &str = "TWITCH";

pub struct Twitch {
    /// Bearer token used for authentication of the Discord API
    token: String,
    /// Client ID of your registered bot
    client_id: String,
    /// Reqwest client
    client: reqwest::Client,
}

impl Twitch {
    const BASE_URL: &str = "https://api.twitch.tv/helix";

    #[must_use]
    pub const fn new(token: String, client_id: String, client: Client) -> Self {
        Self {
            token,
            client_id,
            client,
        }
    }

    async fn refresh_token(&mut self) -> Result<(), reqwest::Error> {
        info!(target: TARGET, "Refreshing token");

        let url = "https://id.twitch.tv/oauth2/token";

        let params = [
            ("client_id", &self.client_id),
            (
                "client_secret",
                &std::env::var("TWITCH_CLIENT_SECRET")
                    .expect("Can't refresh without client secret"),
            ),
            ("grant_type", &"client_credentials".to_owned()),
        ];
        let response = self.client.post(url).form(&params).send().await?;

        let credentials = response.json::<Credentials>().await?;

        debug!(
            target: TARGET,
            "New access token: {}", credentials.access_token
        );
        self.token = credentials.access_token;
        Ok(())
    }

    async fn validate_token(&mut self) -> Result<(), reqwest::Error> {
        info!(target: TARGET, "Validating token");

        let url = "https://id.twitch.tv/oauth2/validate";

        let response = self.client.get(url).bearer_auth(&self.token).send().await?;

        match response.status() {
            StatusCode::UNAUTHORIZED => {
                info!(target: TARGET, "Token has expired.");
                self.refresh_token().await?;
                Ok(())
            }
            StatusCode::OK => {
                let validation = response.json::<Validation>().await?;
                debug!(
                    target: TARGET,
                    "Token is valid. Expires in: {} seconds", validation.expires_in
                );
                Ok(())
            }
            status_code => {
                warn!(
                    target: TARGET,
                    "Did not recognise response code: {}",
                    status_code.as_u16()
                );
                Ok(())
            }
        }
    }

    /// Fetches the current stream information for the `user_id` provided.
    ///
    /// Reference: `https://dev.twitch.tv/docs/api/reference#get-streams`
    ///
    /// # Errors
    ///
    /// Returns an error when token is invalid and cannot be refreshed.
    pub async fn get_stream_info(&mut self, user_id: u64) -> Result<StreamInfo, String> {
        info!(target: TARGET, "Getting stream info for user: {}", user_id);

        let result = self.validate_token().await;

        if let Some(error) = result.err() {
            return Err(format!("Could not validate token. Reason: {error}"));
        }

        let url = format!("{}{}", Self::BASE_URL, "/streams");
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .query(&[("user_id", user_id)])
            .header("Client-Id", &self.client_id)
            .send()
            .await;

        match response {
            Ok(response) => {
                debug!(target: TARGET, "Got response: {:?}", response);
                let status_code = response.status();
                if StatusCode::is_success(&status_code) {
                    debug!(target: TARGET, "Got stream info");
                    let res = response.json::<StreamInfo>().await;
                    match res {
                        Ok(stream_info) => Ok(stream_info),
                        Err(e) => {
                            warn!(
                                target: TARGET,
                                "Failed parsing response body. Reason: {}",
                                e.to_string()
                            );
                            Err(format!("Failed to get stream info. Reason: {e}"))
                        }
                    }
                } else {
                    let response_text = response.text().await.unwrap_or_else(|err| err.to_string());
                    warn!(
                        target: TARGET,
                        "Failed getting stream info. Reason: {response_text}"
                    );
                    Err(format!(
                        "Failed to get stream info. Reason: {response_text}"
                    ))
                }
            }
            Err(error) => {
                error!(
                    target: TARGET,
                    "Error when sending request. Reason: {:?}", error
                );
                Err(format!("Failed to get stream info. Reason: {error}"))
            }
        }
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct StreamData {
    /// An ID that identifies the stream. You can use this ID later to look up the video on demand (VOD).
    pub id: String,
    /// The ID of the user that’s broadcasting the stream.
    pub user_id: String,
    /// The user’s login name.
    pub user_login: String,
    /// The user’s display name.
    pub user_name: String,
    /// The ID of the category or game being played.
    pub game_id: String,
    /// The name of the category or game being played.
    pub game_name: String,
    /// The type of stream. Possible values are:
    /// * live
    /// If an error occurs, this field is set to an empty string.
    #[serde(rename = "type")]
    pub stream_type: String,
    /// The stream’s title. Is an empty string if not set.
    pub title: String,
    /// The tags applied to the stream.
    pub tags: Vec<String>,
    /// The number of users watching the stream.
    pub viewer_count: u32,
    /// The UTC date and time (in RFC3339 format) of when the broadcast began.
    pub started_at: String,
    /// The language that the stream uses. This is an ISO 639-1 two-letter language code or *other* if the stream uses a language not in the list of [supported stream languages](https://help.twitch.tv/s/article/languages-on-twitch#streamlang).
    pub language: String,
    /// A URL to an image of a frame from the last 5 minutes of the stream. Replace the width and height placeholders in the URL (`{width}x{height}`) with the size of the image you want, in pixels.
    pub thumbnail_url: String,
    /// A Boolean value that indicates whether the stream is meant for mature audiences.
    pub is_mature: bool,
}

#[derive(Default, Debug, Deserialize)]
pub struct Pagination {
    /// The cursor used to get the next page of results. Set the request’s *after* or before query parameter to this value depending on whether you’re paging forwards or backwards.
    cursor: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct StreamInfo {
    /// The list of streams.
    data: Vec<StreamData>,
    /// The information used to page through the list of results. The object is empty if there are no more pages left to page through. [Read More](https://dev.twitch.tv/docs/api/guide#pagination)
    pagination: Pagination,
}

#[derive(Default, Debug, Deserialize)]
struct Credentials {
    access_token: String,
    expires_in: u32,
    token_type: String,
}

#[derive(Default, Debug, Deserialize)]
struct Validation {
    client_id: String,
    login: Option<String>,
    scopes: Option<Vec<String>>,
    user_id: Option<String>,
    expires_in: u32,
}
