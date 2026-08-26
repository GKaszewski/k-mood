use std::sync::Arc;

use domain::entry::{Date, DateSpan};
use domain::metric::{
    AlcoholicDrinks, AwakeMinutes, DailyMetric, ExerciseMinutes, Hrv, MetricKind, MetricValue,
    RestingHeartRate, ScreenTimeMinutes, SleepMinutes, Source, Steps,
};
use domain::ports::{
    CascadeDeletePort, DailyMetricCommandPort, DailyMetricQueryPort, UserCommandPort, UserQueryPort,
};
use domain::provider::ProviderName;
use domain::testing::{InMemoryStore, test_user};
use domain::user::{Timezone, UserId};

use application::metric::commands::{MetricChange, SetDailyMetricsCommand};
use application::metric::use_cases::{list_daily_metrics, set_daily_metrics};

async fn a_user_in_warsaw(store: &Arc<InMemoryStore>) -> UserId {
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    user.id().clone()
}

fn set_deps(store: &Arc<InMemoryStore>) -> set_daily_metrics::Deps {
    set_daily_metrics::Deps {
        metrics: store.clone() as Arc<dyn DailyMetricCommandPort>,
        users: store.clone() as Arc<dyn UserQueryPort>,
    }
}

fn list_deps(store: &Arc<InMemoryStore>) -> list_daily_metrics::Deps {
    list_daily_metrics::Deps {
        metrics: store.clone() as Arc<dyn DailyMetricQueryPort>,
    }
}

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

fn steps(count: u32) -> MetricValue {
    MetricValue::Steps(Steps::new(count).unwrap())
}

async fn on_that_day(store: &Arc<InMemoryStore>, user_id: &UserId) -> Vec<DailyMetric> {
    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    list_daily_metrics::execute(user_id.clone(), span, &list_deps(store))
        .await
        .unwrap()
}

async fn only_metric(store: &Arc<InMemoryStore>, user_id: &UserId, day: &str) -> DailyMetric {
    let span = DateSpan::new(on(day), on(day)).unwrap();
    let stored = list_daily_metrics::execute(user_id.clone(), span, &list_deps(store))
        .await
        .unwrap();

    assert_eq!(stored.len(), 1);
    stored.into_iter().next().unwrap()
}

#[tokio::test]
async fn a_step_count_the_user_states_is_stored_for_that_date() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].value(), &steps(8_412));
    assert_eq!(stored[0].source(), &Source::Manual);
}

#[tokio::test]
async fn an_import_does_not_overwrite_the_count_the_user_stated() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let imported = DailyMetric::new(
        user_id.clone(),
        on("2026-08-20"),
        steps(1_000),
        Source::Provider(ProviderName::new("healthkit").unwrap()),
    );
    DailyMetricCommandPort::save(store.as_ref(), &[imported])
        .await
        .unwrap();

    let stored = only_metric(&store, &user_id, "2026-08-20").await;

    assert_eq!(stored.value(), &steps(8_412));
    assert_eq!(stored.source(), &Source::Manual);
}

#[tokio::test]
async fn a_count_the_user_states_replaces_what_a_provider_reported() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    let imported = DailyMetric::new(
        user_id.clone(),
        on("2026-08-20"),
        steps(1_000),
        Source::Provider(ProviderName::new("healthkit").unwrap()),
    );
    DailyMetricCommandPort::save(store.as_ref(), &[imported])
        .await
        .unwrap();

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let stored = only_metric(&store, &user_id, "2026-08-20").await;

    assert_eq!(stored.value(), &steps(8_412));
    assert_eq!(stored.source(), &Source::Manual);
}

#[tokio::test]
async fn restating_a_date_corrects_it_rather_than_recording_it_twice() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    for count in [8_000, 8_412] {
        set_daily_metrics::execute(
            SetDailyMetricsCommand {
                user_id: user_id.clone(),
                date: on("2026-08-20"),
                changes: vec![MetricChange::Stated(steps(count))],
                source: Source::Manual,
            },
            &set_deps(&store),
        )
        .await
        .unwrap();
    }

    let stored = only_metric(&store, &user_id, "2026-08-20").await;

    assert_eq!(stored.value(), &steps(8_412));
}

#[tokio::test]
async fn a_date_that_has_not_happened_yet_is_refused() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    let error = set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id,
            date: on("2099-01-01"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("has not happened yet"));
}

