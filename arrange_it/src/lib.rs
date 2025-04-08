pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(&str, usize)> = phrase
        .split_whitespace()
        .filter_map(|word| {
            let num = word.chars().find(|c| c.is_digit(10))?; // Find the number in the word
            Some((word, num.to_digit(10)? as usize))
        })
        .collect();

    words.sort_by_key(|&(_, num)| num); // Sort by extracted number

    words.iter().map(|&(word, _)| {
        word.chars().filter(|c| !c.is_digit(10)).collect::<String>() // Remove numbers
    }).collect::<Vec<String>>().join(" ")
}

