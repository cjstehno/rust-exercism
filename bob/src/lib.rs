
pub fn reply(input : &str) -> String {
    return match input {
        _ if is_nothing(input)  => "Fine. Be that way!".to_string(),
        _ if is_yelling(input)  => "Whoa, chill out!".to_string(),
        _ if is_question(input) => "Sure.".to_string(),
        _                       => "Whatever.".to_string()
    }
}

fn is_yelling(text : &str) -> bool {
    return text.chars().find(|ch| ch.is_lowercase() ).is_none();
}

fn is_question(text : &str) -> bool {
    return text.ends_with("?");
}

fn is_nothing(text: &str) -> bool {
    return text.is_empty();
}
