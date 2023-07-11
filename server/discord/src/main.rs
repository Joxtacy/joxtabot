use discord::Discord;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("Needs a Discord token to continue");
    let client = reqwest::Client::new();

    let discord = Discord::new(&token, client);
    let message = "henlo";
    let _result = discord
        .create_message(843_289_296_260_825_098, message)
        .await;
}
