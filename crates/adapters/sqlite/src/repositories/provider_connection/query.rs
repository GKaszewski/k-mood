use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::provider::{ProviderConnection, ProviderName};
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::{ProviderConnectionRow, row_to_connection};

pub struct SqliteProviderConnectionQueryRepository {
    pool: SqlitePool,
}

impl SqliteProviderConnectionQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::ProviderConnectionQueryPort for SqliteProviderConnectionQueryRepository {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ProviderConnection>, DomainError> {
        let rows = sqlx::query_as::<_, ProviderConnectionRow>(
            "SELECT * FROM provider_connections WHERE user_id = ? ORDER BY provider",
        )
        .bind(user_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        rows.into_iter().map(row_to_connection).collect()
    }

    async fn find_by_user_and_provider(
        &self,
        user_id: &UserId,
        provider: &ProviderName,
    ) -> Result<Option<ProviderConnection>, DomainError> {
        let row = sqlx::query_as::<_, ProviderConnectionRow>(
            "SELECT * FROM provider_connections WHERE user_id = ? AND provider = ?",
        )
        .bind(user_id.value().to_string())
        .bind(provider.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        row.map(row_to_connection).transpose()
    }
}
