pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exponential = (c as f64).exp();
    let log = (c as f64).ln();
    (c, exponential, log)
}

pub fn str_function(a: String) -> (String, String) {
    let exponential = a
        .split_whitespace()
        .filter_map(|num| num.parse::<f64>().ok())
        .map(|num| num.exp().to_string())
        .collect::<Vec<String>>().join(" ");
    
    (a, exponential)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log = b
        .iter()
        .map(|&num| (num.abs() as f64).ln())
        .collect::<Vec<f64>>();
    (b, log)
}