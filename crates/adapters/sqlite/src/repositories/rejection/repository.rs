use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::rejection::RejectedMetric;
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::{RejectionRow, readable};

pub struct SqliteRejectionRepository {
    pool: SqlitePool,
    kept_per_user: usize,
}

impl SqliteRejectionRepository {
    pub fn new(pool: SqlitePool, kept_per_user: usize) -> Self {
        Self {
            pool,
            kept_per_user,
        }
    }
}

#[async_trait::async_trait]
impl domain::ports::RejectionCommandPort for SqliteRejectionRepository {
    async fn record(&self, rejections: &[RejectedMetric]) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        for rejected in rejections {
            sqlx::query(
                "INSERT INTO metric_rejections
                 (id, user_id, origin, provider, date, kind, value, reason, recorded_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(rejected.id().value().to_string())
            .bind(rejected.user_id().value().to_string())
            .bind(rejected.origin().name())
            .bind(rejected.detail().provider().map(|name| name.value()))
            .bind(rejected.detail().date().map(|date| date.to_string()))
            .bind(rejected.detail().kind())
            .bind(rejected.detail().value())
            .bind(rejected.reason())
            .bind(rejected.recorded_at().to_rfc3339())
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        }

        for owner in owners(rejections) {
            sqlx::query(
                "DELETE FROM metric_rejections WHERE user_id = ? AND id NOT IN (
                     SELECT id FROM metric_rejections WHERE user_id = ?
                     ORDER BY recorded_at DESC, id DESC LIMIT ?
                 )",
            )
            .bind(&owner)
            .bind(&owner)
            .bind(self.kept_per_user as i64)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        }

        tx.commit().await.map_err(db_err)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl domain::ports::RejectionQueryPort for SqliteRejectionRepository {
    async fn find_recent_by_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<RejectedMetric>, DomainError> {
        let rows: Vec<RejectionRow> = sqlx::query_as(
            "SELECT id, user_id, origin, provider, date, kind, value, reason, recorded_at
             FROM metric_rejections WHERE user_id = ?
             ORDER BY recorded_at DESC, id DESC LIMIT ?",
        )
        .bind(user_id.value().to_string())
        .bind(self.kept_per_user as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }
}

fn owners(rejections: &[RejectedMetric]) -> Vec<String> {
    let mut owners: Vec<String> = rejections
        .iter()
        .map(|rejected| rejected.user_id().value().to_string())
        .collect();
    owners.sort();
    owners.dedup();

    owners
}
