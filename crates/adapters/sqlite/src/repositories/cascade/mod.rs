use sqlx::SqlitePool;

use domain::entry::{DateRange, MoodEntry};
use domain::errors::DomainError;
use domain::user::UserId;

use super::entry::rows::{EntryRow, hydrate_batch};
use super::shared::db_err;

pub struct SqliteCascadeDeleteRepository {
    pool: SqlitePool,
}

impl SqliteCascadeDeleteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::CascadeDeletePort for SqliteCascadeDeleteRepository {
    async fn delete_all_user_data(&self, user_id: &UserId) -> Result<(), DomainError> {
        let uid = user_id.value().to_string();
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        sqlx::query("DELETE FROM mood_entries WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM activities WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM reminders WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM daily_metrics WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn delete_user_account(&self, user_id: &UserId) -> Result<(), DomainError> {
        let uid = user_id.value().to_string();
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        sqlx::query("DELETE FROM mood_entries WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM activities WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM reminders WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM refresh_sessions WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM push_subscriptions WHERE user_id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(&uid)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn delete_entries_in_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let uid = user_id.value().to_string();

        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT * FROM mood_entries WHERE user_id = ? AND logged_at >= ? AND logged_at <= ? ORDER BY logged_at DESC",
        )
        .bind(&uid)
        .bind(range.start().to_rfc3339())
        .bind(range.end().to_rfc3339())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let entries = hydrate_batch(&self.pool, rows).await?;

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        sqlx::query(
            "DELETE FROM mood_entries WHERE user_id = ? AND logged_at >= ? AND logged_at <= ?",
        )
        .bind(&uid)
        .bind(range.start().to_rfc3339())
        .bind(range.end().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;

        Ok(entries)
    }
}
