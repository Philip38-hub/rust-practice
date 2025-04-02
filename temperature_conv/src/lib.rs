pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-32.0) * 1.8
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    9.0/5.0 * c + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }
}
