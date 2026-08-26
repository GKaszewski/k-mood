use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::provider::{ProviderConnection, ProviderName};
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqliteProviderConnectionCommandRepository {
    pool: SqlitePool,
}

impl SqliteProviderConnectionCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ProviderConnectionCommandPort for SqliteProviderConnectionCommandRepository {
    async fn save(&self, connection: &ProviderConnection) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO provider_connections (id, user_id, provider, credential, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(user_id, provider) DO UPDATE SET
                credential = excluded.credential, updated_at = excluded.updated_at",
        )
        .bind(connection.id().value().to_string())
        .bind(connection.user_id().value().to_string())
        .bind(connection.provider().value())
        .bind(connection.credential().value())
        .bind(connection.created_at().to_rfc3339())
        .bind(connection.updated_at().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId, provider: &ProviderName) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM provider_connections WHERE user_id = ? AND provider = ?")
            .bind(user_id.value().to_string())
            .bind(provider.value())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}
