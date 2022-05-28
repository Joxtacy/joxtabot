use twitch_irc_parser::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", parse_message("message"));
}
