use twitch::Twitch;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let token = std::env::var("TWITCH_APP_ACCESS_TOKEN")
        .expect("Needs a Twitch app access token to continue");
    let client_id =
        std::env::var("TWITCH_CLIENT_ID").expect("Needs a Twitch client id to continue");
    let client = reqwest::Client::new();

    let user_id = 54_605_357;

    let mut twitch = Twitch::new(token, client_id, client);
    let response = twitch.get_stream_info(user_id).await;
}
