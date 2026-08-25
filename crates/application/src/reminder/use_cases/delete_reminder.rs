use std::sync::Arc;

use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{EventPublisherPort, ReminderCommandPort, ReminderQueryPort};
use domain::reminder::ReminderId;
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub command: Arc<dyn ReminderCommandPort>,
    pub query: Arc<dyn ReminderQueryPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    reminder_id: ReminderId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let reminder = deps
        .query
        .find_by_id(&reminder_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("reminder not found".into()))?;

    verify_ownership(reminder.user_id(), &caller_id)?;

    deps.command.delete(&reminder_id).await?;

    let event = DomainEvent::ReminderDeleted {
        reminder_id,
        user_id: reminder.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(())
}
