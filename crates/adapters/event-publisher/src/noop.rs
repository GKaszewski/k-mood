use domain::errors::DomainError;
use domain::events::EventEnvelope;

pub struct NoopEventPublisher;

#[async_trait::async_trait]
impl domain::ports::EventPublisherPort for NoopEventPublisher {
    async fn publish(&self, _envelope: EventEnvelope) -> Result<(), DomainError> {
        Ok(())
    }
}
