use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::push::{PushSubscription, PushSubscriptionId};
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqlitePushSubscriptionCommandRepository {
    pool: SqlitePool,
}

impl SqlitePushSubscriptionCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::PushSubscriptionCommandPort for SqlitePushSubscriptionCommandRepository {
    async fn save(&self, sub: &PushSubscription) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO push_subscriptions (id, user_id, endpoint, p256dh, auth, created_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(endpoint) DO UPDATE SET
                user_id = excluded.user_id,
                p256dh = excluded.p256dh,
                auth = excluded.auth",
        )
        .bind(sub.id().value().to_string())
        .bind(sub.user_id().value().to_string())
        .bind(sub.endpoint())
        .bind(sub.p256dh())
        .bind(sub.auth())
        .bind(sub.created_at().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn delete(&self, id: &PushSubscriptionId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM push_subscriptions WHERE id = ?")
            .bind(id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_by_endpoint(&self, endpoint: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM push_subscriptions WHERE endpoint = ?")
            .bind(endpoint)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM push_subscriptions WHERE user_id = ?")
            .bind(user_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}
