use crate::entry::{Date, DateSpan};
use crate::errors::DomainError;
use crate::metric::{DailyMetric, MetricKind};
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::DailyMetricCommandPort for InMemoryStore {
    async fn save(&self, metrics: &[DailyMetric]) -> Result<usize, DomainError> {
        let mut stored = self.daily_metrics.write().unwrap();
        let mut written = 0;

        for incoming in metrics {
            let existing = stored.iter().position(|held: &DailyMetric| {
                held.user_id() == incoming.user_id()
                    && held.date() == incoming.date()
                    && held.kind() == incoming.kind()
            });

            match existing {
                Some(index) if incoming.supersedes(&stored[index]) => {
                    stored[index] = incoming.clone();
                    written += 1;
                }
                Some(_) => continue,
                None => {
                    stored.push(incoming.clone());
                    written += 1;
                }
            }
        }

        Ok(written)
    }

    async fn delete(
        &self,
        user_id: &UserId,
        date: &Date,
        kinds: &[MetricKind],
    ) -> Result<(), DomainError> {
        self.daily_metrics.write().unwrap().retain(|held| {
            held.user_id() != user_id || held.date() != date || !kinds.contains(&held.kind())
        });

        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::DailyMetricQueryPort for InMemoryStore {
    async fn find_by_span(
        &self,
        user_id: &UserId,
        span: &DateSpan,
    ) -> Result<Vec<DailyMetric>, DomainError> {
        Ok(self
            .daily_metrics
            .read()
            .unwrap()
            .iter()
            .filter(|metric| metric.user_id() == user_id && span.contains(metric.date()))
            .cloned()
            .collect())
    }
}
