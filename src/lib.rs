use chacha20poly1305::{
    aead::{Aead, KeyInit},
    Error as ChachaError, Key, XChaCha20Poly1305, XNonce,
};
use rand::seq::IteratorRandom;
use std::{fs, path::PathBuf};
use structopt::StructOpt;
use thiserror::Error;

const MAX_KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 24;

#[derive(Debug, Error)]
pub enum SimpleCipherError {
    #[error(transparent)]
    Chacha(#[from] ChachaError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Conversion(#[from] std::string::FromUtf8Error),
    #[error("Key is {0} bytes long. Select a key that is less than 32 bytes long")]
    KeyTooLong(usize),
    #[error("Nonce generation not supported with decrypt")]
    NonceGenerate,
    #[error("Must select no-nonce, a nonce string or a generated nonce")]
    NonceChoiceUndeteremined,
    #[error("This nonce is {0} bytes long. Select a key that is less than 24 bytes long")]
    NonceTooLong(usize),
}

#[derive(StructOpt, Debug)]
pub struct CommonEncryptionOpts {
    #[structopt(short, long)]
    /// This is an encryption key. It must be less than 32 characters long.
    key: String,

    #[structopt(short, long, parse(from_os_str), default_value = "data.dat")]
    /// This is the file which an message is encrypted/decrypted to/from.
    encrypted_file: PathBuf,

    #[structopt(long)]
    /// **NOT RECOMMENDED:** This is a helper option to enable the nonce be all zeros. This results
    /// in the encrypted message be the same on every encryption and subject to a replay attacks.
    no_nonce: bool,

    #[structopt(short, long)]
    /// This is a flag to enable a newly generated nonce on encryption. This will error when used
    /// on decryption.
    generate_nonce: bool,

    #[structopt(short, long)]
    /// This is the string representation of a nonce as ascii characters. This is required for
    /// decryption unless using the unrecommended --no-nonce feature.
    nonce: Option<String>,
}
impl CommonEncryptionOpts {
    pub fn encrypt(&self, message: String) -> Result<Option<String>, SimpleCipherError> {
        let key = self.get_key_from_string()?;
        let nonce = self.nonce()?;

        let cipher = XChaCha20Poly1305::new(&key);
        let ciphertext = cipher.encrypt(&nonce, message.into_bytes().as_ref())?;
        fs::write(&self.encrypted_file, ciphertext)?;
        if self.generate_nonce {
            Ok(Some(Self::stringify_nonce(&nonce)))
        } else {
            Ok(None)
        }
    }

    pub fn decrypt(&self) -> Result<String, SimpleCipherError> {
        if self.generate_nonce {
            return Err(SimpleCipherError::NonceGenerate);
        }
        let key = self.get_key_from_string()?;
        let nonce = self.nonce()?;

        let cipher = XChaCha20Poly1305::new(&key);
        let ciphertext = &fs::read(&self.encrypted_file)?;
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_slice())?;
        let plaintext = String::from_utf8(plaintext)?;
        Ok(plaintext)
    }

    // This function simply takes a string, converts it to bytes, and pads the vec to be 32 bytes long
    // as this key is 32 bytes long.
    fn get_key_from_string(&self) -> Result<Key, SimpleCipherError> {
        let mut key = self.key.clone().into_bytes();
        if key.len() > MAX_KEY_LENGTH {
            return Err(SimpleCipherError::KeyTooLong(key.len()));
        }
        let mut padding_bytes = vec![0_u8; MAX_KEY_LENGTH - key.len()];
        key.append(&mut padding_bytes);
        Ok(*Key::from_slice(&key))
    }

    // This is a helper function to make a nonce a string. This is for converting a generated nonce
    // into a string for decryption
    fn stringify_nonce(nonce: &XNonce) -> String {
        let nonce: String = nonce
            .iter()
            .map(|val| *val as char)
            .collect::<Vec<char>>()
            .into_iter()
            .collect();
        nonce
    }

    // This is a helper function to turn a string into a nonce. This is used when the user wants to
    // specify a given nonce via the CLI.
    fn nonce_from_string(nonce: String) -> Result<XNonce, SimpleCipherError> {
        if nonce.len() > NONCE_LENGTH {
            return Err(SimpleCipherError::NonceTooLong(nonce.len()));
        }
        let mut nonce: Vec<u8> = nonce.chars().map(|v| v as u8).collect();
        let mut padding_bytes = vec![0_u8; NONCE_LENGTH - nonce.len()];
        nonce.append(&mut padding_bytes);
        Ok(*XNonce::from_slice(nonce.as_slice()))
    }

