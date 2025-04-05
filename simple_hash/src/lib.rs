use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &'a Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();

    for &word in words {
        // Remove trailing punctuation like "." or "," using trim_end_matches
        let cleaned = word.trim_end_matches(|c: char| !c.is_alphanumeric());

        *map.entry(cleaned).or_insert(0) += 1;
    }

    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
