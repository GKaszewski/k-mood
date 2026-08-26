use std::sync::Arc;

use sqlx::SqlitePool;

use domain::entry::DateSpan;
use domain::errors::DomainError;
use domain::metric::DailyMetric;
use domain::ports::RejectionCommandPort;
use domain::rejection::{RejectedMetric, RejectionDetail, RejectionOrigin};
use domain::user::UserId;

use super::super::shared::db_err;
use super::rows::{DailyMetricRow, row_to_metric};

pub struct SqliteDailyMetricQueryRepository {
    pool: SqlitePool,
    rejections: Arc<dyn RejectionCommandPort>,
}

impl SqliteDailyMetricQueryRepository {
    pub fn new(pool: SqlitePool, rejections: Arc<dyn RejectionCommandPort>) -> Self {
        Self { pool, rejections }
    }
}

#[async_trait::async_trait]
impl domain::ports::DailyMetricQueryPort for SqliteDailyMetricQueryRepository {
    async fn find_by_span(
        &self,
        user_id: &UserId,
        span: &DateSpan,
    ) -> Result<Vec<DailyMetric>, DomainError> {
        let rows: Vec<DailyMetricRow> = sqlx::query_as(
            "SELECT user_id, date, kind, value, provider FROM daily_metrics
             WHERE user_id = ? AND date >= ? AND date <= ?
             ORDER BY date, kind",
        )
        .bind(user_id.value().to_string())
        .bind(span.start().to_string())
        .bind(span.end().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let mut readable = Vec::with_capacity(rows.len());
        let mut unreadable = Vec::new();

        for row in &rows {
            match row_to_metric(row) {
                Some(metric) => readable.push(metric),
                None => unreadable.push(unreadable_row(user_id, row)),
            }
        }

        self.trace(&unreadable).await;

        Ok(readable)
    }
}

impl SqliteDailyMetricQueryRepository {
    async fn trace(&self, unreadable: &[RejectedMetric]) {
        if unreadable.is_empty() {
            return;
        }

        tracing::warn!(
            count = unreadable.len(),
            "skipped stored metrics this build cannot read"
        );

        if let Err(error) = self.rejections.record(unreadable).await {
            tracing::warn!(%error, "could not write to the rejection trace");
        }
    }
}

fn unreadable_row(user_id: &UserId, row: &DailyMetricRow) -> RejectedMetric {
    let date = row
        .date
        .parse()
        .ok()
        .map(domain::entry::Date::from_persistence);

    RejectedMetric::new(
        user_id.clone(),
        RejectionOrigin::StoredRow,
        RejectionDetail::new(
            row.provider
                .clone()
                .map(domain::provider::ProviderName::from_persistence),
            date,
            row.kind.clone(),
            Some(row.value),
        ),
        "this reading is stored but cannot be read back by this build",
    )
}