    // This function either:
    // * generates a nonce
    // * returns a nonce of all zeros (**NOT RECOMMENDED**)
    // * converts a nonce-string to an XNonce.
    fn nonce(&self) -> Result<XNonce, SimpleCipherError> {
        if !self.no_nonce && self.nonce.is_none() && !self.generate_nonce {
            return Err(SimpleCipherError::NonceChoiceUndeteremined);
        }
        if self.no_nonce {
            let nonce = vec![0_u8; NONCE_LENGTH];
            return Ok(*XNonce::from_slice(&nonce));
        }
        if self.generate_nonce {
            let mut rng = rand::thread_rng();

            // There is almost certainly a better way to do this.
            // The choos_multiple function in rand does not reuse existing values from my short
            // tests.
            // https://docs.rs/rand/latest/rand/seq/trait.IteratorRandom.html#method.choose_multiple
            // Given that the goal of this is to make a nonce easy to enter, copy and paste
            // usage of a corpus of each lower case letter of the alphabet repeated NONCE_LENGTH
            // times, there is probably enough entropy.
            //
            // The ChaCha23Poly1305 documentation has actual math behind theuir random nonces.
            // https://docs.rs/aead/latest/src/aead/lib.rs.html#114-148
            //
            // In this case, 26*24 input characters with selecting  24 characters and as
            // `choose_multiple` selects some without repetitions, I think the number of
            // combinations is 624 choose 24. Which has ~1.25e43 combinations, this *feels* like a
            // sufficiently large set but the author of this nonce-subset hack is not a
            // cyrptographer and would require a proper audit.
            let potential_nonce_chars: String = vec![
                vec!["a"; NONCE_LENGTH].join(""),
                vec!["b"; NONCE_LENGTH].join(""),
                vec!["c"; NONCE_LENGTH].join(""),
                vec!["d"; NONCE_LENGTH].join(""),
                vec!["e"; NONCE_LENGTH].join(""),
                vec!["f"; NONCE_LENGTH].join(""),
                vec!["g"; NONCE_LENGTH].join(""),
                vec!["h"; NONCE_LENGTH].join(""),
                vec!["i"; NONCE_LENGTH].join(""),
                vec!["j"; NONCE_LENGTH].join(""),
                vec!["k"; NONCE_LENGTH].join(""),
                vec!["l"; NONCE_LENGTH].join(""),
                vec!["m"; NONCE_LENGTH].join(""),
                vec!["o"; NONCE_LENGTH].join(""),
                vec!["o"; NONCE_LENGTH].join(""),
                vec!["p"; NONCE_LENGTH].join(""),
                vec!["q"; NONCE_LENGTH].join(""),
                vec!["r"; NONCE_LENGTH].join(""),
                vec!["s"; NONCE_LENGTH].join(""),
                vec!["t"; NONCE_LENGTH].join(""),
                vec!["u"; NONCE_LENGTH].join(""),
                vec!["v"; NONCE_LENGTH].join(""),
                vec!["w"; NONCE_LENGTH].join(""),
                vec!["x"; NONCE_LENGTH].join(""),
                vec!["y"; NONCE_LENGTH].join(""),
                vec!["z"; NONCE_LENGTH].join(""),
            ]
            .join("");
            let nonce: String = potential_nonce_chars
                .chars()
                .choose_multiple(&mut rng, NONCE_LENGTH)
                .into_iter()
                .collect();

            return Ok(*XNonce::from_slice(nonce.as_bytes()));
        }
        if let Some(nonce_string) = &self.nonce {
            return Self::nonce_from_string(nonce_string.to_string());
        }
        Err(SimpleCipherError::NonceChoiceUndeteremined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt_with_nonce() {
        let key = "baz".to_string();
        let input = "foobar".to_string();
        let nonce = vec!["a"; NONCE_LENGTH].join("");
        let tmpdir = tempfile::tempdir().expect("Failed to create tempdir");
        let encrypted_file = tmpdir.path().join("encyrpted.dat");
        let encrypt_opts = CommonEncryptionOpts {
            key: key.clone(),
            generate_nonce: false,
            encrypted_file: encrypted_file.clone(),
            no_nonce: false,
            nonce: Some(nonce.clone()),
        };
        let decrypt_opts = CommonEncryptionOpts {
            key,
            encrypted_file,
            generate_nonce: false,
            no_nonce: false,
            nonce: Some(nonce),
        };

        let _ = encrypt_opts
            .encrypt(input.clone())
            .expect("Failed to encrypt data");
        let output = decrypt_opts.decrypt().expect("Failed to decrypt data");
        assert_eq!(input, output);
    }

    #[test]
    fn fail_to_decrypt() {
        let encrypt_key = "this is the encryption key".to_string();
        let decrypt_key = "this is not the encryption key".to_string();
        let input = "foobar".to_string();
        let nonce = vec!["b"; NONCE_LENGTH].join("");

        let tmpdir = tempfile::tempdir().expect("Failed to create tempdir");
        let encrypted_file = tmpdir.path().join("encyrpted.dat");
        let encrypt_opts = CommonEncryptionOpts {
            key: encrypt_key,
            encrypted_file: encrypted_file.clone(),
            generate_nonce: false,
            no_nonce: false,
            nonce: Some(nonce.clone()),
        };
        let decrypt_opts = CommonEncryptionOpts {
            key: decrypt_key,
            generate_nonce: false,
            encrypted_file,
            no_nonce: false,
            nonce: Some(nonce),
        };

        let _ = encrypt_opts.encrypt(input).expect("Failed to encrypt data");
        let output = decrypt_opts.decrypt();
        assert!(output.is_err());
    }

    // Verify that if our input key string exceeds 32 bytes, it throws an error.
    #[test]
    fn encryption_key_too_long() {
        const BAD_KEY_LENGTH: usize = MAX_KEY_LENGTH + 1;

        let encrypt_key = vec!["a"; BAD_KEY_LENGTH].join("");
        let input = "foobar".to_string();
        let nonce = vec!["c"; NONCE_LENGTH].join("");

        let tmpdir = tempfile::tempdir().expect("Failed to create tempdir");
        let encrypted_file = tmpdir.path().join("encyrpted.dat");
        let encrypt_opts = CommonEncryptionOpts {
            key: encrypt_key,
            encrypted_file: encrypted_file.clone(),
            generate_nonce: false,
            no_nonce: false,
            nonce: Some(nonce.clone()),
        };

        let encrypt_out = encrypt_opts.encrypt(input.clone());
        assert!(encrypt_out.is_err());
        let encrypt_out = encrypt_out.err().unwrap();
        assert_eq!(
            format!("{encrypt_out:?}"),
            format!("{:?}", SimpleCipherError::KeyTooLong(BAD_KEY_LENGTH))
        );

        let decrypt_key = vec!["a"; BAD_KEY_LENGTH].join("");
        let encrypt_key = vec!["a"; MAX_KEY_LENGTH].join("");
        let encrypt_opts = CommonEncryptionOpts {
            key: encrypt_key,
            encrypted_file: encrypted_file.clone(),
            no_nonce: false,
            nonce: Some(nonce.clone()),
            generate_nonce: false,
        };
        let decrypt_opts = CommonEncryptionOpts {
            key: decrypt_key,
            encrypted_file,
            no_nonce: false,
            generate_nonce: false,
            nonce: Some(nonce),
        };

        let _ = encrypt_opts.encrypt(input).expect("Failed to encrypt data");
        let decrypt_out = decrypt_opts.decrypt();

        assert!(decrypt_out.is_err());
        let decrypt_out = decrypt_out.unwrap_err();

        // To quote the docs, this error is intentionally opaque to prevent side channel attacks.
        assert_eq!(
            format!("{decrypt_out:?}"),
            format!("{:?}", SimpleCipherError::KeyTooLong(BAD_KEY_LENGTH))
        );
    }

    #[test]
    fn encrypt_and_decrypt_zeros_as_a_nonce() {
        let key = "baz".to_string();
        let input = "foobar".to_string();

        let tmpdir = tempfile::tempdir().expect("Failed to create tempdir");
        let encrypted_file = tmpdir.path().join("encyrpted.dat");
        let encrypt_opts = CommonEncryptionOpts {
            generate_nonce: false,
            key: key.clone(),
            encrypted_file: encrypted_file.clone(),
            no_nonce: true,
            nonce: None,
        };
        let _ = encrypt_opts
            .encrypt(input.clone())
            .expect("Failed to encrypt data");
        let decrypt_opts = CommonEncryptionOpts {
            key,
            encrypted_file,
            generate_nonce: false,
            no_nonce: true,
            nonce: None,
        };

        let output = decrypt_opts.decrypt().expect("Failed to decrypt data");
        assert_eq!(input, output);
    }

    #[test]
    fn encrypt_and_decrypt_zeros_with_generated_nonce() {
        let key = "baz".to_string();
        let input = "foobar".to_string();

        let tmpdir = tempfile::tempdir().expect("Failed to create tempdir");
        let encrypted_file = tmpdir.path().join("encyrpted.dat");
        let encrypt_opts = CommonEncryptionOpts {
            generate_nonce: true,
            key: key.clone(),
            encrypted_file: encrypted_file.clone(),
            no_nonce: false,
            nonce: None,
        };
        let generated_nonce = encrypt_opts
            .encrypt(input.clone())
            .expect("Failed to encrypt data");

        let decrypt_opts = CommonEncryptionOpts {
            key,
            encrypted_file,
            generate_nonce: false,
            no_nonce: false,
            nonce: generated_nonce,
        };

        let output = decrypt_opts.decrypt().expect("Failed to decrypt data");
        assert_eq!(input, output);
    }
}
