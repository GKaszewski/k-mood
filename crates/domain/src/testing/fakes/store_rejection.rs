use crate::errors::DomainError;
use crate::rejection::RejectedMetric;
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::RejectionCommandPort for InMemoryStore {
    async fn record(&self, rejections: &[RejectedMetric]) -> Result<(), DomainError> {
        self.rejections
            .write()
            .unwrap()
            .extend(rejections.iter().cloned());

        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::RejectionQueryPort for InMemoryStore {
    async fn find_recent_by_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<RejectedMetric>, DomainError> {
        Ok(self
            .rejections
            .read()
            .unwrap()
            .iter()
            .filter(|rejection| rejection.user_id() == user_id)
            .cloned()
            .collect())
    }
}
