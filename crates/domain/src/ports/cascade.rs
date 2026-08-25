use crate::entry::{DateRange, MoodEntry};
use crate::errors::DomainError;
use crate::user::UserId;

#[async_trait::async_trait]
pub trait CascadeDeletePort: Send + Sync {
    async fn delete_all_user_data(&self, user_id: &UserId) -> Result<(), DomainError>;

    async fn delete_user_account(&self, user_id: &UserId) -> Result<(), DomainError>;

    async fn delete_entries_in_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError>;
}
