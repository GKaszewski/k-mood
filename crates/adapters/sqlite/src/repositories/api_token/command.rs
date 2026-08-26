use sqlx::SqlitePool;

use domain::api_token::{ApiToken, ApiTokenId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqliteApiTokenCommandRepository {
    pool: SqlitePool,
}

impl SqliteApiTokenCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ApiTokenCommandPort for SqliteApiTokenCommandRepository {
    async fn save(&self, token: &ApiToken) -> Result<(), DomainError> {
        let taken: Option<(String,)> =
            sqlx::query_as("SELECT id FROM api_tokens WHERE user_id = ? AND name = ?")
                .bind(token.user_id().value().to_string())
                .bind(token.name().value())
                .fetch_optional(&self.pool)
                .await
                .map_err(db_err)?;

        if taken.is_some() {
            return Err(DomainError::Conflict(format!(
                "a token named {} already exists",
                token.name().value()
            )));
        }

        sqlx::query(
            "INSERT INTO api_tokens (id, user_id, name, digest, scope, created_at, last_used_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(token.id().value().to_string())
        .bind(token.user_id().value().to_string())
        .bind(token.name().value())
        .bind(token.digest().value())
        .bind(token.scope().name())
        .bind(token.created_at().to_rfc3339())
        .bind(token.last_used_at().map(|used| used.to_rfc3339()))
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }

    async fn revoke(&self, user_id: &UserId, id: &ApiTokenId) -> Result<(), DomainError> {
        let removed = sqlx::query("DELETE FROM api_tokens WHERE id = ? AND user_id = ?")
            .bind(id.value().to_string())
            .bind(user_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        if removed.rows_affected() == 0 {
            return Err(DomainError::NotFound("api token not found".into()));
        }

        Ok(())
    }

    async fn mark_used(&self, id: &ApiTokenId) -> Result<(), DomainError> {
        sqlx::query("UPDATE api_tokens SET last_used_at = ? WHERE id = ?")
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}
