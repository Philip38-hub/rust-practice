use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect::<HashSet<char>>()
        .len()== 26
}
