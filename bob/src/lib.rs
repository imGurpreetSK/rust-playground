pub fn reply(message: &str) -> &str {
    let message = message.trim().replace(" ", "");
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    
    let ends_with_question = message.ends_with('?');
    let filtered = message
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>();
    let is_all_caps = filtered.chars().all(|c| c.is_uppercase()) && filtered.chars().count() > 0; // all takes a mut ref!

    if is_all_caps && ends_with_question {
        return "Calm down, I know what I'm doing!";
    }
    if is_all_caps {
        return "Whoa, chill out!";
    }
    if ends_with_question {
        return "Sure.";
    }
    "Whatever."
}
