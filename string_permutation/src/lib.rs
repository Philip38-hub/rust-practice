use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut freq1 = HashMap::new();
    let mut freq2 = HashMap::new();

    for c in s1.chars() {
        *freq1.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *freq2.entry(c).or_insert(0) += 1;
    }

    freq1 == freq2
}
