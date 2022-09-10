use discord_utils::DiscordBuilder;

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
    let res = DiscordBuilder::new(&token)
        .build()
        .create_message(channel_id, "testing")
        .await;
    println!("How did it go? {:?}", res);
}
