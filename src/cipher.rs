use aes_gcm_siv::{
    Aes256GcmSiv, KeyInit, Nonce,
    aead::{Aead, OsRng, generic_array::GenericArray, rand_core::RngCore},
};

const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

#[derive(Debug, Clone)]
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

    #[must_use]
    pub fn encrypt(&self, input: &[u8]) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let cipher_text = cipher
            .encrypt(Nonce::from_slice(&self.nonce), input)
            .expect("Encryption failed");

        self.salt
            .iter()
            .chain(self.nonce.iter())
            .chain(cipher_text.iter())
            .copied()
            .collect()
    }

    #[must_use]
    pub fn decrypt(&self, input: &[u8]) -> Vec<u8> {
        let cipher_text = &input[SALT_LENGTH + NONCE_LENGTH..];

        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));

        cipher
            .decrypt(Nonce::from_slice(&self.nonce), cipher_text)
            .expect("Decryption failed")
    }

    #[cfg(test)]
    fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
        // Argon2 is slow by design, so blake3 is used for unit testing
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(password);
        hasher.update(salt);
        *hasher.finalize().as_bytes()
    }

    #[cfg(not(test))]
    fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
        use argon2::Argon2;
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password, salt, &mut key)
            .unwrap();
        key
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
        let encrypted = key.encrypt(&input);
        let decrypted_1 = key.decrypt(&encrypted);
        let decrypted_2 = SaltedKey::extract(&password, &encrypted).decrypt(&encrypted);

        assert_eq!(decrypted_1, input);
        assert_eq!(decrypted_1, decrypted_2);
    }
}
