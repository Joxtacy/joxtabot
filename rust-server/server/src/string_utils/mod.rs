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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{create_privmsg, create_stream_online_message};

    #[test]
    fn test_create_privmsg() {
        let result = create_privmsg("My_Channel", "My Message");

        let expected = "PRIVMSG #My_Channel :My Message";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_stream_online_message() {
        let result = create_stream_online_message("Rust", "Playing Game With Toxic Gamers");

        let expected = "Hi @everyone! I am **live**!\n\
                            > Playing: Rust\n\
                            > Title: Playing Game With Toxic Gamers\n\
                            https://twitch.tv/joxtacy";

        assert_eq!(result, expected);
    }
}
