use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::user::{User, UserId};

use super::super::shared::db_err;

pub struct SqliteUserCommandRepository {
    pool: SqlitePool,
}

impl SqliteUserCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::UserCommandPort for SqliteUserCommandRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, display_name, timezone, role, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
                username = excluded.username, email = excluded.email,
                password_hash = excluded.password_hash, display_name = excluded.display_name,
                timezone = excluded.timezone, role = excluded.role,
                updated_at = excluded.updated_at"
        )
        .bind(user.id().value().to_string())
        .bind(user.username().value())
        .bind(user.email().value())
        .bind(user.password_hash().value())
        .bind(user.display_name().map(|d| d.value().to_string()))
        .bind(user.timezone().map(|t| t.value().to_string()))
        .bind(format!("{:?}", user.role()))
        .bind(user.created_at().to_rfc3339())
        .bind(user.updated_at().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}
