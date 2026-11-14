use std::collections::BTreeMap;
use std::sync::Arc;

use domain::entry::Date;
use domain::ports::{MoodEntryQueryPort, UserQueryPort};
use domain::trend::{CalendarBucketing, Granularity, TrendBucketing, TrendDay, TrendPoint};

use crate::day::{date_of, day_moods, instants_of, timezone_for};
use crate::errors::ApplicationError;

use super::super::queries::MoodTrendQuery;

pub struct MoodTrend {
    pub granularity: Granularity,
    pub points: Vec<TrendPoint>,
}

pub struct Deps {
    pub entries: Arc<dyn MoodEntryQueryPort>,
    pub users: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(query: MoodTrendQuery, deps: &Deps) -> Result<MoodTrend, ApplicationError> {
    let timezone = timezone_for(&query.user_id, &deps.users).await?;
    let entries = deps
        .entries
        .find_by_date_range(&query.user_id, &instants_of(&query.span))
        .await?;

    let mut entry_counts: BTreeMap<Date, usize> = BTreeMap::new();
    for entry in &entries {
        *entry_counts.entry(date_of(entry, &timezone)).or_default() += 1;
    }

    let days: Vec<TrendDay> = day_moods(&entries, &timezone)
        .into_iter()
        .filter(|(date, _)| query.span.contains(date))
        .map(|(date, mood)| {
            TrendDay::new(
                date,
                mood,
                entry_counts.get(&date).copied().unwrap_or_default(),
            )
        })
        .collect();

    let granularity = Granularity::for_single_period(query.span.days());
    let points = CalendarBucketing::new(query.week_start).bucket(&query.span, &days, granularity);

    Ok(MoodTrend {
        granularity,
        points,
    })
}
