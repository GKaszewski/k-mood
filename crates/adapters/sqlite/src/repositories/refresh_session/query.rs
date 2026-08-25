use sqlx::SqlitePool;

use domain::auth::RefreshSession;
use domain::errors::DomainError;

use super::super::shared::db_err;
use super::rows::RefreshSessionRow;

pub struct SqliteRefreshSessionQueryRepository {
    pool: SqlitePool,
}

impl SqliteRefreshSessionQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::RefreshSessionQueryPort for SqliteRefreshSessionQueryRepository {
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshSession>, DomainError> {
        let row = sqlx::query_as::<_, RefreshSessionRow>(
            "SELECT * FROM refresh_sessions WHERE token = ?",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(RefreshSessionRow::into_domain))
    }
}
