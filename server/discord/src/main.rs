use discord::Discord;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("Needs a Discord token to continue");
    let client = reqwest::Client::new();

    let discord = Discord::new(&token, client);
    let message = "henlo";
    discord
        .create_message(843289296260825098, message)
        .await
        .unwrap();
}
