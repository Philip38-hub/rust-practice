pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;

    for c in input.chars() {
        if c.is_alphabetic() && new_word {
            result.extend(c.to_uppercase());
            new_word = false;
        } else {
            result.push(c);
            new_word = c.is_whitespace();
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}
