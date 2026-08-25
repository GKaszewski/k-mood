use std::sync::Arc;

use chrono::{Datelike, NaiveTime, Utc};

use domain::ports::{ReminderQueryPort, ReminderSenderPort, UserQueryPort};
use domain::reminder::Reminder;

use crate::errors::ApplicationError;

pub struct Deps {
    pub reminder_query: Arc<dyn ReminderQueryPort>,
    pub user_query: Arc<dyn UserQueryPort>,
    pub sender: Arc<dyn ReminderSenderPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(deps: &Deps) -> Result<u64, ApplicationError> {
    let reminders = deps.reminder_query.find_all_enabled().await?;
    tracing::debug!(
        reminder_count = reminders.len(),
        "checking enabled reminders"
    );

    let mut sent_count = 0u64;

    for reminder in &reminders {
        if should_send(reminder, &deps.user_query).await? {
            tracing::info!(user_id = %reminder.user_id(), "sending reminder");
            deps.sender.send_reminder(reminder.user_id()).await?;
            sent_count += 1;
        }
    }

    tracing::info!(sent_count, "reminder processing completed");
    Ok(sent_count)
}

async fn should_send(
    reminder: &Reminder,
    user_query: &Arc<dyn UserQueryPort>,
) -> Result<bool, ApplicationError> {
    let user = user_query.find_by_id(reminder.user_id()).await?;
    let user = match user {
        Some(u) => u,
        None => {
            tracing::warn!(user_id = %reminder.user_id(), "reminder references nonexistent user");
            return Ok(false);
        }
    };

    let now = match user.timezone() {
        Some(tz) => {
            let tz: chrono_tz::Tz = match tz.value().parse() {
                Ok(tz) => tz,
                Err(_) => {
                    tracing::warn!(
                        user_id = %reminder.user_id(),
                        timezone = tz.value(),
                        "invalid timezone for user"
                    );
                    return Ok(false);
                }
            };
            Utc::now().with_timezone(&tz)
        }
        None => {
            tracing::debug!(user_id = %reminder.user_id(), "user has no timezone set, skipping reminder");
            return Ok(false);
        }
    };

    let weekday = now.weekday();
    let scheduled_time = match reminder.schedule().time_for(weekday) {
        Some(time) => time,
        None => return Ok(false),
    };

    let current_time = now.time();
    Ok(is_within_window(current_time, scheduled_time))
}

fn is_within_window(current: NaiveTime, scheduled: NaiveTime) -> bool {
    let diff = current.signed_duration_since(scheduled);
    let minutes = diff.num_minutes();
    (0..5).contains(&minutes)
}
