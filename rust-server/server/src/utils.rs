/// Get the port number from the `RUST_PORT` environment variable.
///
/// Defaults to `8000`
fn get_env_port() -> u16 {
    let default_port = 8000;
    std::env::var("RUST_PORT")
        .unwrap_or_else(|_| default_port.to_string())
        .parse::<u16>()
        .unwrap_or(default_port)
}

/// Initializes the environment.
///
/// Returns a tuple with the Twitch Bot Token as the `String` and the server port as the `u16`.
pub(crate) fn init_env() -> (String, u16) {
    std::env::set_var("RUST_BACKTRACE", "1");

    dotenv::dotenv().ok();
    env_logger::init();

    let token = match std::env::var("TWITCH_IRC_BOT_OAUTH") {
        Ok(token) => token,
        Err(e) => panic!("Can't proceed without bot token: {}", e),
    };
    let port = get_env_port();

    (token, port)
}
