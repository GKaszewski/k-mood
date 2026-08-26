use std::sync::Arc;

use domain::entry::{Date, DateSpan};
use domain::metric::{MetricValue, Source, Steps};
use domain::ports::{
    DailyMetricCommandPort, DailyMetricQueryPort, RejectionQueryPort, UserCommandPort,
};
use domain::provider::ProviderName;
use domain::rejection::RejectionOrigin;
use domain::testing::{InMemoryStore, test_user};
use domain::user::{Timezone, User};

use application::import::commands::{ImportDailyMetricsCommand, ImportedDay, ImportedMetric};
use application::import::use_cases::import_daily_metrics::{self, ImportOutcome};

const DAY_LIMIT: usize = 90;

async fn a_user_in_warsaw(store: &Arc<InMemoryStore>) -> User {
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    user
}

fn healthkit() -> ProviderName {
    ProviderName::new("healthkit").unwrap()
}

fn metric(kind: &str, value: Option<i64>) -> ImportedMetric {
    ImportedMetric {
        kind: kind.to_string(),
        value,
    }
}

fn day(date: &str, metrics: Vec<ImportedMetric>) -> ImportedDay {
    ImportedDay {
        date: date.to_string(),
        metrics,
    }
}

async fn import(
    store: &Arc<InMemoryStore>,
    user: &User,
    days: Vec<ImportedDay>,
) -> Result<ImportOutcome, application::errors::ApplicationError> {
    let deps = import_daily_metrics::Deps {
        metrics: store.clone(),
        rejections: store.clone(),
    };

    import_daily_metrics::execute(
        ImportDailyMetricsCommand {
            user_id: user.id().clone(),
            provider: healthkit(),
            days,
            maximum_days: DAY_LIMIT,
        },
        &deps,
    )
    .await
}

async fn stored(store: &Arc<InMemoryStore>, user: &User, from: &str, to: &str) -> Vec<String> {
    let span = DateSpan::new(
        Date::from_persistence(from.parse().unwrap()),
        Date::from_persistence(to.parse().unwrap()),
    )
    .unwrap();

    let metrics = DailyMetricQueryPort::find_by_span(store.as_ref(), user.id(), &span)
        .await
        .unwrap();

    let mut described: Vec<String> = metrics
        .iter()
        .map(|metric| {
            format!(
                "{} {} {}",
                metric.date(),
                metric.kind().name(),
                metric.value().count()
            )
        })
        .collect();
    described.sort();
    described
}

#[tokio::test]
async fn a_whole_day_of_readings_is_stored_under_the_providers_name() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let outcome = import(
        &store,
        &user,
        vec![day(
            "2026-08-20",
            vec![
                metric("steps", Some(8_412)),
                metric("sleepMinutes", Some(447)),
            ],
        )],
    )
    .await
    .unwrap();

    assert_eq!(outcome.accepted, 2);
    assert!(outcome.rejected.is_empty());
    assert_eq!(
        stored(&store, &user, "2026-08-20", "2026-08-20").await,
        ["2026-08-20 sleepMinutes 447", "2026-08-20 steps 8412"]
    );
}

#[tokio::test]
async fn one_bad_reading_does_not_cost_the_good_ones() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let outcome = import(
        &store,
        &user,
        vec![day(
            "2026-08-20",
            vec![
                metric("steps", Some(8_412)),
                metric("hrv", Some(9_999)),
                metric("sleepMinutes", Some(447)),
            ],
        )],
    )
    .await
    .unwrap();

    assert_eq!(outcome.accepted, 2);
    assert_eq!(outcome.rejected.len(), 1);
    assert_eq!(outcome.rejected[0].kind, "hrv");
    assert!(outcome.rejected[0].reason.contains("between"));
    assert_eq!(
        stored(&store, &user, "2026-08-20", "2026-08-20").await,
        ["2026-08-20 sleepMinutes 447", "2026-08-20 steps 8412"]
    );
}

