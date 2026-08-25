use sqlx::SqlitePool;

use domain::activity::{Activity, ActivityId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::ActivityRow;

pub struct SqliteActivityQueryRepository {
    pool: SqlitePool,
}

impl SqliteActivityQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ActivityQueryPort for SqliteActivityQueryRepository {
    async fn find_by_id(&self, id: &ActivityId) -> Result<Option<Activity>, DomainError> {
        let row = sqlx::query_as::<_, ActivityRow>("SELECT * FROM activities WHERE id = ?")
            .bind(id.value().to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(row.map(ActivityRow::into_domain))
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Activity>, DomainError> {
        let rows = sqlx::query_as::<_, ActivityRow>(
            "SELECT * FROM activities WHERE user_id = ? ORDER BY name",
        )
        .bind(user_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(ActivityRow::into_domain).collect())
    }

    async fn find_active_by_user(&self, user_id: &UserId) -> Result<Vec<Activity>, DomainError> {
        let rows = sqlx::query_as::<_, ActivityRow>(
            "SELECT * FROM activities WHERE user_id = ? AND archived = 0 ORDER BY name",
        )
        .bind(user_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(ActivityRow::into_domain).collect())
    }
}
