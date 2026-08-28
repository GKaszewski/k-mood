use std::sync::Arc;

use chrono::{Duration, Utc};

use domain::ports::{ReminderCommandPort, ReminderQueryPort, ReminderSenderPort, UserQueryPort};
use domain::reminder::{DueOccurrence, Reminder};
use domain::user::Timezone;

use crate::errors::ApplicationError;

pub struct Deps {
    pub reminder_query: Arc<dyn ReminderQueryPort>,
    pub reminder_command: Arc<dyn ReminderCommandPort>,
    pub user_query: Arc<dyn UserQueryPort>,
    pub sender: Arc<dyn ReminderSenderPort>,
    pub grace: Duration,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(deps: &Deps) -> Result<u64, ApplicationError> {
    let reminders = deps.reminder_query.find_all_enabled().await?;
    tracing::debug!(
        reminder_count = reminders.len(),
        "checking enabled reminders"
    );

    let now = Utc::now();
    let mut sent_count = 0u64;

    for reminder in reminders {
        let Some(timezone) = timezone_of(&reminder, deps).await else {
            continue;
        };

        let Some(occurrence) = reminder.occurrence_reached(now, &timezone, deps.grace) else {
            continue;
        };

        if send(reminder, occurrence, deps).await {
            sent_count += 1;
        }
    }

    tracing::info!(sent_count, "reminder processing completed");

    Ok(sent_count)
}

async fn send(mut reminder: Reminder, occurrence: DueOccurrence, deps: &Deps) -> bool {
    tracing::info!(user_id = %reminder.user_id(), "sending reminder");

    if let Err(error) = deps.sender.send_reminder(reminder.user_id()).await {
        tracing::warn!(user_id = %reminder.user_id(), %error, "could not send reminder");

        return false;
    }

    reminder.mark_sent(occurrence);

    if let Err(error) = deps.reminder_command.save(&reminder).await {
        tracing::error!(
            reminder_id = %reminder.id(),
            %error,
            "sent a reminder but could not record it, so it may be sent again"
        );
    }

    true
}

async fn timezone_of(reminder: &Reminder, deps: &Deps) -> Option<Timezone> {
    let found = deps.user_query.find_by_id(reminder.user_id()).await;

    let user = match found {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::warn!(user_id = %reminder.user_id(), "reminder references nonexistent user");

            return None;
        }
        Err(error) => {
            tracing::warn!(
                user_id = %reminder.user_id(),
                %error,
                "could not read the account behind a reminder, so the rest of the sweep goes on"
            );

            return None;
        }
    };

    if user.timezone().is_none() {
        tracing::debug!(user_id = %reminder.user_id(), "user has no timezone set, skipping reminder");
    }

    user.timezone().copied()
}
