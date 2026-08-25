use sqlx::SqlitePool;

use domain::auth::RefreshSession;
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqliteRefreshSessionCommandRepository {
    pool: SqlitePool,
}

impl SqliteRefreshSessionCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::RefreshSessionCommandPort for SqliteRefreshSessionCommandRepository {
    async fn create(&self, session: &RefreshSession) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO refresh_sessions (id, user_id, token, expires_at, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(session.id().value().to_string())
        .bind(session.user_id().value().to_string())
        .bind(session.token())
        .bind(session.expires_at().to_rfc3339())
        .bind(session.created_at().to_rfc3339())
        .execute(&self.pool).await.map_err(db_err)?;
        Ok(())
    }

    async fn revoke(&self, token: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM refresh_sessions WHERE token = ?")
            .bind(token)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM refresh_sessions WHERE user_id = ?")
            .bind(user_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, DomainError> {
        let result = sqlx::query("DELETE FROM refresh_sessions WHERE expires_at < ?")
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(result.rows_affected())
    }
}
