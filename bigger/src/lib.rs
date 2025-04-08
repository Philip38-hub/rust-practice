use::std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max = 0;
    for &value in h.values(){
        if value > max {
            max = value;
        }
    }
    max
}