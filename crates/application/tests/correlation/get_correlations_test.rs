use std::sync::Arc;

use chrono::{DateTime, Duration, FixedOffset};

use domain::correlation::{CorrelationInput, CorrelationStrategy};
use domain::entry::{Date, DateSpan, Mood, MoodEntry};
use domain::metric::{DailyMetric, MetricKind, MetricValue, Source, Steps};
use domain::ports::{
    ActivityCommandPort, DailyMetricCommandPort, MoodEntryCommandPort, UserCommandPort,
};
use domain::testing::{InMemoryStore, test_activity, test_user};
use domain::user::{Timezone, User};

use application::correlation::queries::CorrelationQuery;
use application::correlation::use_cases::get_correlations::{self, CorrelationRow};

const MINIMUM: usize = 30;

fn a_user_in_warsaw() -> User {
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    user
}

fn noon_on(day: usize) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339("2026-01-01T12:00:00+01:00").unwrap() + Duration::days(day as i64)
}

fn date_of(day: usize) -> Date {
    Date::from_persistence(noon_on(day).date_naive())
}

async fn logged(store: &Arc<InMemoryStore>, user: &User, mood: Mood, day: usize) {
    let entry = MoodEntry::new(user.id().clone(), mood, noon_on(day));
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();
}

async fn walked(store: &Arc<InMemoryStore>, user: &User, steps: u32, day: usize) {
    let metric = DailyMetric::new(
        user.id().clone(),
        date_of(day),
        MetricValue::Steps(Steps::new(steps).unwrap()),
        Source::Manual,
    );
    DailyMetricCommandPort::save(store.as_ref(), &[metric])
        .await
        .unwrap();
}

fn a_rising_mood_for(day: usize) -> Mood {
    match day / 8 {
        0 => Mood::Awful,
        1 => Mood::Bad,
        2 => Mood::Meh,
        3 => Mood::Good,
        _ => Mood::Rad,
    }
}

fn a_mood_for(day: usize) -> Mood {
    match day % 5 {
        0 => Mood::Awful,
        1 => Mood::Bad,
        2 => Mood::Meh,
        3 => Mood::Good,
        _ => Mood::Rad,
    }
}

async fn correlations(store: &Arc<InMemoryStore>, user: &User) -> Vec<CorrelationRow> {
    let deps = get_correlations::Deps {
        entries: store.clone(),
        metrics: store.clone(),
        activities: store.clone(),
        cycles: store.clone(),
        weather_store: Arc::new(domain::testing::InMemoryDimensionStore::new(
            domain::dimension::DimensionKind::Weather,
        )),
        preferences: store.clone(),
        users: store.clone(),
    };

    let query = CorrelationQuery {
        user_id: user.id().clone(),
        span: DateSpan::new(date_of(0), date_of(120)).unwrap(),
        minimum_sample_size: MINIMUM,
        false_discovery_rate: 0.10,
    };

    get_correlations::execute(query, &deps).await.unwrap()
}

fn row_for(rows: &[CorrelationRow], input: CorrelationInput) -> &CorrelationRow {
    rows.iter()
        .find(|row| row.input == input)
        .expect("every input has a row")
}

fn spearman_of(row: &CorrelationRow) -> Option<f64> {
    row.scores
        .iter()
        .find(|score| score.strategy == CorrelationStrategy::Spearman)
        .map(|score| score.coefficient.value())
}

#[tokio::test]
async fn steps_that_track_mood_exactly_score_one() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_rising_mood_for(day), day).await;
        walked(&store, &user, 1_000 + day as u32 * 100, day).await;
    }

    let rows = correlations(&store, &user).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert_eq!(steps.sample_size, 40);
    let score = spearman_of(steps).expect("40 days is above the floor");
    assert!(score > 0.9, "expected a strong positive score, got {score}");
}

