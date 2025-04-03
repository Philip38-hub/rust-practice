pub fn str_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = str_len("camelCase");
        assert_eq!(result, 9);
    }
}
