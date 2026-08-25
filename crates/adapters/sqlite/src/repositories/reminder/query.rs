use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::reminder::{Reminder, ReminderId};
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::ReminderRow;

pub struct SqliteReminderQueryRepository {
    pool: SqlitePool,
}

impl SqliteReminderQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ReminderQueryPort for SqliteReminderQueryRepository {
    async fn find_by_id(&self, id: &ReminderId) -> Result<Option<Reminder>, DomainError> {
        let row = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders WHERE id = ?")
            .bind(id.value().to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(row.map(ReminderRow::into_domain))
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Reminder>, DomainError> {
        let rows = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders WHERE user_id = ?")
            .bind(user_id.value().to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(ReminderRow::into_domain).collect())
    }

    async fn find_all_enabled(&self) -> Result<Vec<Reminder>, DomainError> {
        let rows = sqlx::query_as::<_, ReminderRow>("SELECT * FROM reminders WHERE enabled = 1")
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(ReminderRow::into_domain).collect())
    }
}