#[tokio::test]
async fn too_few_days_yield_a_row_but_no_coefficient() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..12 {
        logged(&store, &user, a_mood_for(day), day).await;
        walked(&store, &user, 1_000 + day as u32 * 100, day).await;
    }

    let rows = correlations(&store, &user).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert_eq!(steps.sample_size, 12);
    assert!(steps.scores.is_empty());
}

#[tokio::test]
async fn a_day_with_several_entries_contributes_one_point() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..31 {
        logged(&store, &user, a_mood_for(day), day).await;
        walked(&store, &user, 1_000 + day as u32 * 100, day).await;
    }
    for _ in 0..4 {
        logged(&store, &user, Mood::Rad, 0).await;
    }

    let rows = correlations(&store, &user).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert_eq!(steps.sample_size, 31);
}

#[tokio::test]
async fn days_without_the_metric_are_left_out_of_its_sample() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }
    for day in 0..33 {
        walked(&store, &user, 1_000 + day as u32 * 100, day).await;
    }

    let rows = correlations(&store, &user).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert_eq!(steps.sample_size, 33);
}

#[tokio::test]
async fn the_moon_is_scored_against_every_day_that_has_a_mood() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let rows = correlations(&store, &user).await;
    let moon = row_for(&rows, CorrelationInput::MoonPhase);

    assert_eq!(moon.sample_size, 40);
    assert!(spearman_of(moon).is_some());
}

#[tokio::test]
async fn a_kind_that_was_never_recorded_still_has_a_row_with_no_days() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let rows = correlations(&store, &user).await;
    let hrv = row_for(&rows, CorrelationInput::Metric(MetricKind::Hrv));

    assert_eq!(hrv.sample_size, 0);
    assert!(hrv.scores.is_empty());
    let inputs_that_need_no_data: usize = 2;
    assert_eq!(
        rows.len(),
        MetricKind::ALL.len() + inputs_that_need_no_data,
        "every metric kind, plus the moon and the temperature"
    );
}

#[tokio::test]
async fn an_entry_just_after_local_midnight_counts_for_that_day() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 1..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let just_after_midnight = DateTime::parse_from_rfc3339("2026-01-01T00:30:00+01:00").unwrap();
    let entry = MoodEntry::new(user.id().clone(), Mood::Good, just_after_midnight);
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();

    let rows = correlations(&store, &user).await;
    let moon = row_for(&rows, CorrelationInput::MoonPhase);

    assert_eq!(
        moon.sample_size, 40,
        "the first day's early entry was dropped"
    );
}

#[tokio::test]
async fn a_day_outside_the_span_is_not_counted() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let the_day_before = DateTime::parse_from_rfc3339("2025-12-31T12:00:00+01:00").unwrap();
    let entry = MoodEntry::new(user.id().clone(), Mood::Rad, the_day_before);
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();

    let rows = correlations(&store, &user).await;
    let moon = row_for(&rows, CorrelationInput::MoonPhase);

    assert_eq!(moon.sample_size, 40, "a day outside the span was counted");
}

async fn correlations_with_floor(
    store: &Arc<InMemoryStore>,
    user: &User,
    floor: usize,
) -> Vec<CorrelationRow> {
    let deps = get_correlations::Deps {
        entries: store.clone(),
        metrics: store.clone(),
        activities: store.clone(),
        cycles: store.clone(),
        weather_store: Arc::new(domain::testing::InMemoryDimensionStore::new(
            domain::dimension::DimensionKind::Weather,
        )),
        preferences: store.clone(),
        users: store.clone(),
    };

    let query = CorrelationQuery {
        user_id: user.id().clone(),
        span: DateSpan::new(date_of(0), date_of(120)).unwrap(),
        minimum_sample_size: floor,
        false_discovery_rate: 0.10,
    };

    get_correlations::execute(query, &deps).await.unwrap()
}

fn strategies_of(row: &CorrelationRow) -> Vec<&str> {
    row.scores
        .iter()
        .map(|score| score.strategy.name())
        .collect()
}

