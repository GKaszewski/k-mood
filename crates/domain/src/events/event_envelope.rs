use chrono::{DateTime, Utc};

use super::{DomainEvent, EventId};

#[derive(Debug, Clone)]
pub struct EventEnvelope {
    id: EventId,
    event: DomainEvent,
    occurred_at: DateTime<Utc>,
}

impl EventEnvelope {
    pub fn wrap(event: DomainEvent) -> Self {
        Self {
            id: EventId::generate(),
            event,
            occurred_at: Utc::now(),
        }
    }
}

impl EventEnvelope {
    pub fn id(&self) -> &EventId {
        &self.id
    }

    pub fn event(&self) -> &DomainEvent {
        &self.event
    }

    pub fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
}
