use crate::errors::DomainError;
use crate::user::PasswordHash;

pub struct FakePasswordHasher;

impl crate::ports::PasswordHasherPort for FakePasswordHasher {
    fn hash(&self, raw_password: &str) -> Result<PasswordHash, DomainError> {
        Ok(PasswordHash::new(format!("hashed:{raw_password}")))
    }

    fn verify(&self, raw_password: &str, hash: &PasswordHash) -> Result<bool, DomainError> {
        Ok(hash.value() == format!("hashed:{raw_password}"))
    }
}
