pub mod ceaser_cipher;

#[cfg(test)]
mod tests {
    use crate::ceaser_cipher::CeaserCipher;

    #[test]
    fn test_encrypt_ceaser() {
        let plaintext = "enemyfallingbackbreakthroughimminentlucius";
        let ciphertext = "jsjrdkfqqnslgfhpgwjfpymwtzlmnrrnsjsyqzhnzx";
        assert_eq!(CeaserCipher::encrypt(plaintext, 5), ciphertext);
    }

    #[test]
    fn test_decrypt_ceaser() {
        let plaintext = "enemyfallingbackbreakthroughimminentlucius";
        let ciphertext = "jsjrdkfqqnslgfhpgwjfpymwtzlmnrrnsjsyqzhnzx";
        assert_eq!(CeaserCipher::decrypt(ciphertext, 5), plaintext);
    }
}
