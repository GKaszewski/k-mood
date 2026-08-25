use tokio::sync::mpsc;

use domain::errors::DomainError;
use domain::events::EventEnvelope;

const DEFAULT_CHANNEL_CAPACITY: usize = 256;

pub type EventReceiver = mpsc::Receiver<EventEnvelope>;

pub struct ChannelEventPublisher {
    sender: mpsc::Sender<EventEnvelope>,
}

impl ChannelEventPublisher {
    fn new(sender: mpsc::Sender<EventEnvelope>) -> Self {
        Self { sender }
    }
}

#[async_trait::async_trait]
impl domain::ports::EventPublisherPort for ChannelEventPublisher {
    async fn publish(&self, envelope: EventEnvelope) -> Result<(), DomainError> {
        self.sender
            .send(envelope)
            .await
            .map_err(|_| DomainError::InvalidInput("event channel closed".into()))?;

        Ok(())
    }
}

pub fn create_event_channel() -> (ChannelEventPublisher, EventReceiver) {
    let (sender, receiver) = mpsc::channel(DEFAULT_CHANNEL_CAPACITY);
    (ChannelEventPublisher::new(sender), receiver)
}
