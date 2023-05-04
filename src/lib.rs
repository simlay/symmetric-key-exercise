use std::path::PathBuf;
use thiserror::Error;
use chacha20poly1305::{
    Key,
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Error as ChachaError,
    XChaCha20Poly1305, XNonce,
};
use structopt::StructOpt;

#[derive(Debug, Error, PartialEq)]
pub enum SimpleCipherError {
    #[error(transparent)]
    Chacha(#[from] ChachaError),
    #[error(transparent)]
    Utf8Conversion(#[from] std::string::FromUtf8Error),
    #[error("foobar")]
    KeyTooLong(usize),
}

#[derive(StructOpt, Debug)]
pub struct CommonEncryptionOpts {
    #[structopt(short, long)]
    key: String,

    #[structopt(short, long, parse(from_os_str), default_value = "data.dat")]
    encrypted_file: PathBuf,

    #[structopt(short, long)]
    no_nonce: bool,

    #[structopt(short, long)]
    nonce: String,
}


const MAX_KEY_LENGTH: usize = 32;
pub const NONCE_LENGTH: usize = 24;


pub fn encrypt(key: String, message: String, nonce: Option<&XNonce>) -> Result<(Vec<u8>, XNonce), SimpleCipherError> {
    let key = get_key_from_string(key)?;
    let nonce = if let Some(nonce) = nonce {
        *nonce
    } else {
        XChaCha20Poly1305::generate_nonce(&mut OsRng)
    };

    let cipher = XChaCha20Poly1305::new(&key);
    let ciphertext = cipher.encrypt(&nonce, message.into_bytes().as_ref())?;
    Ok((ciphertext, nonce))
}

pub fn decrypt(key: String, ciphertext: Vec<u8>, nonce: &XNonce) -> Result<String, SimpleCipherError> {
    let key = get_key_from_string(key)?;

    let cipher = XChaCha20Poly1305::new(&key);
    let plaintext = cipher.decrypt(nonce, &*ciphertext)?;
    let plaintext = String::from_utf8(plaintext)?;
    Ok(plaintext)
}


// This function simply takes a string, converts it to bytes, and pads the vec to be 32 bytes long
// as this key is 32 bytes long.
fn get_key_from_string(key: String) -> Result<Key, SimpleCipherError> {
    let mut key = key.into_bytes();
    if key.len() > MAX_KEY_LENGTH {
        return Err(SimpleCipherError::KeyTooLong(key.len()));
    }
    let mut padding_bytes = vec![0_u8; MAX_KEY_LENGTH- key.len()];
    key.append(&mut padding_bytes);
    Ok(*Key::from_slice(&key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt_with_nonce() {
        let key = "baz".to_string();
        let input = "foobar".to_string();
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let (encrypted, nonce) = encrypt(key.clone(), input.clone(), Some(&nonce)).expect("Failed to encrypt data");
        let output = decrypt(key, encrypted, &nonce).expect("Failed to decrypt data");
        assert_eq!(input, output);
    }

    #[test]
    fn fail_to_decrypt() {
        let encrypt_key = "this is the encryption key".to_string();
        let decrypt_key = "this is not the encryption key".to_string();
        let input = "foobar".to_string();
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let (encrypted, nonce) = encrypt(encrypt_key.clone(), input.clone(), Some(&nonce)).expect("Failed to encrypt data");
        let output = decrypt(decrypt_key, encrypted, &nonce);

        // To quote the docs, this error is intentionally opaque to prevent side channel attacks.
        assert_eq!(output, Err(SimpleCipherError::Chacha(ChachaError)));
    }


    // Verify that if our input key string exceeds 32 bytes, it throws an error.
    #[test]
    fn encryption_key_too_long() {

        const BAD_KEY_LENGTH : usize = MAX_KEY_LENGTH + 1;

        let encrypt_key = vec!["a"; BAD_KEY_LENGTH].join("");
        let input = "foobar".to_string();
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let encrypt_out = encrypt(encrypt_key.clone(), input.clone(), Some(&nonce));
        assert_eq!(encrypt_out, Err(SimpleCipherError::KeyTooLong(BAD_KEY_LENGTH)));

        let decrypt_key = vec!["a"; BAD_KEY_LENGTH].join("");
        let encrypt_key = vec!["a"; MAX_KEY_LENGTH ].join("");
        let input = "foobar".to_string();
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

        let (encrypted, nonce) = encrypt(encrypt_key.clone(), input.clone(), Some(&nonce)).expect("Failed to encrypt data");
        let decrypt_out = decrypt(decrypt_key, encrypted, &nonce);

        // To quote the docs, this error is intentionally opaque to prevent side channel attacks.
        assert_eq!(decrypt_out, Err(SimpleCipherError::KeyTooLong(BAD_KEY_LENGTH)));
    }

    #[test]
    fn encrypt_and_decrypt_without_nonce() {
        let key = "baz".to_string();
        let input = "foobar".to_string();

        let (encrypted, nonce) = encrypt(key.clone(), input.clone(), None).expect("Failed to encrypt data");
        let output = decrypt(key, encrypted, &nonce).expect("Failed to decrypt data");
        assert_eq!(input, output);
    }

    #[test]
    fn encrypt_and_decrypt_zeros_as_a_nonce() {
        let key = "baz".to_string();
        let input = "foobar".to_string();
        let nonce = vec![0_u8; NONCE_LENGTH];
        let nonce = XNonce::from_slice(&nonce);

        let (encrypted, nonce) = encrypt(key.clone(), input.clone(), Some(nonce)).expect("Failed to encrypt data");
        let output = decrypt(key, encrypted, &nonce).expect("Failed to decrypt data");
        assert_eq!(input, output);
    }
}