#[tokio::test]
async fn every_applicable_strategy_scores_a_metric() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_rising_mood_for(day), day).await;
        walked(&store, &user, 1_000 + day as u32 * 100, day).await;
    }

    let rows = correlations(&store, &user).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert_eq!(strategies_of(steps), ["pearson", "spearman", "kendall"]);
    assert_eq!(steps.agreement.applicable(), 3);
    assert_eq!(steps.agreement.agreeing(), 3);
}

#[tokio::test]
async fn one_outlying_day_can_set_pearson_against_the_rank_methods() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let moods = [
        Mood::Awful,
        Mood::Awful,
        Mood::Bad,
        Mood::Bad,
        Mood::Meh,
        Mood::Meh,
        Mood::Good,
        Mood::Good,
        Mood::Rad,
        Mood::Awful,
    ];
    let steps = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10_000];

    for day in 0..10 {
        logged(&store, &user, moods[day], day).await;
        walked(&store, &user, steps[day], day).await;
    }

    let rows = correlations_with_floor(&store, &user, 10).await;
    let row = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    let pearson = row
        .scores
        .iter()
        .find(|score| score.strategy == CorrelationStrategy::Pearson)
        .unwrap();

    assert!(
        pearson.coefficient.value() < 0.0,
        "the outlier pulls the line down"
    );
    assert_eq!(row.agreement.applicable(), 3);
    assert_eq!(
        row.agreement.agreeing(),
        2,
        "the two rank methods still agree"
    );
}

#[tokio::test]
async fn an_activity_is_scored_by_mean_difference_alone() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let exercise = test_activity(user.id().clone(), "exercise");
    ActivityCommandPort::save(store.as_ref(), &exercise)
        .await
        .unwrap();

    for day in 0..40 {
        let mood = if day % 2 == 0 { Mood::Rad } else { Mood::Bad };
        let entry = MoodEntry::new(user.id().clone(), mood, noon_on(day));
        MoodEntryCommandPort::save(store.as_ref(), &entry)
            .await
            .unwrap();

        if day % 2 == 0 {
            store.assign_activities(entry.id(), vec![exercise.id().clone()]);
        }
    }

    let rows = correlations(&store, &user).await;
    let row = row_for(&rows, CorrelationInput::Activity(exercise.id().clone()));

    assert_eq!(strategies_of(row), ["meanDifference"]);
    assert_eq!(row.agreement.applicable(), 1);
    assert_eq!(row.agreement.agreeing(), 1);
    assert!(row.scores[0].coefficient.value() > 0.0);
}

#[tokio::test]
async fn an_activity_logged_every_single_day_cannot_be_compared() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let coffee = test_activity(user.id().clone(), "coffee");
    ActivityCommandPort::save(store.as_ref(), &coffee)
        .await
        .unwrap();

    for day in 0..40 {
        let entry = MoodEntry::new(user.id().clone(), a_mood_for(day), noon_on(day));
        MoodEntryCommandPort::save(store.as_ref(), &entry)
            .await
            .unwrap();
        store.assign_activities(entry.id(), vec![coffee.id().clone()]);
    }

    let rows = correlations(&store, &user).await;
    let row = row_for(&rows, CorrelationInput::Activity(coffee.id().clone()));

    assert!(
        row.scores.is_empty(),
        "there are no days without it to compare"
    );
    assert_eq!(row.agreement.applicable(), 1);
    assert_eq!(row.agreement.agreeing(), 0);
}

#[tokio::test]
async fn an_archived_activity_is_not_offered() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let mut retired = test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();
    ActivityCommandPort::save(store.as_ref(), &retired)
        .await
        .unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let rows = correlations(&store, &user).await;

    assert!(
        !rows
            .iter()
            .any(|row| row.input == CorrelationInput::Activity(retired.id().clone())),
        "an archived activity cannot be tagged onto new entries and is not an input"
    );
}

