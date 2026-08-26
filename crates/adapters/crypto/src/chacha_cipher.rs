use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit, OsRng, rand_core::RngCore};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};

use domain::errors::DomainError;
use domain::provider::{CredentialCipher, EncryptedCredential};

const KEY_BYTES: usize = 32;
const NONCE_BYTES: usize = 24;

pub struct ChaChaCredentialCipher {
    cipher: XChaCha20Poly1305,
}

impl ChaChaCredentialCipher {
    pub fn new(base64_key: &str) -> Result<Self, DomainError> {
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(base64_key.trim())
            .map_err(|_| {
                DomainError::InvalidInput("credential encryption key must be base64".into())
            })?;

        if key_bytes.len() != KEY_BYTES {
            return Err(DomainError::InvalidInput(format!(
                "credential encryption key must decode to {KEY_BYTES} bytes, got {}",
                key_bytes.len()
            )));
        }

        Ok(Self {
            cipher: XChaCha20Poly1305::new(Key::from_slice(&key_bytes)),
        })
    }
}

impl CredentialCipher for ChaChaCredentialCipher {
    fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedCredential, DomainError> {
        let mut nonce_bytes = [0u8; NONCE_BYTES];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = XNonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| DomainError::InvalidInput("failed to encrypt credential".into()))?;

        let mut sealed = nonce_bytes.to_vec();
        sealed.extend_from_slice(&ciphertext);

        Ok(EncryptedCredential::from_persistence(sealed))
    }

    fn decrypt(&self, credential: &EncryptedCredential) -> Result<Vec<u8>, DomainError> {
        let sealed = credential.value();

        if sealed.len() <= NONCE_BYTES {
            return Err(DomainError::InvalidInput(
                "stored credential is truncated".into(),
            ));
        }

        let (nonce_bytes, ciphertext) = sealed.split_at(NONCE_BYTES);

        self.cipher
            .decrypt(XNonce::from_slice(nonce_bytes), ciphertext)
            .map_err(|_| DomainError::InvalidInput("failed to decrypt credential".into()))
    }
}
