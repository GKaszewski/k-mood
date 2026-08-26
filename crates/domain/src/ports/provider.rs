use crate::errors::DomainError;
use crate::provider::{ProviderConnection, ProviderName};
use crate::user::UserId;

#[async_trait::async_trait]
pub trait ProviderConnectionCommandPort: Send + Sync {
    async fn save(&self, connection: &ProviderConnection) -> Result<(), DomainError>;

    async fn delete(&self, user_id: &UserId, provider: &ProviderName) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait ProviderConnectionQueryPort: Send + Sync {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ProviderConnection>, DomainError>;

    async fn find_by_user_and_provider(
        &self,
        user_id: &UserId,
        provider: &ProviderName,
    ) -> Result<Option<ProviderConnection>, DomainError>;
}