#[tokio::test]
async fn an_activity_never_logged_in_the_span_is_not_a_row_at_all() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let untouched = test_activity(user.id().clone(), "fencing");
    ActivityCommandPort::save(store.as_ref(), &untouched)
        .await
        .unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let rows = correlations(&store, &user).await;

    assert!(
        !rows
            .iter()
            .any(|row| row.input == CorrelationInput::Activity(untouched.id().clone())),
        "an activity with no logged day says nothing and should not be reported as measured"
    );
}

async fn deps_free_entries(store: &Arc<InMemoryStore>, user: &User) -> Vec<MoodEntry> {
    use domain::ports::MoodEntryQueryPort;

    MoodEntryQueryPort::find_by_user(store.as_ref(), user.id(), None, None)
        .await
        .unwrap()
}

async fn correlations_with_rate(
    store: &Arc<InMemoryStore>,
    user: &User,
    floor: usize,
    false_discovery_rate: f64,
) -> Vec<CorrelationRow> {
    let deps = get_correlations::Deps {
        entries: store.clone(),
        metrics: store.clone(),
        activities: store.clone(),
        cycles: store.clone(),
        weather_store: Arc::new(domain::testing::InMemoryDimensionStore::new(
            domain::dimension::DimensionKind::Weather,
        )),
        preferences: store.clone(),
        users: store.clone(),
    };

    let query = CorrelationQuery {
        user_id: user.id().clone(),
        span: DateSpan::new(date_of(0), date_of(120)).unwrap(),
        minimum_sample_size: floor,
        false_discovery_rate,
    };

    get_correlations::execute(query, &deps).await.unwrap()
}

fn held_up(row: &CorrelationRow, strategy: CorrelationStrategy) -> bool {
    row.scores
        .iter()
        .find(|score| score.strategy == strategy)
        .map(|score| score.held_up)
        .unwrap_or(false)
}

async fn a_year_of_steps_tracking_mood(store: &Arc<InMemoryStore>, user: &User) {
    for day in 0..40 {
        logged(store, user, a_rising_mood_for(day), day).await;
        walked(store, user, 1_000 + day as u32 * 100, day).await;
    }
}

#[tokio::test]
async fn a_relationship_that_survives_the_number_of_things_tested_is_marked() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    a_year_of_steps_tracking_mood(&store, &user).await;

    let rows = correlations_with_rate(&store, &user, 30, 0.10).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert!(held_up(steps, CorrelationStrategy::Spearman));
    assert!(held_up(steps, CorrelationStrategy::Kendall));
}

#[tokio::test]
async fn the_moon_does_not_survive_the_correction_on_ordinary_data() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    a_year_of_steps_tracking_mood(&store, &user).await;

    let rows = correlations_with_rate(&store, &user, 30, 0.10).await;
    let moon = row_for(&rows, CorrelationInput::MoonPhase);

    assert!(!held_up(moon, CorrelationStrategy::Spearman));
}

#[tokio::test]
async fn keeping_more_activities_does_not_change_whether_a_metric_is_marked() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    a_year_of_steps_tracking_mood(&store, &user).await;

    let before = correlations_with_rate(&store, &user, 30, 0.10).await;
    let steps_before = held_up(
        row_for(&before, CorrelationInput::Metric(MetricKind::Steps)),
        CorrelationStrategy::Spearman,
    );

    for number in 0..20 {
        let activity = test_activity(user.id().clone(), &format!("activity {number}"));
        ActivityCommandPort::save(store.as_ref(), &activity)
            .await
            .unwrap();

        let entries = deps_free_entries(&store, &user).await;
        for (index, entry) in entries.iter().enumerate() {
            if index % (number + 2) == 0 {
                store.assign_activities(entry.id(), vec![activity.id().clone()]);
            }
        }
    }

    let after = correlations_with_rate(&store, &user, 30, 0.10).await;
    let steps_after = held_up(
        row_for(&after, CorrelationInput::Metric(MetricKind::Steps)),
        CorrelationStrategy::Spearman,
    );

    assert!(
        after.len() > before.len(),
        "the activities should have produced rows of their own"
    );
    assert_eq!(
        steps_before, steps_after,
        "a metric's mark must not depend on how many activities the user keeps"
    );
}

