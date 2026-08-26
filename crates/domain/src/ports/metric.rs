use crate::entry::{Date, DateSpan};
use crate::errors::DomainError;
use crate::metric::{DailyMetric, MetricKind};
use crate::user::UserId;

#[async_trait::async_trait]
pub trait DailyMetricCommandPort: Send + Sync {
    async fn save(&self, metrics: &[DailyMetric]) -> Result<usize, DomainError>;

    async fn delete(
        &self,
        user_id: &UserId,
        date: &Date,
        kinds: &[MetricKind],
    ) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait DailyMetricQueryPort: Send + Sync {
    async fn find_by_span(
        &self,
        user_id: &UserId,
        span: &DateSpan,
    ) -> Result<Vec<DailyMetric>, DomainError>;
}
