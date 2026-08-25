use crate::auth::{GeneratedToken, RefreshSession};
use crate::errors::DomainError;
use crate::user::{PasswordHash, UserId};

pub trait PasswordHasherPort: Send + Sync {
    fn hash(&self, raw_password: &str) -> Result<PasswordHash, DomainError>;
    fn verify(&self, raw_password: &str, hash: &PasswordHash) -> Result<bool, DomainError>;
}

#[async_trait::async_trait]
pub trait AuthServicePort: Send + Sync {
    async fn generate_token(&self, user_id: &UserId) -> Result<GeneratedToken, DomainError>;
    async fn validate_token(&self, token: &str) -> Result<UserId, DomainError>;
}

#[async_trait::async_trait]
pub trait RefreshSessionCommandPort: Send + Sync {
    async fn create(&self, session: &RefreshSession) -> Result<(), DomainError>;
    async fn revoke(&self, token: &str) -> Result<(), DomainError>;
    async fn revoke_all_for_user(&self, user_id: &UserId) -> Result<(), DomainError>;
    async fn delete_expired(&self) -> Result<u64, DomainError>;
}

#[async_trait::async_trait]
pub trait RefreshSessionQueryPort: Send + Sync {
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshSession>, DomainError>;
}
