pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let vec = s
        .split_whitespace()
        .map(|item| {
            if item.ends_with('k') {
                let num = item.trim_end_matches('k').parse::<f32>().unwrap_or(0.0);
                (num * 1000.0) as u32
            } else {
                item.parse::<u32>().unwrap_or(0)
            }
        })
        .collect::<Vec<u32>>();
    Box::new(vec)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}