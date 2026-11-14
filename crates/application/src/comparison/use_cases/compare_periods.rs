use std::collections::BTreeMap;
use std::sync::Arc;

use domain::activity::{Activity, ActivityId};
use domain::comparison::{ComparedSpans, Comparison, PeriodInput, PeriodSummary};
use domain::cycle::CycleCalendar;
use domain::entry::{ClippedSpan, Date, DateSpan};
use domain::errors::DomainError;
use domain::metric::MetricKind;
use domain::ports::{
    ActivityQueryPort, CycleStartQueryPort, DailyMetricQueryPort, EntryDimensionPort,
    MoodEntryQueryPort, UserPreferencesQueryPort, UserQueryPort,
};
use domain::trend::TrendDay;
use domain::user::{Timezone, UserId};

use crate::comparison::period_days::period_days;
use crate::comparison::queries::ComparisonQuery;
use crate::cycle::use_cases::read_cycle::calendar_of;
use crate::day::{day_moods, instants_of, timezone_for, today_in};
use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

pub struct Deps {
    pub entries: Arc<dyn MoodEntryQueryPort>,
    pub metrics: Arc<dyn DailyMetricQueryPort>,
    pub activities: Arc<dyn ActivityQueryPort>,
    pub cycles: Arc<dyn CycleStartQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub preferences: Arc<dyn UserPreferencesQueryPort>,
    pub users: Arc<dyn UserQueryPort>,
}

pub struct PeriodComparison {
    pub comparison: Comparison,
    pub spans: (ClippedSpan, ClippedSpan),
    pub activities: BTreeMap<ActivityId, Activity>,
}

struct Loaded {
    input: PeriodInput,
    trend: Vec<TrendDay>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    query: ComparisonQuery,
    deps: &Deps,
) -> Result<PeriodComparison, ApplicationError> {
    let spans = ComparedSpans::new(query.a, query.b, query.maximum_span)?;
    let timezone = timezone_for(&query.user_id, &deps.users).await?;
    let today = today_in(&timezone);

    let clipped_a = clipped(spans.a(), today)?;
    let clipped_b = clipped(spans.b(), today)?;

    let tracks_cycle = preferences_of(&query.user_id, &deps.preferences)
        .await?
        .tracks_cycle();
    let calendar = if tracks_cycle {
        Some(calendar_of(&query.user_id, &deps.cycles).await?)
    } else {
        None
    };

    let a = load(
        &query,
        deps,
        &timezone,
        clipped_a,
        tracks_cycle,
        calendar.as_ref(),
    )
    .await?;
    let b = load(
        &query,
        deps,
        &timezone,
        clipped_b,
        tracks_cycle,
        calendar.as_ref(),
    )
    .await?;

    let comparison = Comparison::between(
        PeriodSummary::of(&a.input),
        PeriodSummary::of(&b.input),
        &spans,
        &a.trend,
        &b.trend,
        query.rules,
    );

    let activities = labels(&query.user_id, deps).await?;

    Ok(PeriodComparison {
        comparison,
        spans: (clipped_a, clipped_b),
        activities,
    })
}

fn clipped(span: &DateSpan, today: Date) -> Result<ClippedSpan, ApplicationError> {
    span.clipped_to(today).ok_or_else(|| {
        DomainError::InvalidInput("a compared span cannot lie wholly in the future".into()).into()
    })
}

async fn load(
    query: &ComparisonQuery,
    deps: &Deps,
    timezone: &Timezone,
    span: ClippedSpan,
    tracks_cycle: bool,
    calendar: Option<&CycleCalendar>,
) -> Result<Loaded, ApplicationError> {
    let entries = deps
        .entries
        .find_by_date_range(&query.user_id, &instants_of(span.span()))
        .await?;

    let mut moods = day_moods(&entries, timezone);
    moods.retain(|date, _| span.span().contains(date));

    let composed = EntryComposer::new(deps.dimensions.clone())
        .compose(entries)
        .await?;

    let days = period_days(&moods, &composed, timezone, calendar);
    let trend = trend_days(&days);
    let metrics = metrics_in(query, deps, span.span()).await?;

    Ok(Loaded {
        input: PeriodInput {
            span,
            days,
            metrics,
            tracks_cycle,
            top_music_count: query.top_music_count,
        },
        trend,
    })
}

fn trend_days(days: &[domain::comparison::PeriodDay]) -> Vec<TrendDay> {
    days.iter()
        .map(|day| TrendDay::new(day.date, day.mood, day.entries))
        .collect()
}

async fn metrics_in(
    query: &ComparisonQuery,
    deps: &Deps,
    span: &DateSpan,
) -> Result<BTreeMap<MetricKind, BTreeMap<Date, f64>>, ApplicationError> {
    let metrics = deps.metrics.find_by_span(&query.user_id, span).await?;

    let mut by_kind: BTreeMap<MetricKind, BTreeMap<Date, f64>> = BTreeMap::new();
    for metric in metrics {
        by_kind
            .entry(metric.kind())
            .or_default()
            .insert(*metric.date(), metric.value().count() as f64);
    }

    Ok(by_kind)
}

async fn labels(
    user_id: &UserId,
    deps: &Deps,
) -> Result<BTreeMap<ActivityId, Activity>, ApplicationError> {
    Ok(deps
        .activities
        .find_by_user(user_id)
        .await?
        .into_iter()
        .map(|activity| (activity.id().clone(), activity))
        .collect())
}
