#[derive(Debug)]
pub struct CeaserCipher {}

impl CeaserCipher {
    pub fn encrypt(plaintext: impl AsRef<str>, shift: u8) -> String {
        plaintext
            .as_ref()
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let shifted = (c as u8 - base + shift) % 26 + base;
                    shifted as char
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn decrypt(ciphertext: impl AsRef<str>, shift: u8) -> String {
        ciphertext
            .as_ref()
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let shifted = (c as u8 - base + 26 - (shift % 26)) % 26 + base;
                    shifted as char
                } else {
                    c
                }
            })
            .collect()
    }
}
