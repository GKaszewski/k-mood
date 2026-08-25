use std::sync::Arc;

use domain::errors::DomainError;
use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{EventPublisherPort, ReminderCommandPort, ReminderQueryPort};
use domain::reminder::Reminder;
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

use super::super::commands::UpdateReminderCommand;

pub struct Deps {
    pub command: Arc<dyn ReminderCommandPort>,
    pub query: Arc<dyn ReminderQueryPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    cmd: UpdateReminderCommand,
    caller_id: UserId,
    deps: &Deps,
) -> Result<Reminder, ApplicationError> {
    let mut reminder = deps
        .query
        .find_by_id(&cmd.reminder_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("reminder not found".into()))?;

    verify_ownership(reminder.user_id(), &caller_id)?;

    if let Some(schedule) = cmd.schedule {
        reminder.update_schedule(schedule);
    }

    if let Some(enabled) = cmd.enabled {
        if enabled {
            reminder.enable();
        } else {
            reminder.disable();
        }
    }

    deps.command.save(&reminder).await?;

    let event = DomainEvent::ReminderUpdated {
        reminder_id: reminder.id().clone(),
        user_id: reminder.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(reminder)
}
