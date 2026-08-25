use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::ReminderQueryPort;
use domain::reminder::{Reminder, ReminderId};
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ReminderQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    reminder_id: ReminderId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<Reminder, ApplicationError> {
    let reminder = deps
        .query
        .find_by_id(&reminder_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("reminder not found".into()))?;

    verify_ownership(reminder.user_id(), &caller_id)?;
    Ok(reminder)
}
