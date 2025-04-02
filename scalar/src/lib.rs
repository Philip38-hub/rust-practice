pub fn sum(left: u8, right: u8) -> u8 {
    left + right
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rem(-128.23, 2.0); // 'rem: -0.22999573'
        assert_eq!(result, -0.22999573);
    }
}