#[tokio::test]
async fn an_account_with_no_timezone_cannot_record_a_day() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("bob");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let error = set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user.id().clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("timezone"));
}

#[tokio::test]
async fn only_the_days_in_the_span_come_back() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    for day in ["2026-08-19", "2026-08-20", "2026-08-21"] {
        set_daily_metrics::execute(
            SetDailyMetricsCommand {
                user_id: user_id.clone(),
                date: on(day),
                changes: vec![MetricChange::Stated(steps(8_412))],
                source: Source::Manual,
            },
            &set_deps(&store),
        )
        .await
        .unwrap();
    }

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-21")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    let mut days: Vec<String> = stored
        .iter()
        .map(|metric| metric.date().to_string())
        .collect();
    days.sort();

    assert_eq!(days, vec!["2026-08-20", "2026-08-21"]);
}

#[tokio::test]
async fn another_users_days_are_not_returned() {
    let store = Arc::new(InMemoryStore::new());
    let mine = a_user_in_warsaw(&store).await;
    let theirs = UserId::generate();

    let hers = DailyMetric::new(theirs, on("2026-08-20"), steps(1_000), Source::Manual);
    DailyMetricCommandPort::save(store.as_ref(), &[hers])
        .await
        .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(mine, span, &list_deps(&store))
        .await
        .unwrap();

    assert!(stored.is_empty());
}

#[tokio::test]
async fn clearing_a_users_data_removes_their_days() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    CascadeDeletePort::delete_all_user_data(store.as_ref(), &user_id)
        .await
        .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    assert!(stored.is_empty());
}

fn one_of_every_kind() -> Vec<MetricChange> {
    [
        MetricValue::Steps(Steps::new(8_412).unwrap()),
        MetricValue::SleepMinutes(SleepMinutes::new(447).unwrap()),
        MetricValue::AwakeMinutes(AwakeMinutes::new(23).unwrap()),
        MetricValue::RestingHeartRate(RestingHeartRate::new(52).unwrap()),
        MetricValue::Hrv(Hrv::new(61).unwrap()),
        MetricValue::ExerciseMinutes(ExerciseMinutes::new(35).unwrap()),
        MetricValue::ScreenTimeMinutes(ScreenTimeMinutes::new(212).unwrap()),
        MetricValue::AlcoholicDrinks(AlcoholicDrinks::new(2).unwrap()),
    ]
    .into_iter()
    .map(MetricChange::Stated)
    .collect()
}

#[tokio::test]
async fn every_kind_can_be_stated_and_read_back_on_one_day() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: one_of_every_kind(),
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    let mut names: Vec<&str> = stored.iter().map(|metric| metric.kind().name()).collect();
    names.sort_unstable();

    assert_eq!(
        names,
        [
            "alcoholicDrinks",
            "awakeMinutes",
            "exerciseMinutes",
            "hrv",
            "restingHeartRate",
            "screenTimeMinutes",
            "sleepMinutes",
            "steps",
        ]
    );

    let values: Vec<&MetricValue> = stored.iter().map(|metric| metric.value()).collect();
    for change in one_of_every_kind() {
        let MetricChange::Stated(expected) = change else {
            unreachable!("every kind is stated, never cleared, in this test")
        };
        assert!(
            values.contains(&&expected),
            "{expected:?} did not come back"
        );
    }
}

#[tokio::test]
async fn stating_one_kind_leaves_another_providers_reading_alone() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    let reported = DailyMetric::new(
        user_id.clone(),
        on("2026-08-20"),
        MetricValue::Hrv(Hrv::new(61).unwrap()),
        Source::Provider(ProviderName::new("healthkit").unwrap()),
    );
    DailyMetricCommandPort::save(store.as_ref(), &[reported])
        .await
        .unwrap();

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    let hrv = stored
        .iter()
        .find(|metric| metric.kind() == MetricKind::Hrv)
        .expect("the provider's reading was lost");

    assert_eq!(hrv.value(), &MetricValue::Hrv(Hrv::new(61).unwrap()));
    assert!(hrv.source().provider().is_some());
    assert_eq!(stored.len(), 2);
}

#[tokio::test]
async fn clearing_a_kind_removes_what_was_stated() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Cleared(MetricKind::Steps)],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    assert!(stored.is_empty());
}

