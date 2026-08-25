use std::sync::Arc;

use domain::events::{DomainEvent, EventEnvelope};
use domain::ports::{EventPublisherPort, ReminderCommandPort};
use domain::reminder::Reminder;

use crate::errors::ApplicationError;

use super::super::commands::CreateReminderCommand;

pub struct Deps {
    pub reminders: Arc<dyn ReminderCommandPort>,
    pub events: Arc<dyn EventPublisherPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    cmd: CreateReminderCommand,
    deps: &Deps,
) -> Result<Reminder, ApplicationError> {
    let reminder = Reminder::new(cmd.user_id, cmd.schedule);

    deps.reminders.save(&reminder).await?;

    let event = DomainEvent::ReminderCreated {
        reminder_id: reminder.id().clone(),
        user_id: reminder.user_id().clone(),
    };
    deps.events.publish(EventEnvelope::wrap(event)).await?;

    Ok(reminder)
}
