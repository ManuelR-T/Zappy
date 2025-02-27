//
// EPITECH PROJECT, 2024
// Zappy
// File description:
// cipher
//

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use hex::{decode, encode};
use sha2::{Digest, Sha256};

use zappy_macros::Bean;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// The `Crypt` struct holds the encryption key and nonce.
///
/// * `key` - A string that represents the encryption key.
/// * `nonce` - A 12-byte array used as the nonce for encryption and decryption.
#[derive(Debug, Bean)]
pub struct Crypt {
    key: String,
    nonce: [u8; 12],
}

impl Crypt {
    /// Creates a new `Crypt` instance with the provided key.
    ///
    /// * `key` - A string that represents the encryption key.
    ///
    /// ### Example
    ///
    /// ```
    /// let crypt = Crypt::new("my-secret-key".to_string());
    /// ```
    pub fn new(key: String) -> Self {
        let nonce = derive_nonce_from_string("zappy");
        Self { key, nonce }
    }

    fn get_key(key: &str) -> [u8; 32] {
        let mut key_bytes = [0u8; 32];
        let key_slice = key.as_bytes();
        let len = key_slice.len().min(32);
        key_bytes[..len].copy_from_slice(&key_slice[..len]);
        key_bytes
    }

    fn create_cipher(&self) -> Aes256Gcm {
        let key_bytes = Crypt::get_key(&self.key);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Aes256Gcm::new(key)
    }

    /// Encrypts the given data.
    ///
    /// * `data` - A vector of bytes representing the data to be encrypted.
    ///
    /// returns an `Option<String>` containing the encrypted data in hexadecimal format, or `None` if encryption fails.
    ///
    /// ### Example
    ///
    /// ```
    /// let crypt = Crypt::new("my-secret-key".to_string());
    /// let encrypted = crypt.encrypt(b"Hello, world!".to_vec()).unwrap();
    /// ```
    pub fn encrypt(&self, data: Vec<u8>) -> Option<String> {
        let cipher = self.create_cipher();
        let nonce = Nonce::from_slice(&self.nonce);
        debug!("Encrypting with nonce: {}", encode(self.nonce));

        cipher.encrypt(nonce, data.as_ref()).ok().map(encode)
    }

    /// Decrypts the given encrypted data.
    ///
    /// * `encrypted_data` - A string slice containing the encrypted data in hexadecimal format.
    ///
    /// returns an `Option<String>` containing the decrypted data as a UTF-8 string, or `None` if decryption fails.
    ///
    /// ### Example
    ///
    /// ```
    /// let crypt = Crypt::new("my-secret-key".to_string());
    /// let decrypted = crypt.decrypt(&encrypted_data).unwrap();
    /// ```
    pub fn decrypt(&self, encrypted_data: &str) -> Option<String> {
        let cipher = self.create_cipher();
        let nonce = Nonce::from_slice(&self.nonce);
        debug!("Decrypting with nonce: {}", encode(self.nonce));

        let ciphertext = decode(encrypted_data).ok()?;
        let decrypted_data = cipher.decrypt(nonce, ciphertext.as_ref()).ok()?;
        String::from_utf8(decrypted_data).ok()
    }
}

fn derive_nonce_from_string(input: &str) -> [u8; 12] {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&result[..12]);
    nonce
}

#[cfg(test)]
mod test_crypt {
    use super::{derive_nonce_from_string, Crypt};

    #[test]
    fn test_encrypt_decrypt() {
        let crypt = Crypt::new("test".to_string());
        let data = "Hello, World!".as_bytes().to_vec();
        let encrypted_data = crypt.encrypt(data.clone()).unwrap();
        let decrypted_data = crypt.decrypt(&encrypted_data).unwrap();
        assert_eq!(data, decrypted_data.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let crypt = Crypt::new("test".to_string());
        let data = "".as_bytes().to_vec();
        let encrypted_data = crypt.encrypt(data.clone()).unwrap();
        let decrypted_data = crypt.decrypt(&encrypted_data).unwrap();
        assert_eq!(data, decrypted_data.as_bytes());
    }

    #[test]
    fn test_encrypt_decrypt_long() {
        let crypt = Crypt::new("Team1".to_string());
        let data = "2 Need Incantationers for Level 2".as_bytes().to_vec();
        let encrypted_data = crypt.encrypt(data.clone()).unwrap();
        let decrypted_data = crypt.decrypt(&encrypted_data).unwrap();
        assert_eq!(data, decrypted_data.as_bytes());
    }

    #[test]
    fn test_derive_nonce_from_string() {
        let nonce = derive_nonce_from_string("zappy");
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_derive_nonce_from_string_empty() {
        let nonce = derive_nonce_from_string("");
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_get_key() {
        let key = "test";
        let key = Crypt::get_key(key);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_get_key_empty() {
        let key = "";
        let key = Crypt::get_key(key);
        assert_eq!(key.len(), 32);
    }
}
