use crate::errors::DomainError;
use crate::rejection::RejectedMetric;
use crate::user::UserId;

#[async_trait::async_trait]
pub trait RejectionCommandPort: Send + Sync {
    async fn record(&self, rejections: &[RejectedMetric]) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait RejectionQueryPort: Send + Sync {
    async fn find_recent_by_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<RejectedMetric>, DomainError>;
}
