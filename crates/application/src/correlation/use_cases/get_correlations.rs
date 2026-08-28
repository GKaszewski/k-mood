use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use domain::activity::ActivityId;
use domain::correlation::{
    Adjustment, Agreement, Coefficient, CorrelationInput, CorrelationStrategy, Observation, PValue,
    Tested,
};
use domain::dimension::ComposedEntry;
use domain::entry::{Date, DayMood};
use domain::metric::MetricKind;
use domain::moon::MoonPhase;
use domain::ports::{
    ActivityQueryPort, CycleStartQueryPort, DailyMetricQueryPort, EntryDimensionPort,
    MoodEntryQueryPort, UserPreferencesQueryPort, UserQueryPort,
};
use domain::user::Timezone;

use crate::correlation::queries::CorrelationQuery;
use crate::cycle::use_cases::read_cycle::calendar_of;
use crate::day::{date_of, day_moods, instants_of, timezone_for};
use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

const PRESENT: f64 = 1.0;
const ABSENT: f64 = 0.0;

pub struct Score {
    pub strategy: CorrelationStrategy,
    pub coefficient: Coefficient,
    pub held_up: bool,
}

pub struct CorrelationRow {
    pub input: CorrelationInput,
    pub label: Option<String>,
    pub sample_size: usize,
    pub scores: Vec<Score>,
    pub agreement: Agreement,
}

pub struct Deps {
    pub entries: Arc<dyn MoodEntryQueryPort>,
    pub metrics: Arc<dyn DailyMetricQueryPort>,
    pub activities: Arc<dyn ActivityQueryPort>,
    pub cycles: Arc<dyn CycleStartQueryPort>,
    pub weather_store: Arc<dyn EntryDimensionPort>,
    pub activity_store: Arc<dyn EntryDimensionPort>,
    pub preferences: Arc<dyn UserPreferencesQueryPort>,
    pub users: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    query: CorrelationQuery,
    deps: &Deps,
) -> Result<Vec<CorrelationRow>, ApplicationError> {
    let timezone = timezone_for(&query.user_id, &deps.users).await?;

    let entries = deps
        .entries
        .find_by_date_range(&query.user_id, &instants_of(&query.span))
        .await?;

    let mut moods = day_moods(&entries, &timezone);
    moods.retain(|date, _| query.span.contains(date));

    let composed = EntryComposer::new(vec![
        deps.weather_store.clone(),
        deps.activity_store.clone(),
    ])
    .compose(entries)
    .await?;

    let mut scored = metric_rows(&query, deps, &moods).await?;

    let lunar = paired(&moods, |date| Some(MoonPhase::on(date).illumination()));
    scored.push(row(
        CorrelationInput::MoonPhase,
        None,
        &lunar,
        query.minimum_sample_size,
    ));

    if let Some(cycle) = cycle_row(&query, deps, &moods).await? {
        scored.push(cycle);
    }

    scored.push(temperature_row(&query, &composed, &moods, &timezone));

    scored.extend(activity_rows(&query, deps, &composed, &moods, &timezone).await?);

    Ok(mark_what_holds_up(scored, query.false_discovery_rate))
}

async fn metric_rows(
    query: &CorrelationQuery,
    deps: &Deps,
    moods: &BTreeMap<Date, DayMood>,
) -> Result<Vec<Pending>, ApplicationError> {
    let metrics = deps
        .metrics
        .find_by_span(&query.user_id, &query.span)
        .await?;

    let mut by_kind: BTreeMap<MetricKind, BTreeMap<Date, f64>> = BTreeMap::new();
    for metric in metrics {
        by_kind
            .entry(metric.kind())
            .or_default()
            .insert(*metric.date(), metric.value().count() as f64);
    }

    Ok(MetricKind::ALL
        .into_iter()
        .map(|kind| {
            let values = by_kind.remove(&kind).unwrap_or_default();
            let observations = paired(moods, |date| values.get(date).copied());

            row(
                CorrelationInput::Metric(kind),
                None,
                &observations,
                query.minimum_sample_size,
            )
        })
        .collect())
}

async fn cycle_row(
    query: &CorrelationQuery,
    deps: &Deps,
    moods: &BTreeMap<Date, DayMood>,
) -> Result<Option<Pending>, ApplicationError> {
    let preferences = preferences_of(&query.user_id, &deps.preferences).await?;

    if !preferences.tracks_cycle() {
        return Ok(None);
    }

    let calendar = calendar_of(&query.user_id, &deps.cycles).await?;
    let observations = paired(moods, |date| {
        calendar
            .position_on(date)
            .map(|position| position.progress())
    });

    Ok(Some(row(
        CorrelationInput::CycleProgress,
        None,
        &observations,
        query.minimum_sample_size,
    )))
}

fn temperature_row(
    query: &CorrelationQuery,
    composed: &[ComposedEntry],
    moods: &BTreeMap<Date, DayMood>,
    timezone: &Timezone,
) -> Pending {
    let warmest = warmest_by_date(composed, timezone);
    let observations = paired(moods, |date| warmest.get(date).copied());

    row(
        CorrelationInput::Temperature,
        None,
        &observations,
        query.minimum_sample_size,
    )
}

