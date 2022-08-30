#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_BOT_TOKEN").unwrap();
    let channel_id = std::env::var("DISCORD_TESTING_JOXTABOT_CHANNELID")
        .unwrap()
        .parse()
        .unwrap();
    println!("token: {}", token);
    println!("channelid: {}", channel_id);
    let res =
        discord_utils::create_message(token.to_string(), channel_id, "testing".to_string()).await;
    println!("How did it go? {:?}", res);
}
