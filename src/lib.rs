#[derive(Debug)]
pub struct EncryptionError {}

pub fn encrypt(_key: String, _decrypted_message: String) -> Result<String, EncryptionError> {
    todo!()
}

#[derive(Debug)]
pub struct DecryptionError {}

pub fn decrypt(_key: String, _encrypted_message: String) -> Result<String, DecryptionError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encrypt_and_decrypt() {
        let shared_key = "baz".to_string();
        let input = "foobar".to_string();
        let encrypted = encrypt(shared_key.clone(), input.clone()).expect("Failed to encrypt data");
        let output = decrypt(shared_key, encrypted).expect("Failed to decrypt data");
        assert_eq!(input, output);
    }
}
