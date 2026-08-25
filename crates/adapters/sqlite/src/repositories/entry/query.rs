use sqlx::SqlitePool;

use domain::activity::ActivityId;
use domain::entry::{DateRange, Mood, MoodEntry, MoodEntryId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::{EntryRow, hydrate_batch, hydrate_single};

pub struct SqliteEntryQueryRepository {
    pool: SqlitePool,
}

impl SqliteEntryQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::MoodEntryQueryPort for SqliteEntryQueryRepository {
    async fn find_by_id(&self, id: &MoodEntryId) -> Result<Option<MoodEntry>, DomainError> {
        let row = sqlx::query_as::<_, EntryRow>("SELECT * FROM mood_entries WHERE id = ?")
            .bind(id.value().to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;

        match row {
            Some(r) => Ok(Some(hydrate_single(&self.pool, r).await?)),
            None => Ok(None),
        }
    }

    async fn find_by_user(
        &self,
        user_id: &UserId,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let limit = limit.unwrap_or(i64::MAX);
        let offset = offset.unwrap_or(0);

        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT * FROM mood_entries WHERE user_id = ? ORDER BY logged_at DESC LIMIT ? OFFSET ?",
        )
        .bind(user_id.value().to_string())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }

    async fn find_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT * FROM mood_entries WHERE user_id = ? AND logged_at >= ? AND logged_at <= ? ORDER BY logged_at DESC",
        )
        .bind(user_id.value().to_string())
        .bind(range.start().to_rfc3339())
        .bind(range.end().to_rfc3339())
        .fetch_all(&self.pool).await.map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }

    async fn find_by_mood(
        &self,
        user_id: &UserId,
        mood: Mood,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT * FROM mood_entries WHERE user_id = ? AND mood = ? ORDER BY logged_at DESC",
        )
        .bind(user_id.value().to_string())
        .bind(mood.value() as i32)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }

    async fn find_by_activity(
        &self,
        user_id: &UserId,
        activity_id: &ActivityId,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT me.* FROM mood_entries me
             INNER JOIN entry_activities ea ON ea.entry_id = me.id
             WHERE me.user_id = ? AND ea.activity_id = ?
             ORDER BY me.logged_at DESC",
        )
        .bind(user_id.value().to_string())
        .bind(activity_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }
}
