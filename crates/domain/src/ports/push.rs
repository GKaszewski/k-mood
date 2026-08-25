use crate::errors::DomainError;
use crate::push::{PushSubscription, PushSubscriptionId};
use crate::user::UserId;

#[async_trait::async_trait]
pub trait PushSubscriptionCommandPort: Send + Sync {
    async fn save(&self, subscription: &PushSubscription) -> Result<(), DomainError>;
    async fn delete(&self, id: &PushSubscriptionId) -> Result<(), DomainError>;
    async fn delete_by_endpoint(&self, endpoint: &str) -> Result<(), DomainError>;
    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait PushSubscriptionQueryPort: Send + Sync {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<PushSubscription>, DomainError>;

    async fn find_by_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Option<PushSubscription>, DomainError>;
}