#[tokio::test]
async fn clearing_a_kind_that_was_never_recorded_changes_nothing() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Cleared(MetricKind::Hrv)],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let span = DateSpan::new(on("2026-08-20"), on("2026-08-20")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    assert!(stored.is_empty());
}

#[tokio::test]
async fn a_providers_reading_can_be_cleared_and_a_later_import_may_return_it() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    let reading = || {
        DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(1_000),
            Source::Provider(ProviderName::new("healthkit").unwrap()),
        )
    };

    DailyMetricCommandPort::save(store.as_ref(), &[reading()])
        .await
        .unwrap();

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Cleared(MetricKind::Steps)],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    assert!(
        on_that_day(&store, &user_id).await.is_empty(),
        "clearing left the provider's reading behind"
    );

    DailyMetricCommandPort::save(store.as_ref(), &[reading()])
        .await
        .unwrap();

    let stored = on_that_day(&store, &user_id).await;

    assert_eq!(stored.len(), 1);
    assert!(stored[0].source().provider().is_some());
}

#[tokio::test]
async fn one_request_can_state_one_kind_and_clear_another() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![
                MetricChange::Stated(steps(8_412)),
                MetricChange::Stated(MetricValue::Hrv(Hrv::new(61).unwrap())),
            ],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![
                MetricChange::Stated(steps(9_000)),
                MetricChange::Cleared(MetricKind::Hrv),
            ],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let stored = on_that_day(&store, &user_id).await;

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].value(), &steps(9_000));
}

#[tokio::test]
async fn naming_a_kind_twice_in_one_request_is_refused() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    let error = set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id,
            date: on("2026-08-20"),
            changes: vec![
                MetricChange::Stated(steps(8_412)),
                MetricChange::Cleared(MetricKind::Steps),
            ],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("more than once"));
}

#[tokio::test]
async fn clearing_one_kind_leaves_the_others_on_that_day_alone() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: one_of_every_kind(),
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Cleared(MetricKind::Hrv)],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let stored = on_that_day(&store, &user_id).await;
    let remaining: Vec<MetricKind> = stored.iter().map(|metric| metric.kind()).collect();

    assert_eq!(stored.len(), 7);
    assert!(!remaining.contains(&MetricKind::Hrv));
    assert!(remaining.contains(&MetricKind::Steps));
}

#[tokio::test]
async fn clearing_a_kind_on_one_day_leaves_the_same_kind_on_other_days() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    for day in ["2026-08-19", "2026-08-20", "2026-08-21"] {
        set_daily_metrics::execute(
            SetDailyMetricsCommand {
                user_id: user_id.clone(),
                date: on(day),
                changes: vec![MetricChange::Stated(steps(8_412))],
                source: Source::Manual,
            },
            &set_deps(&store),
        )
        .await
        .unwrap();
    }

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Cleared(MetricKind::Steps)],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let span = DateSpan::new(on("2026-08-19"), on("2026-08-21")).unwrap();
    let stored = list_daily_metrics::execute(user_id, span, &list_deps(&store))
        .await
        .unwrap();

    let mut days: Vec<String> = stored
        .iter()
        .map(|metric| metric.date().to_string())
        .collect();
    days.sort();

    assert_eq!(days, ["2026-08-19", "2026-08-21"]);
}

fn healthkit() -> Source {
    Source::Provider(ProviderName::new("healthkit").unwrap())
}

#[tokio::test]
async fn an_importer_writes_under_its_own_name() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: healthkit(),
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let stored = only_metric(&store, &user_id, "2026-08-20").await;

    assert_eq!(stored.source(), &healthkit());
}

#[tokio::test]
async fn an_importer_cannot_clear_a_reading() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = a_user_in_warsaw(&store).await;

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Stated(steps(8_412))],
            source: Source::Manual,
        },
        &set_deps(&store),
    )
    .await
    .unwrap();

    let refused = set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: user_id.clone(),
            date: on("2026-08-20"),
            changes: vec![MetricChange::Cleared(MetricKind::Steps)],
            source: healthkit(),
        },
        &set_deps(&store),
    )
    .await
    .unwrap_err();

    assert!(refused.to_string().contains("clear"), "got {refused}");
    assert_eq!(
        only_metric(&store, &user_id, "2026-08-20").await.value(),
        &steps(8_412),
        "the reading must survive an importer trying to clear it"
    );
}
