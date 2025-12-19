use aes_gcm_siv::{
    Aes256GcmSiv, Error, KeyInit, Nonce,
    aead::{Aead, OsRng, generic_array::GenericArray, rand_core::RngCore},
};
use blake3::Hasher;

const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaltedKey {
    key: [u8; KEY_LENGTH],
    salt: [u8; SALT_LENGTH],
    nonce: [u8; NONCE_LENGTH],
}

impl SaltedKey {
    #[must_use]
    pub fn new(password: &str) -> Self {
        let mut salt = [0u8; SALT_LENGTH];
        OsRng.fill_bytes(&mut salt);

        let mut nonce = [0u8; NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce);

        let key = Self::derive_key(password.as_bytes(), &salt);
        Self { key, salt, nonce }
    }

    #[must_use]
    pub fn extract(password: &str, encrypted: &[u8]) -> Self {
        let salt = &encrypted[0..SALT_LENGTH];
        let nonce = &encrypted[SALT_LENGTH..SALT_LENGTH + NONCE_LENGTH];
        let key = Self::derive_key(password.as_bytes(), salt);

        Self {
            key,
            salt: salt.try_into().unwrap(),
            nonce: nonce.try_into().unwrap(),
        }
    }

    /// Reuse salt/nonce from existing encrypted data if available, otherwise generate new ones.
    /// This prevents unnecessary git churn when re-encrypting unchanged content.
    #[must_use]
    pub fn reuse_or_new(password: &str, existing_encrypted: Option<&[u8]>) -> Self {
        if let Some(encrypted) = existing_encrypted
            && encrypted.len() >= SALT_LENGTH + NONCE_LENGTH
        {
            // Reuse existing salt and nonce
            return Self::extract(password, encrypted);
        }
        Self::new(password)
    }

    pub fn encrypt(&self, input: &[u8]) -> Result<Vec<u8>, Error> {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let cipher_text = cipher.encrypt(Nonce::from_slice(&self.nonce), input)?;

        Ok(self
            .salt
            .iter()
            .chain(self.nonce.iter())
            .chain(cipher_text.iter())
            .copied()
            .collect())
    }

    pub fn decrypt(&self, input: &[u8]) -> Result<Vec<u8>, Error> {
        let cipher_text = &input[SALT_LENGTH + NONCE_LENGTH..];

        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));

        cipher.decrypt(Nonce::from_slice(&self.nonce), cipher_text)
    }

    fn derive_key(password: &[u8], salt: &[u8]) -> [u8; KEY_LENGTH] {
        // Blake3 is used instead of Argon2 for performance reasons.
        // The encrypted data has minimal security requirements as it consists
        // solely of puzzle data that is publicly available.
        let mut hasher = Hasher::new();
        hasher.update(password);
        hasher.update(salt);
        *hasher.finalize().as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn identity(input: Vec<u8>, password: String) {
        let key = SaltedKey::new(&password);
        let encrypted = key.encrypt(&input).unwrap();
        let decrypted1 = key.decrypt(&encrypted).unwrap();
        let decrypted2 = SaltedKey::extract(&password, &encrypted)
            .decrypt(&encrypted)
            .unwrap();

        assert_eq!(decrypted1, input);
        assert_eq!(decrypted1, decrypted2);
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn reuse_produces_identical_encryption(data: Vec<u8>, password: String) {
        let key1 = SaltedKey::new(&password);
        let encrypted1 = key1.encrypt(&data).unwrap();

        let key2 = SaltedKey::reuse_or_new(&password, Some(&encrypted1));
        let encrypted2 = key2.encrypt(&data).unwrap();

        assert_eq!(encrypted1, encrypted2);
        assert_eq!(key1, key2);
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn new_keys_produce_different_encryptions(data: Vec<u8>, password: String) {
        let key1 = SaltedKey::reuse_or_new(&password, None);
        let encrypted1 = key1.encrypt(&data).unwrap();

        let key2 = SaltedKey::reuse_or_new(&password, None);
        let encrypted2 = key2.encrypt(&data).unwrap();

        assert_ne!(encrypted1, encrypted2);
        assert_ne!(key1, key2);
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn reuse_preserves_decryptability(data1: Vec<u8>, data2: Vec<u8>, password: String) {
        let key1 = SaltedKey::new(&password);
        let encrypted = key1.encrypt(&data1).unwrap();

        let key2 = SaltedKey::reuse_or_new(&password, Some(&encrypted));
        let new_encrypted = key2.encrypt(&data2).unwrap();

        let decrypted_original = key2.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted_original, data1);

        let decrypted_new = key2.decrypt(&new_encrypted).unwrap();
        assert_eq!(decrypted_new, data2);
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn deterministic_reencryption(data: Vec<u8>, password: String) {
        let key = SaltedKey::new(&password);
        let encrypted1 = key.encrypt(&data).unwrap();

        let reused_key = SaltedKey::reuse_or_new(&password, Some(&encrypted1));
        let encrypted2 = reused_key.encrypt(&data).unwrap();

        assert_eq!(encrypted1, encrypted2);

        let encrypted3 = SaltedKey::reuse_or_new(&password, Some(&encrypted2))
            .encrypt(&data)
            .unwrap();
        assert_eq!(encrypted1, encrypted3);
    }

    #[test]
    fn reuse_with_invalid_data_generates_new_key() {
        let password = "password";

        let invalid_data = b"too short";

        let key = SaltedKey::reuse_or_new(password, Some(invalid_data));

        // Should generate new key since data is invalid
        let data = b"test";
        let encrypted = key.encrypt(data).unwrap();
        let decrypted = key.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, data);
    }
}
