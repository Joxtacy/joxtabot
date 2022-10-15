///
/// Hi @everyone! I am **live**!
/// > Playing: {game}
/// > Title: {title}
/// https://twitch.tv/joxtacy
///
pub fn create_stream_online_message(game: &str, title: &str) -> String {
    format!(
        "Hi @everyone! I am **live**!\n\
             > Playing: {}\n\
             > Title: {}\n\
             https://twitch.tv/joxtacy",
        game, title
    )
}

pub fn create_privmsg(channel: &str, message: &str) -> String {
    format!("PRIVMSG #{} :{}", channel, message)
}
