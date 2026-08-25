use chrono::Weekday;
use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::reminder::{Reminder, ReminderId};
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::format_time;

pub struct SqliteReminderCommandRepository {
    pool: SqlitePool,
}

impl SqliteReminderCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ReminderCommandPort for SqliteReminderCommandRepository {
    async fn save(&self, reminder: &Reminder) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO reminders (id, user_id, monday, tuesday, wednesday, thursday, friday, saturday, sunday, enabled, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                monday = excluded.monday, tuesday = excluded.tuesday,
                wednesday = excluded.wednesday, thursday = excluded.thursday,
                friday = excluded.friday, saturday = excluded.saturday,
                sunday = excluded.sunday, enabled = excluded.enabled"
        )
        .bind(reminder.id().value().to_string())
        .bind(reminder.user_id().value().to_string())
        .bind(reminder.schedule().time_for(Weekday::Mon).map(format_time))
        .bind(reminder.schedule().time_for(Weekday::Tue).map(format_time))
        .bind(reminder.schedule().time_for(Weekday::Wed).map(format_time))
        .bind(reminder.schedule().time_for(Weekday::Thu).map(format_time))
        .bind(reminder.schedule().time_for(Weekday::Fri).map(format_time))
        .bind(reminder.schedule().time_for(Weekday::Sat).map(format_time))
        .bind(reminder.schedule().time_for(Weekday::Sun).map(format_time))
        .bind(reminder.is_enabled())
        .bind(reminder.created_at().to_rfc3339())
        .execute(&self.pool).await.map_err(db_err)?;
        Ok(())
    }

    async fn delete(&self, id: &ReminderId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM reminders WHERE id = ?")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM reminders WHERE user_id = ?")
            .bind(user_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}
