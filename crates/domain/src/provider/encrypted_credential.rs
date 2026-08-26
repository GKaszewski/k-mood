use crate::errors::DomainError;

#[derive(Clone, PartialEq, Eq)]
pub struct EncryptedCredential(Vec<u8>);

impl EncryptedCredential {
    pub fn from_persistence(ciphertext: Vec<u8>) -> Self {
        Self(ciphertext)
    }

    pub fn value(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for EncryptedCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EncryptedCredential(<redacted {} bytes>)", self.0.len())
    }
}

#[async_trait::async_trait]
pub trait CredentialCipher: Send + Sync {
    fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedCredential, DomainError>;
    fn decrypt(&self, credential: &EncryptedCredential) -> Result<Vec<u8>, DomainError>;
}
