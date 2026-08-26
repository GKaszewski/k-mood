use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset, NaiveTime, Utc};

use domain::entry::{Date, DateRange, DateSpan, DayMood, MoodEntry};
use domain::errors::DomainError;
use domain::ports::UserQueryPort;
use domain::user::{Timezone, UserId};

use crate::errors::ApplicationError;

pub async fn timezone_for(
    user_id: &UserId,
    users: &Arc<dyn UserQueryPort>,
) -> Result<Timezone, ApplicationError> {
    let user = users
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("user not found".into()))?;

    user.timezone().copied().ok_or_else(|| {
        DomainError::InvalidInput(
            "this account has no timezone set, so days cannot be resolved: set one in settings"
                .into(),
        )
        .into()
    })
}

pub fn today_in(timezone: &Timezone) -> Date {
    Date::from_instant(&Utc::now().fixed_offset(), timezone)
}

pub fn date_of(entry: &MoodEntry, timezone: &Timezone) -> Date {
    Date::from_instant(entry.logged_at(), timezone)
}

pub fn day_moods(entries: &[MoodEntry], timezone: &Timezone) -> BTreeMap<Date, DayMood> {
    let mut moods: BTreeMap<Date, Vec<domain::entry::Mood>> = BTreeMap::new();

    for entry in entries {
        moods
            .entry(date_of(entry, timezone))
            .or_default()
            .push(entry.mood());
    }

    moods
        .into_iter()
        .filter_map(|(date, moods)| DayMood::of(&moods).map(|mood| (date, mood)))
        .collect()
}

pub fn instants_of(span: &DateSpan) -> DateRange {
    let from = midnight_utc(&span.start().previous());
    let to = midnight_utc(&span.end().next());

    DateRange::new(from, to).unwrap_or_else(|_| {
        DateRange::new(from, from).expect("a range of one instant is always valid")
    })
}

fn midnight_utc(date: &Date) -> DateTime<FixedOffset> {
    date.value()
        .and_time(NaiveTime::MIN)
        .and_utc()
        .fixed_offset()
}
