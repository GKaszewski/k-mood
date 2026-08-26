use sqlx::SqlitePool;

use domain::entry::Date;
use domain::errors::DomainError;
use domain::metric::{DailyMetric, MetricKind};
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::{DailyMetricRow, provider_column, source_of};

pub struct SqliteDailyMetricCommandRepository {
    pool: SqlitePool,
}

impl SqliteDailyMetricCommandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::DailyMetricCommandPort for SqliteDailyMetricCommandRepository {
    async fn save(&self, metrics: &[DailyMetric]) -> Result<usize, DomainError> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let mut written = 0;

        for metric in metrics {
            let user_id = metric.user_id().value().to_string();
            let date = metric.date().to_string();
            let kind = metric.kind().name();

            let stored: Option<DailyMetricRow> = sqlx::query_as(
                "SELECT user_id, date, kind, value, provider FROM daily_metrics
                 WHERE user_id = ? AND date = ? AND kind = ?",
            )
            .bind(&user_id)
            .bind(&date)
            .bind(kind)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db_err)?;

            if let Some(row) = &stored
                && !source_of(row).is_superseded_by(metric.source())
            {
                continue;
            }

            sqlx::query(
                "INSERT INTO daily_metrics (user_id, date, kind, value, provider)
                 VALUES (?, ?, ?, ?, ?)
                 ON CONFLICT(user_id, date, kind) DO UPDATE SET
                    value = excluded.value, provider = excluded.provider",
            )
            .bind(&user_id)
            .bind(&date)
            .bind(kind)
            .bind(metric.value().count())
            .bind(provider_column(metric.source()))
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

            written += 1;
        }

        tx.commit().await.map_err(db_err)?;

        Ok(written)
    }

    async fn delete(
        &self,
        user_id: &UserId,
        date: &Date,
        kinds: &[MetricKind],
    ) -> Result<(), DomainError> {
        if kinds.is_empty() {
            return Ok(());
        }

        let placeholders = vec!["?"; kinds.len()].join(",");
        let sql = format!(
            "DELETE FROM daily_metrics WHERE user_id = ? AND date = ? AND kind IN ({placeholders})"
        );

        let mut query = sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(user_id.value().to_string())
            .bind(date.to_string());

        for kind in kinds {
            query = query.bind(kind.name());
        }

        query.execute(&self.pool).await.map_err(db_err)?;

        Ok(())
    }
}
