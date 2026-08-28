use sqlx::SqlitePool;

use domain::activity::ActivityId;
use domain::entry::{DateRange, MoodEntry, MoodEntryId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::{db_err, sortable_instant};

pub struct SqliteEntryCommandRepository {
    pool: SqlitePool,
}

impl SqliteEntryCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::MoodEntryCommandPort for SqliteEntryCommandRepository {
    async fn save(&self, entry: &MoodEntry) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO mood_entries (id, user_id, mood, logged_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                mood = excluded.mood, logged_at = excluded.logged_at,
                updated_at = excluded.updated_at",
        )
        .bind(entry.id().value().to_string())
        .bind(entry.user_id().value().to_string())
        .bind(entry.mood().value() as i32)
        .bind(sortable_instant(entry.logged_at()))
        .bind(entry.created_at().to_rfc3339())
        .bind(entry.updated_at().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }

    async fn save_batch(&self, entries: &[MoodEntry]) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        for entry in entries {
            let entry_id = entry.id().value().to_string();

            sqlx::query(
                "INSERT INTO mood_entries (id, user_id, mood, logged_at, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?)
                 ON CONFLICT(id) DO UPDATE SET
                    mood = excluded.mood, logged_at = excluded.logged_at,
                    updated_at = excluded.updated_at",
            )
            .bind(&entry_id)
            .bind(entry.user_id().value().to_string())
            .bind(entry.mood().value() as i32)
            .bind(sortable_instant(entry.logged_at()))
            .bind(entry.created_at().to_rfc3339())
            .bind(entry.updated_at().to_rfc3339())
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        }

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn delete(&self, id: &MoodEntryId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM mood_entries WHERE id = ?")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM mood_entries WHERE user_id = ?")
            .bind(user_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<u64, DomainError> {
        let result = sqlx::query(
            "DELETE FROM mood_entries WHERE user_id = ? AND logged_at >= ? AND logged_at <= ?",
        )
        .bind(user_id.value().to_string())
        .bind(sortable_instant(range.start()))
        .bind(sortable_instant(range.end()))
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(result.rows_affected())
    }

    async fn replace_activity(
        &self,
        user_id: &UserId,
        old_activity_id: &ActivityId,
        new_activity_id: &ActivityId,
    ) -> Result<u64, DomainError> {
        let result = sqlx::query(
            "UPDATE entry_activities SET activity_id = ?
             WHERE activity_id = ? AND entry_id IN (SELECT id FROM mood_entries WHERE user_id = ?)",
        )
        .bind(new_activity_id.value().to_string())
        .bind(old_activity_id.value().to_string())
        .bind(user_id.value().to_string())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(result.rows_affected())
    }
}
