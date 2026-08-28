use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeDelta, Utc};

use crate::user::Timezone;

use super::DaySchedule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DueOccurrence {
    scheduled_for: DateTime<Utc>,
}

impl DueOccurrence {
    pub fn scheduled_for(&self) -> DateTime<Utc> {
        self.scheduled_for
    }
}

pub fn occurrence_reached(
    schedule: &DaySchedule,
    last_sent_at: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
    timezone: &Timezone,
    grace: Duration,
) -> Option<DueOccurrence> {
    let local_today = now.with_timezone(&timezone.resolve()).date_naive();

    [local_today, local_today.pred_opt()?]
        .into_iter()
        .filter_map(|local_date| scheduled_instant(schedule, local_date, timezone))
        .filter(|scheduled| reached_but_not_stale(*scheduled, now, grace))
        .filter(|scheduled| not_already_sent(*scheduled, last_sent_at))
        .max()
        .map(|scheduled_for| DueOccurrence { scheduled_for })
}

fn scheduled_instant(
    schedule: &DaySchedule,
    local_date: NaiveDate,
    timezone: &Timezone,
) -> Option<DateTime<Utc>> {
    let time = schedule.time_for(local_date.weekday())?;

    placed_in(local_date, time, timezone)
}

fn placed_in(local_date: NaiveDate, time: NaiveTime, timezone: &Timezone) -> Option<DateTime<Utc>> {
    local_date
        .and_time(time)
        .and_local_timezone(timezone.resolve())
        .earliest()
        .map(|placed| placed.with_timezone(&Utc))
}

fn reached_but_not_stale(scheduled: DateTime<Utc>, now: DateTime<Utc>, grace: Duration) -> bool {
    let since = now.signed_duration_since(scheduled);

    since >= TimeDelta::zero() && since < grace
}

fn not_already_sent(scheduled: DateTime<Utc>, last_sent_at: Option<DateTime<Utc>>) -> bool {
    last_sent_at.is_none_or(|sent| sent < scheduled)
}
