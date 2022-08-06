mod api;
use api::task::get_task;
use api::webhooks::twitch::twitch_webhook;

use actix_web::{middleware::Logger, App, HttpServer};
use twitch_irc_parser::parse_messages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let port = get_env_port();

    println!("Running on port {}", port);
    let messages = "Hemnlo";
    parse_messages(messages);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_task)
            .service(twitch_webhook)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn get_env_port() -> u16 {
    std::env::var("RUST_PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse::<u16>()
        .unwrap_or(8080)
}
