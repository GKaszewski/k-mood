use crate::errors::DomainError;
use crate::events::EventEnvelope;

#[async_trait::async_trait]
pub trait EventPublisherPort: Send + Sync {
    async fn publish(&self, envelope: EventEnvelope) -> Result<(), DomainError>;
}