fn warmest_by_date(composed: &[ComposedEntry], timezone: &Timezone) -> BTreeMap<Date, f64> {
    let mut warmest: BTreeMap<Date, f64> = BTreeMap::new();

    for entry in composed {
        let Some(weather) = entry.weather() else {
            continue;
        };

        let date = date_of(&entry.entry, timezone);
        let temperature = weather.temperature().value();

        warmest
            .entry(date)
            .and_modify(|highest| *highest = highest.max(temperature))
            .or_insert(temperature);
    }

    warmest
}

async fn activity_rows(
    query: &CorrelationQuery,
    deps: &Deps,
    composed: &[ComposedEntry],
    moods: &BTreeMap<Date, DayMood>,
    timezone: &Timezone,
) -> Result<Vec<Pending>, ApplicationError> {
    let catalogue = deps.activities.find_active_by_user(&query.user_id).await?;
    let tagged = days_tagged_by_activity(composed, timezone);
    let mut rows = Vec::with_capacity(catalogue.len());

    for activity in catalogue {
        let days = tagged.get(activity.id());

        let Some(days) = days.filter(|days| ever_logged(days, moods)) else {
            continue;
        };

        let observations = paired(moods, |date| {
            Some(if days.contains(date) { PRESENT } else { ABSENT })
        });

        rows.push(row(
            CorrelationInput::Activity(activity.id().clone()),
            Some(activity.name().value().to_string()),
            &observations,
            query.minimum_sample_size,
        ));
    }

    Ok(rows)
}

fn days_tagged_by_activity(
    composed: &[ComposedEntry],
    timezone: &Timezone,
) -> BTreeMap<ActivityId, BTreeSet<Date>> {
    let mut tagged: BTreeMap<ActivityId, BTreeSet<Date>> = BTreeMap::new();

    for entry in composed {
        let date = date_of(&entry.entry, timezone);

        for activity_id in entry.activities() {
            tagged.entry(activity_id.clone()).or_default().insert(date);
        }
    }

    tagged
}

fn ever_logged(days: &BTreeSet<Date>, moods: &BTreeMap<Date, DayMood>) -> bool {
    days.iter().any(|date| moods.contains_key(date))
}

fn paired(
    moods: &BTreeMap<Date, DayMood>,
    value_on: impl Fn(&Date) -> Option<f64>,
) -> Vec<Observation> {
    moods
        .iter()
        .filter_map(|(date, mood)| value_on(date).map(|value| Observation::new(value, *mood)))
        .collect()
}

struct Pending {
    row: CorrelationRow,
    significance: Vec<Option<PValue>>,
}

fn mark_what_holds_up(pending: Vec<Pending>, false_discovery_rate: f64) -> Vec<CorrelationRow> {
    let mut pending = pending;
    let places = scored_places(&pending);

    let tested: Vec<Tested> = places
        .iter()
        .filter_map(|(row_index, score_index)| {
            let entry = &pending[*row_index];

            Some(Tested {
                family: entry.row.input.family(),
                strategy: entry.row.scores[*score_index].strategy,
                p_value: entry.significance[*score_index]?,
            })
        })
        .collect();

    let adjustment = Adjustment::controlling_false_discovery_at(false_discovery_rate);

    for (place, holds) in places.iter().zip(adjustment.holds_up_across(&tested)) {
        pending[place.0].row.scores[place.1].held_up = holds;
    }

    pending.into_iter().map(|entry| entry.row).collect()
}

fn scored_places(pending: &[Pending]) -> Vec<(usize, usize)> {
    pending
        .iter()
        .enumerate()
        .flat_map(|(row_index, entry)| {
            entry
                .significance
                .iter()
                .enumerate()
                .filter(|(_, p_value)| p_value.is_some())
                .map(move |(score_index, _)| (row_index, score_index))
        })
        .collect()
}

fn row(
    input: CorrelationInput,
    label: Option<String>,
    observations: &[Observation],
    minimum_sample_size: usize,
) -> Pending {
    let sample_size = observations.len();
    let applicable = CorrelationStrategy::ALL
        .into_iter()
        .filter(|strategy| strategy.can_score(&input))
        .count();

    let (scores, significance) = if sample_size < minimum_sample_size {
        (Vec::new(), Vec::new())
    } else {
        scored(&input, observations)
    };

    let coefficients: Vec<Coefficient> = scores.iter().map(|score| score.coefficient).collect();

    Pending {
        row: CorrelationRow {
            input,
            label,
            sample_size,
            agreement: Agreement::of(&coefficients, applicable),
            scores,
        },
        significance,
    }
}

fn scored(
    input: &CorrelationInput,
    observations: &[Observation],
) -> (Vec<Score>, Vec<Option<PValue>>) {
    CorrelationStrategy::ALL
        .into_iter()
        .filter(|strategy| strategy.can_score(input))
        .filter_map(|strategy| {
            let coefficient = strategy.score(observations)?;

            Some((
                Score {
                    strategy,
                    coefficient,
                    held_up: false,
                },
                strategy.significance(observations),
            ))
        })
        .unzip()
}
