#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = atbash_cipher(original);

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected
        })
    }
}

fn atbash_cipher(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let b = if c.is_ascii_lowercase() { b'z' } else { b'Z' };
                let mirrored = (base + (b - c as u8)) as char;
                mirrored
            } else {
                c
            }
        })
        .collect()
}