use domain::reminder::{DaySchedule, ReminderId};
use domain::user::UserId;

#[derive(Debug)]
pub struct CreateReminderCommand {
    pub user_id: UserId,
    pub schedule: DaySchedule,
}

#[derive(Debug)]
pub struct UpdateReminderCommand {
    pub reminder_id: ReminderId,
    pub schedule: Option<DaySchedule>,
    pub enabled: Option<bool>,
}
