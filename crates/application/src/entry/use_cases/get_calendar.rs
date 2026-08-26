use std::collections::BTreeMap;
use std::sync::Arc;

use domain::cycle::CycleDay;
use domain::dimension::ComposedEntry;
use domain::entry::{Date, DateRange, DayMood};
use domain::ports::{
    CycleStartQueryPort, EntryDimensionPort, MoodEntryQueryPort, UserPreferencesQueryPort,
    UserQueryPort,
};
use domain::user::UserId;

use crate::cycle::use_cases::read_cycle::calendar_of;
use crate::day::{date_of, timezone_for};
use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

pub struct CalendarDay {
    pub date: Date,
    pub entries: Vec<ComposedEntry>,
    pub day_mood: Option<DayMood>,
    pub cycle_day: Option<CycleDay>,
}

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub cycles: Arc<dyn CycleStartQueryPort>,
    pub preferences: Arc<dyn UserPreferencesQueryPort>,
    pub users: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    range: DateRange,
    deps: &Deps,
) -> Result<Vec<CalendarDay>, ApplicationError> {
    let entries = deps.query.find_by_date_range(&user_id, &range).await?;
    let entries = EntryComposer::new(deps.dimensions.clone())
        .compose(entries)
        .await?;

    let timezone = timezone_for(&user_id, &deps.users).await?;

    let mut by_date: BTreeMap<Date, Vec<ComposedEntry>> = BTreeMap::new();
    for entry in entries {
        let date = date_of(&entry.entry, &timezone);
        by_date.entry(date).or_default().push(entry);
    }

    let cycles = tracked_cycle(&user_id, deps).await?;

    let days = by_date
        .into_iter()
        .map(|(date, entries)| {
            let day_mood = day_mood_of(&entries);
            let cycle_day = cycles
                .as_ref()
                .and_then(|calendar| calendar.position_on(&date))
                .map(|position| position.day());

            CalendarDay {
                date,
                entries,
                day_mood,
                cycle_day,
            }
        })
        .collect();

    Ok(days)
}

async fn tracked_cycle(
    user_id: &domain::user::UserId,
    deps: &Deps,
) -> Result<Option<domain::cycle::CycleCalendar>, ApplicationError> {
    let preferences = preferences_of(user_id, &deps.preferences).await?;

    if !preferences.tracks_cycle() {
        return Ok(None);
    }

    Ok(Some(calendar_of(user_id, &deps.cycles).await?))
}

fn day_mood_of(entries: &[ComposedEntry]) -> Option<DayMood> {
    let moods: Vec<_> = entries.iter().map(|entry| entry.entry.mood()).collect();

    DayMood::of(&moods)
}
