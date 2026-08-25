use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use domain::errors::DomainError;

pub struct Argon2PasswordHasher;

impl domain::ports::PasswordHasherPort for Argon2PasswordHasher {
    fn hash(&self, raw_password: &str) -> Result<domain::user::PasswordHash, DomainError> {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(raw_password.as_bytes(), &salt)
            .map_err(|e| DomainError::InvalidInput(format!("failed to hash password: {e}")))?
            .to_string();

        Ok(domain::user::PasswordHash::new(hash))
    }

    fn verify(
        &self,
        raw_password: &str,
        hash: &domain::user::PasswordHash,
    ) -> Result<bool, DomainError> {
        let parsed = argon2::password_hash::PasswordHash::new(hash.value())
            .map_err(|e| DomainError::InvalidInput(format!("invalid password hash: {e}")))?;

        Ok(Argon2::default()
            .verify_password(raw_password.as_bytes(), &parsed)
            .is_ok())
    }
}
