pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|name|
        name.split_whitespace()
            .map(|word|
                format!("{}.", word.chars().next().unwrap())

            )
            .collect::<Vec<String>>()
            .join(" ")
        )
        .collect()
}
