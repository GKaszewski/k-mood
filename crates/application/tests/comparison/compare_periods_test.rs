use std::sync::Arc;

use chrono::{DateTime, FixedOffset, NaiveDate};

use domain::activity::ActivityId;
use domain::comparison::{ComparisonRules, MaximumSpan, Quantity};
use domain::entry::{Date, DateSpan, Mood, MoodEntry};
use domain::errors::DomainError;
use domain::metric::{DailyMetric, MetricKind, MetricValue, Source, Steps};
use domain::ports::{
    ActivityCommandPort, CycleStartCommandPort, DailyMetricCommandPort, MoodEntryCommandPort,
    UserCommandPort, UserPreferencesCommandPort,
};
use domain::testing::{InMemoryStore, test_activity, test_user};
use domain::user::{Timezone, User, UserPreferences};

use application::comparison::queries::ComparisonQuery;
use application::comparison::use_cases::compare_periods::{self, PeriodComparison};
use application::errors::ApplicationError;

const MAXIMUM_DAYS: i64 = 1830;
const TOP_MUSIC: usize = 10;

fn a_user_in_warsaw() -> User {
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    user
}

fn day(year: i32, month: u32, day: u32) -> Date {
    Date::from_persistence(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

fn span(from: Date, to: Date) -> DateSpan {
    DateSpan::new(from, to).unwrap()
}

fn october(year: i32) -> DateSpan {
    span(day(year, 10, 1), day(year, 10, 31))
}

fn noon(date: Date) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(&format!("{}T12:00:00+01:00", date.value())).unwrap()
}

async fn setup() -> (Arc<InMemoryStore>, User) {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    (store, user)
}

async fn logged_at(store: &Arc<InMemoryStore>, user: &User, mood: Mood, at: DateTime<FixedOffset>) {
    let entry = MoodEntry::new(user.id().clone(), mood, at);
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();
}

async fn logged(store: &Arc<InMemoryStore>, user: &User, mood: Mood, date: Date) {
    logged_at(store, user, mood, noon(date)).await;
}

async fn compare(
    store: &Arc<InMemoryStore>,
    user: &User,
    a: DateSpan,
    b: DateSpan,
    maximum_days: i64,
) -> Result<PeriodComparison, ApplicationError> {
    let deps = compare_periods::Deps {
        entries: store.clone(),
        metrics: store.clone(),
        activities: store.clone(),
        cycles: store.clone(),
        dimensions: vec![Arc::new(
            domain::testing::InMemoryActivityDimension::sharing(store.clone()),
        )],
        preferences: store.clone(),
        users: store.clone(),
    };

    let query = ComparisonQuery {
        user_id: user.id().clone(),
        a,
        b,
        maximum_span: MaximumSpan::of_days(maximum_days),
        rules: ComparisonRules {
            minimum_sample_size: 3,
            false_discovery_rate: 0.10,
        },
        top_music_count: TOP_MUSIC,
    };

    compare_periods::execute(query, &deps).await
}

fn row_of(result: &PeriodComparison, quantity: Quantity) -> &domain::comparison::ComparisonRow {
    result
        .comparison
        .rows()
        .iter()
        .find(|row| row.quantity() == &quantity)
        .expect("the quantity is compared")
}

#[tokio::test]
async fn a_better_october_is_compared_against_last_october() {
    let (store, user) = setup().await;
    for offset in 1..=4 {
        logged(&store, &user, Mood::Bad, day(2025, 10, offset)).await;
        logged(&store, &user, Mood::Meh, day(2025, 10, offset + 10)).await;
        logged(&store, &user, Mood::Good, day(2026, 10, offset)).await;
        logged(&store, &user, Mood::Rad, day(2026, 10, offset + 10)).await;
    }

    let result = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();

    let mood = row_of(&result, Quantity::AverageMood);
    assert_eq!(mood.a(), Some(2.5));
    assert_eq!(mood.b(), Some(4.5));
    assert!(!result.comparison.overlap());
}

#[tokio::test]
async fn entries_near_midnight_count_on_the_users_day() {
    let (store, user) = setup().await;
    // 2026-10-31T23:30Z: October in UTC, but already November in Warsaw.
    let after_warsaw_midnight = DateTime::parse_from_rfc3339("2026-11-01T00:30:00+01:00").unwrap();
    logged_at(&store, &user, Mood::Rad, after_warsaw_midnight).await;

    let november = span(day(2026, 11, 1), day(2026, 11, 3));
    let in_november = compare(&store, &user, october(2025), november, MAXIMUM_DAYS)
        .await
        .unwrap();
    assert_eq!(row_of(&in_november, Quantity::LoggedDays).b(), Some(1.0));

    let in_october = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();
    assert_eq!(row_of(&in_october, Quantity::LoggedDays).b(), Some(0.0));
}

#[tokio::test]
async fn a_running_period_is_clipped_to_today() {
    let (store, user) = setup().await;
    let running = span(day(2026, 1, 1), day(2099, 12, 31));

    let result = compare(&store, &user, october(2025), running, 100_000)
        .await
        .unwrap();

    assert!(result.spans.1.was_clipped());
    assert!(!result.spans.0.was_clipped());
}

#[tokio::test]
async fn a_period_wholly_in_the_future_is_refused() {
    let (store, user) = setup().await;
    let future = span(day(2099, 1, 1), day(2099, 1, 31));

    let refused = compare(&store, &user, october(2025), future, MAXIMUM_DAYS).await;

    assert!(matches!(
        refused,
        Err(ApplicationError::Domain(DomainError::InvalidInput(_)))
    ));
}

#[tokio::test]
async fn a_span_beyond_the_maximum_is_refused() {
    let (store, user) = setup().await;

    let refused = compare(&store, &user, october(2025), october(2026), 30).await;

    assert!(matches!(
        refused,
        Err(ApplicationError::Domain(DomainError::InvalidInput(_)))
    ));
}

#[tokio::test]
async fn an_archived_activity_still_has_a_label() {
    let (store, user) = setup().await;
    let mut retired = test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();
    ActivityCommandPort::save(store.as_ref(), &retired)
        .await
        .unwrap();

    let result = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();

    let label: &ActivityId = retired.id();
    assert_eq!(
        result.activities.get(label).map(|a| a.name().value()),
        Some("fencing")
    );
}

#[tokio::test]
async fn activities_tagged_on_entries_are_compared() {
    let (store, user) = setup().await;
    let walking = test_activity(user.id().clone(), "walking");
    ActivityCommandPort::save(store.as_ref(), &walking)
        .await
        .unwrap();
    let entry = MoodEntry::new(user.id().clone(), Mood::Good, noon(day(2026, 10, 5)));
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();
    store.assign_activities(entry.id(), vec![walking.id().clone()]);

    let result = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();

    assert_eq!(
        row_of(&result, Quantity::MoodWithActivity(walking.id().clone())).b(),
        Some(4.0)
    );
}

#[tokio::test]
async fn metrics_are_compared_per_day_with_data() {
    let (store, user) = setup().await;
    for (date, steps) in [(day(2025, 10, 2), 1_000), (day(2026, 10, 2), 3_000)] {
        let metric = DailyMetric::new(
            user.id().clone(),
            date,
            MetricValue::Steps(Steps::new(steps).unwrap()),
            Source::Manual,
        );
        DailyMetricCommandPort::save(store.as_ref(), &[metric])
            .await
            .unwrap();
    }

    let result = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();

    let days = row_of(&result, Quantity::MetricDaysWithData(MetricKind::Steps));
    assert_eq!(days.a(), Some(1.0));
    assert_eq!(days.b(), Some(1.0));
    assert_eq!(
        row_of(&result, Quantity::MetricMean(MetricKind::Steps)).b(),
        Some(3_000.0)
    );
}

#[tokio::test]
async fn cycle_is_skipped_when_not_tracked() {
    let (store, user) = setup().await;
    CycleStartCommandPort::record(store.as_ref(), user.id(), &day(2026, 10, 1))
        .await
        .unwrap();
    logged(&store, &user, Mood::Good, day(2026, 10, 5)).await;

    let untracked = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();
    assert!(!untracked.comparison.b().tracks_cycle());

    let preferences = UserPreferences::from_persistence(user.id().clone(), true);
    UserPreferencesCommandPort::save(store.as_ref(), &preferences)
        .await
        .unwrap();

    let tracked = compare(&store, &user, october(2025), october(2026), MAXIMUM_DAYS)
        .await
        .unwrap();
    assert!(tracked.comparison.b().tracks_cycle());
}
