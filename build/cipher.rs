use aes_gcm_siv::{
    Aes256GcmSiv, KeyInit, Nonce,
    aead::{Aead, generic_array::GenericArray},
};
use blake3::Hasher;
use std::{fs, path::Path};

const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

pub fn decrypt_file(path: &Path, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted = fs::read(path)?;

    if encrypted.len() < SALT_LENGTH + NONCE_LENGTH {
        return Err("File too short".into());
    }

    let salt = &encrypted[0..SALT_LENGTH];
    let nonce = &encrypted[SALT_LENGTH..SALT_LENGTH + NONCE_LENGTH];
    let ciphertext = &encrypted[SALT_LENGTH + NONCE_LENGTH..];

    let key = derive_key(password.as_bytes(), salt);
    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| format!("Decryption failed: {e}"))?;

    String::from_utf8(plaintext).map_err(Into::into)
}

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; KEY_LENGTH] {
    let mut hasher = Hasher::new();
    hasher.update(password);
    hasher.update(salt);
    *hasher.finalize().as_bytes()
}
