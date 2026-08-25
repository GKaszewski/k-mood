use sqlx::SqlitePool;

use domain::activity::{Activity, ActivityId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqliteActivityCommandRepository {
    pool: SqlitePool,
}

impl SqliteActivityCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ActivityCommandPort for SqliteActivityCommandRepository {
    async fn save(&self, activity: &Activity) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO activities (id, user_id, name, category, archived, created_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name, category = excluded.category,
                archived = excluded.archived",
        )
        .bind(activity.id().value().to_string())
        .bind(activity.user_id().value().to_string())
        .bind(activity.name().value())
        .bind(activity.category().map(|c| c.value().to_string()))
        .bind(activity.is_archived())
        .bind(activity.created_at().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn delete(&self, id: &ActivityId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM activities WHERE id = ?")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM activities WHERE user_id = ?")
            .bind(user_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}
