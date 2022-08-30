#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("TWITCH_APP_ACCESS_TOKEN").unwrap();
    let client_id = std::env::var("TWITCH_CLIENT_ID").unwrap();
    let user_id = std::env::var("TWITCH_JOXTACY_USER_ID")
        .unwrap()
        .parse()
        .unwrap();
    let res = twitch_utils::get_stream_info(token.to_string(), client_id, user_id).await;
    println!("How did it go? {:?}", res);
}
