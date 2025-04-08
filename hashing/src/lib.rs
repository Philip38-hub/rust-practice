use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort_unstable(); // faster and fine for primitive types

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2
    } else {
        sorted[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut freq = HashMap::new();

    for &num in list {
        *freq.entry(num).or_insert(0) += 1;
    }

    // Find the value with the max frequency
    *freq.iter().max_by_key(|&(_, count)| count).unwrap().0
}
