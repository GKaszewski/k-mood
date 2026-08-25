use crate::activity::{Activity, ActivityId};
use crate::errors::DomainError;
use crate::user::UserId;

#[async_trait::async_trait]
pub trait ActivityCommandPort: Send + Sync {
    async fn save(&self, activity: &Activity) -> Result<(), DomainError>;
    async fn delete(&self, id: &ActivityId) -> Result<(), DomainError>;
    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait ActivityQueryPort: Send + Sync {
    async fn find_by_id(&self, id: &ActivityId) -> Result<Option<Activity>, DomainError>;
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Activity>, DomainError>;
    async fn find_active_by_user(&self, user_id: &UserId) -> Result<Vec<Activity>, DomainError>;
}