#[tokio::test]
async fn one_strategy_can_hold_up_while_another_does_not() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..30 {
        let mood = if day == 29 {
            Mood::Awful
        } else {
            a_rising_mood_for(day)
        };
        logged(&store, &user, mood, day).await;

        let steps = if day == 29 {
            200_000
        } else {
            1_000 + day as u32 * 100
        };
        walked(&store, &user, steps, day).await;
    }

    let rows = correlations_with_rate(&store, &user, 30, 0.10).await;
    let steps = row_for(&rows, CorrelationInput::Metric(MetricKind::Steps));

    assert!(
        held_up(steps, CorrelationStrategy::Spearman),
        "the order survives one outlying day"
    );
    assert!(
        !held_up(steps, CorrelationStrategy::Pearson),
        "the straight line does not"
    );
}

#[tokio::test]
async fn a_stricter_rate_marks_less() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..30 {
        let mood = match day % 6 {
            0 | 1 => Mood::Bad,
            2 | 3 => Mood::Meh,
            _ => Mood::Good,
        };
        logged(&store, &user, mood, day).await;
        walked(&store, &user, 1_000 + day as u32 * 40, day).await;
    }

    let loose = correlations_with_rate(&store, &user, 30, 0.90).await;
    let strict = correlations_with_rate(&store, &user, 30, 0.000_001).await;

    let marked_loosely = held_up(
        row_for(&loose, CorrelationInput::Metric(MetricKind::Steps)),
        CorrelationStrategy::Spearman,
    );
    let marked_strictly = held_up(
        row_for(&strict, CorrelationInput::Metric(MetricKind::Steps)),
        CorrelationStrategy::Spearman,
    );

    assert!(marked_loosely);
    assert!(!marked_strictly);
}

async fn turn_cycle_tracking_on(store: &Arc<InMemoryStore>, user: &User) {
    let deps = application::user::use_cases::set_preferences::Deps {
        command: store.clone(),
        query: store.clone(),
    };

    application::user::use_cases::set_preferences::execute(user.id().clone(), true, &deps)
        .await
        .unwrap();
}

#[tokio::test]
async fn the_cycle_is_not_correlated_unless_it_is_tracked() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    for day in 0..40 {
        logged(&store, &user, a_mood_for(day), day).await;
    }

    let rows = correlations(&store, &user).await;

    assert!(
        !rows
            .iter()
            .any(|row| row.input == CorrelationInput::CycleProgress),
        "an untracked cycle is not an input at all"
    );
}

#[tokio::test]
async fn a_tracked_cycle_is_scored_like_anything_else_continuous() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw();
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    turn_cycle_tracking_on(&store, &user).await;

    for day in 0..40 {
        logged(&store, &user, a_rising_mood_for(day), day).await;
    }

    let cycle_deps = application::cycle::use_cases::record_cycle_start::Deps {
        command: store.clone(),
        preferences: store.clone(),
    };
    for start in [0, 28] {
        application::cycle::use_cases::record_cycle_start::execute(
            user.id().clone(),
            date_of(start),
            &cycle_deps,
        )
        .await
        .unwrap();
    }

    let rows = correlations(&store, &user).await;
    let cycle = row_for(&rows, CorrelationInput::CycleProgress);

    assert_eq!(cycle.sample_size, 40);
    assert_eq!(
        cycle
            .scores
            .iter()
            .map(|s| s.strategy.name())
            .collect::<Vec<_>>(),
        ["pearson", "spearman", "kendall"]
    );
}
