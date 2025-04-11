pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_question = trimmed.ends_with('?');
    let is_yelling = trimmed.chars().any(|c| c.is_ascii_alphabetic()) &&
                     trimmed.chars().all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
