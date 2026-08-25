use chrono::{NaiveTime, Weekday};

use domain::reminder::{DaySchedule, Reminder, ReminderId};
use domain::user::UserId;

use application::reminder::commands::{CreateReminderCommand, UpdateReminderCommand};

use crate::errors::ApiValidationError;
use crate::requests::{CreateReminderRequest, UpdateReminderRequest};
use crate::responses::{DayScheduleResponse, ReminderResponse};

impl CreateReminderRequest {
    pub fn into_command(
        self,
        user_id: UserId,
    ) -> Result<CreateReminderCommand, ApiValidationError> {
        let schedule = self.into_schedule()?;

        Ok(CreateReminderCommand { user_id, schedule })
    }

    pub fn into_schedule(self) -> Result<DaySchedule, ApiValidationError> {
        let mut schedule = DaySchedule::new();

        set_day(&mut schedule, Weekday::Mon, &self.monday)?;
        set_day(&mut schedule, Weekday::Tue, &self.tuesday)?;
        set_day(&mut schedule, Weekday::Wed, &self.wednesday)?;
        set_day(&mut schedule, Weekday::Thu, &self.thursday)?;
        set_day(&mut schedule, Weekday::Fri, &self.friday)?;
        set_day(&mut schedule, Weekday::Sat, &self.saturday)?;
        set_day(&mut schedule, Weekday::Sun, &self.sunday)?;

        Ok(schedule)
    }
}

impl UpdateReminderRequest {
    pub fn into_command(
        self,
        reminder_id: ReminderId,
    ) -> Result<UpdateReminderCommand, ApiValidationError> {
        let schedule = self.schedule.map(|s| s.into_schedule()).transpose()?;

        Ok(UpdateReminderCommand {
            reminder_id,
            schedule,
            enabled: self.enabled,
        })
    }
}

impl From<Reminder> for ReminderResponse {
    fn from(reminder: Reminder) -> Self {
        Self {
            id: reminder.id().value(),
            schedule: DayScheduleResponse::from(reminder.schedule()),
            enabled: reminder.is_enabled(),
            created_at: *reminder.created_at(),
        }
    }
}

impl From<&DaySchedule> for DayScheduleResponse {
    fn from(schedule: &DaySchedule) -> Self {
        Self {
            monday: schedule.time_for(Weekday::Mon).map(format_time),
            tuesday: schedule.time_for(Weekday::Tue).map(format_time),
            wednesday: schedule.time_for(Weekday::Wed).map(format_time),
            thursday: schedule.time_for(Weekday::Thu).map(format_time),
            friday: schedule.time_for(Weekday::Fri).map(format_time),
            saturday: schedule.time_for(Weekday::Sat).map(format_time),
            sunday: schedule.time_for(Weekday::Sun).map(format_time),
        }
    }
}

fn set_day(
    schedule: &mut DaySchedule,
    day: Weekday,
    time_str: &Option<String>,
) -> Result<(), ApiValidationError> {
    if let Some(s) = time_str {
        let time = parse_time(s)?;
        schedule.set_time(day, Some(time));
    }
    Ok(())
}

fn parse_time(s: &str) -> Result<NaiveTime, ApiValidationError> {
    NaiveTime::parse_from_str(s, "%H:%M")
        .map_err(|e| ApiValidationError::Invalid(format!("invalid time format: {e}")))
}

fn format_time(time: NaiveTime) -> String {
    time.format("%H:%M").to_string()
}
