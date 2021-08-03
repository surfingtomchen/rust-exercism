pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let has_chars = trimmed.chars().filter(|x| x.is_ascii() && x.is_alphabetic()).collect::<String>().len() > 0;

    return if let Some('?') = trimmed.chars().last() {
        if trimmed.to_ascii_uppercase() == trimmed && has_chars {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else {
        if trimmed.to_ascii_uppercase() == trimmed && has_chars {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    };
}
