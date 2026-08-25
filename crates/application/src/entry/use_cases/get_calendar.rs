use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::NaiveDate;

use domain::entry::{DateRange, Mood, MoodEntry};
use domain::ports::MoodEntryQueryPort;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct CalendarDay {
    pub date: NaiveDate,
    pub entries: Vec<MoodEntry>,
    pub dominant_mood: Option<Mood>,
}

pub struct Deps {
    pub query: Arc<dyn MoodEntryQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    range: DateRange,
    deps: &Deps,
) -> Result<Vec<CalendarDay>, ApplicationError> {
    let entries = deps.query.find_by_date_range(&user_id, &range).await?;

    let mut by_date: BTreeMap<NaiveDate, Vec<MoodEntry>> = BTreeMap::new();
    for entry in entries {
        let date = entry.logged_at().date_naive();
        by_date.entry(date).or_default().push(entry);
    }

    let days = by_date
        .into_iter()
        .map(|(date, entries)| {
            let dominant_mood = find_dominant_mood(&entries);
            CalendarDay {
                date,
                entries,
                dominant_mood,
            }
        })
        .collect();

    Ok(days)
}

fn find_dominant_mood(entries: &[MoodEntry]) -> Option<Mood> {
    if entries.is_empty() {
        return None;
    }

    let mut counts = [0u32; 5];
    for entry in entries {
        counts[entry.mood().value() as usize - 1] += 1;
    }

    let max_index = counts
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(i, _)| i)?;

    Mood::try_from(max_index as u8 + 1).ok()
}
