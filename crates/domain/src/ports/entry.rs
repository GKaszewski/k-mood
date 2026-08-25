use crate::activity::ActivityId;
use crate::entry::{DateRange, Mood, MoodEntry, MoodEntryId};
use crate::errors::DomainError;
use crate::user::UserId;

#[async_trait::async_trait]
pub trait MoodEntryCommandPort: Send + Sync {
    async fn save(&self, entry: &MoodEntry) -> Result<(), DomainError>;
    async fn save_batch(&self, entries: &[MoodEntry]) -> Result<(), DomainError>;
    async fn delete(&self, id: &MoodEntryId) -> Result<(), DomainError>;
    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError>;

    async fn delete_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<u64, DomainError>;

    async fn replace_activity(
        &self,
        user_id: &UserId,
        old_activity_id: &ActivityId,
        new_activity_id: &ActivityId,
    ) -> Result<u64, DomainError>;
}

#[async_trait::async_trait]
pub trait MoodEntryQueryPort: Send + Sync {
    async fn find_by_id(&self, id: &MoodEntryId) -> Result<Option<MoodEntry>, DomainError>;
    async fn find_by_user(
        &self,
        user_id: &UserId,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<MoodEntry>, DomainError>;

    async fn find_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError>;

    async fn find_by_mood(
        &self,
        user_id: &UserId,
        mood: Mood,
    ) -> Result<Vec<MoodEntry>, DomainError>;

    async fn find_by_activity(
        &self,
        user_id: &UserId,
        activity_id: &ActivityId,
    ) -> Result<Vec<MoodEntry>, DomainError>;
}
