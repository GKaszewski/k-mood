use crate::entry::Date;
use crate::errors::DomainError;
use crate::user::{UserId, UserPreferences};

#[async_trait::async_trait]
pub trait CycleStartCommandPort: Send + Sync {
    async fn record(&self, user_id: &UserId, date: &Date) -> Result<(), DomainError>;
    async fn forget(&self, user_id: &UserId, date: &Date) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait CycleStartQueryPort: Send + Sync {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Date>, DomainError>;
}

#[async_trait::async_trait]
pub trait UserPreferencesCommandPort: Send + Sync {
    async fn save(&self, preferences: &UserPreferences) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait UserPreferencesQueryPort: Send + Sync {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Option<UserPreferences>, DomainError>;
}
