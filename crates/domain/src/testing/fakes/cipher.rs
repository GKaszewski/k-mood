use crate::errors::DomainError;
use crate::provider::{CredentialCipher, EncryptedCredential};

const MASK: u8 = 0x5a;

pub struct FakeCredentialCipher;

impl CredentialCipher for FakeCredentialCipher {
    fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedCredential, DomainError> {
        Ok(EncryptedCredential::from_persistence(
            plaintext.iter().map(|byte| byte ^ MASK).collect(),
        ))
    }

    fn decrypt(&self, credential: &EncryptedCredential) -> Result<Vec<u8>, DomainError> {
        Ok(credential.value().iter().map(|byte| byte ^ MASK).collect())
    }
}
