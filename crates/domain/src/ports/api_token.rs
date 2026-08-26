use crate::api_token::{ApiToken, ApiTokenId, TokenDigest};
use crate::errors::DomainError;
use crate::user::UserId;

pub trait ApiTokenSecretPort: Send + Sync {
    fn mint(&self) -> String;
    fn digest(&self, secret: &str) -> TokenDigest;
}

#[async_trait::async_trait]
pub trait ApiTokenCommandPort: Send + Sync {
    async fn save(&self, token: &ApiToken) -> Result<(), DomainError>;
    async fn revoke(&self, user_id: &UserId, id: &ApiTokenId) -> Result<(), DomainError>;
    async fn mark_used(&self, id: &ApiTokenId) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait ApiTokenQueryPort: Send + Sync {
    async fn find_by_digest(&self, digest: &TokenDigest) -> Result<Option<ApiToken>, DomainError>;
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ApiToken>, DomainError>;
}