#[tokio::test]
async fn a_rejection_is_written_where_the_user_can_find_it() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    import(
        &store,
        &user,
        vec![day("2026-08-20", vec![metric("hrv", Some(9_999))])],
    )
    .await
    .unwrap();

    let trace = RejectionQueryPort::find_recent_by_user(store.as_ref(), user.id())
        .await
        .unwrap();

    assert_eq!(trace.len(), 1);
    assert_eq!(trace[0].origin(), RejectionOrigin::Import);
    assert_eq!(trace[0].detail().kind(), "hrv");
    assert_eq!(trace[0].detail().value(), Some(9_999));
    assert_eq!(trace[0].detail().provider(), Some(&healthkit()));
}

#[tokio::test]
async fn a_kind_this_build_does_not_know_is_recorded_rather_than_dropped() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let outcome = import(
        &store,
        &user,
        vec![day(
            "2026-08-20",
            vec![metric("bloodOxygen", Some(98)), metric("steps", Some(500))],
        )],
    )
    .await
    .unwrap();

    assert_eq!(outcome.accepted, 1);
    assert_eq!(outcome.rejected.len(), 1);
    assert!(outcome.rejected[0].reason.contains("unknown"));

    let trace = RejectionQueryPort::find_recent_by_user(store.as_ref(), user.id())
        .await
        .unwrap();
    assert_eq!(trace[0].detail().kind(), "bloodOxygen");
}

#[tokio::test]
async fn an_unreadable_date_costs_only_that_day() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let outcome = import(
        &store,
        &user,
        vec![
            day("yesterday", vec![metric("steps", Some(100))]),
            day("2026-08-20", vec![metric("steps", Some(8_412))]),
        ],
    )
    .await
    .unwrap();

    assert_eq!(outcome.accepted, 1);
    assert_eq!(outcome.rejected.len(), 1);
    assert!(outcome.rejected[0].reason.contains("date"));
    assert_eq!(
        stored(&store, &user, "2026-08-01", "2026-08-31").await,
        ["2026-08-20 steps 8412"]
    );
}

#[tokio::test]
async fn an_importer_may_not_clear_a_reading_and_is_told_why() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let outcome = import(
        &store,
        &user,
        vec![day("2026-08-20", vec![metric("steps", None)])],
    )
    .await
    .unwrap();

    assert_eq!(outcome.accepted, 0);
    assert_eq!(outcome.rejected.len(), 1);
    assert!(outcome.rejected[0].reason.contains("clear"));
}

#[tokio::test]
async fn what_the_user_stated_by_hand_is_reported_as_superseding_the_import() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let stated = domain::metric::DailyMetric::new(
        user.id().clone(),
        Date::from_persistence("2026-08-20".parse().unwrap()),
        MetricValue::Steps(Steps::new(8_412).unwrap()),
        Source::Manual,
    );
    DailyMetricCommandPort::save(store.as_ref(), &[stated])
        .await
        .unwrap();

    let outcome = import(
        &store,
        &user,
        vec![day("2026-08-20", vec![metric("steps", Some(1))])],
    )
    .await
    .unwrap();

    assert_eq!(outcome.accepted, 0);
    assert_eq!(outcome.superseded, 1);
    assert!(
        outcome.rejected.is_empty(),
        "being superseded is not a rejection"
    );
    assert_eq!(
        stored(&store, &user, "2026-08-20", "2026-08-20").await,
        ["2026-08-20 steps 8412"]
    );
}

#[tokio::test]
async fn a_backfill_longer_than_the_limit_is_refused_outright() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let days: Vec<ImportedDay> = (1..=DAY_LIMIT + 1)
        .map(|number| {
            day(
                &format!("2026-01-{:02}", (number % 28) + 1),
                vec![metric("steps", Some(100))],
            )
        })
        .collect();

    let refused = import(&store, &user, days).await.unwrap_err();

    assert!(refused.to_string().contains("90"), "got {refused}");
    assert!(
        stored(&store, &user, "2026-01-01", "2026-12-31")
            .await
            .is_empty()
    );
}

#[tokio::test]
async fn posting_the_same_day_again_replaces_rather_than_duplicates() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    for count in [8_000, 8_412] {
        import(
            &store,
            &user,
            vec![day("2026-08-20", vec![metric("steps", Some(count))])],
        )
        .await
        .unwrap();
    }

    assert_eq!(
        stored(&store, &user, "2026-08-20", "2026-08-20").await,
        ["2026-08-20 steps 8412"]
    );
}
