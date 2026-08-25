use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::push::PushSubscription;
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::PushSubscriptionRow;

pub struct SqlitePushSubscriptionQueryRepository {
    pool: SqlitePool,
}

impl SqlitePushSubscriptionQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::PushSubscriptionQueryPort for SqlitePushSubscriptionQueryRepository {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<PushSubscription>, DomainError> {
        let rows = sqlx::query_as::<_, PushSubscriptionRow>(
            "SELECT * FROM push_subscriptions WHERE user_id = ?",
        )
        .bind(user_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows.into_iter().map(|r| r.into_entity()).collect())
    }

    async fn find_by_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Option<PushSubscription>, DomainError> {
        let row = sqlx::query_as::<_, PushSubscriptionRow>(
            "SELECT * FROM push_subscriptions WHERE endpoint = ?",
        )
        .bind(endpoint)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(|r| r.into_entity()))
    }
}
