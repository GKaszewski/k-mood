use sqlx::SqlitePool;

use domain::api_token::{ApiToken, TokenDigest};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::{ApiTokenRow, readable};

pub struct SqliteApiTokenQueryRepository {
    pool: SqlitePool,
}

impl SqliteApiTokenQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ApiTokenQueryPort for SqliteApiTokenQueryRepository {
    async fn find_by_digest(&self, digest: &TokenDigest) -> Result<Option<ApiToken>, DomainError> {
        let row: Option<ApiTokenRow> = sqlx::query_as(
            "SELECT id, user_id, name, digest, scopes, created_at, last_used_at
             FROM api_tokens WHERE digest = ?",
        )
        .bind(digest.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.as_ref().and_then(readable))
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ApiToken>, DomainError> {
        let rows: Vec<ApiTokenRow> = sqlx::query_as(
            "SELECT id, user_id, name, digest, scopes, created_at, last_used_at
             FROM api_tokens WHERE user_id = ? ORDER BY created_at DESC",
        )
        .bind(user_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }
}
