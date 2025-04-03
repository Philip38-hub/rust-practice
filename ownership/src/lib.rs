pub fn first_subword(mut s: String) -> String {
    let c = s.chars()
    let result = String::new();

    if let Some(first) = c.next() {
        result.push(first);
    }

    for ch in c {
        if ch.is_uppercase() || ch == '_' {
            break;
        } else {
            result.push(ch);
        }
    }
    result
}