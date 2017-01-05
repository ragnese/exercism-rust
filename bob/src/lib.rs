pub fn reply(sentence: &str) -> String {
    if sentence.is_empty() {
        return "Fine. Be that way!".to_string();
    }

    if sentence.ends_with("?") {
        return "Sure.".to_string();
    }

    if sentence == &sentence.to_uppercase() {
        return "Whoa, chill out!".to_string();
    }

    "Whatever.".to_string()
}
