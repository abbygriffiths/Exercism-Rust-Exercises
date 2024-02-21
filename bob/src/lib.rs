pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_yelling = message.chars().any(char::is_alphabetic) && message == message.to_uppercase();
    let is_question = message.ends_with("?");

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}
