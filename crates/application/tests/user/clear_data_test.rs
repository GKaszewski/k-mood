use std::sync::Arc;

use domain::entry::{Date, DateSpan, Mood};
use domain::metric::{DailyMetric, MetricValue, Source, Steps};
use domain::ports::{
    ActivityCommandPort, CycleStartCommandPort, CycleStartQueryPort, DailyMetricCommandPort,
    DailyMetricQueryPort, MediaOwnershipPort, MoodEntryCommandPort, RejectionCommandPort,
    RejectionQueryPort, ReminderCommandPort, UserQueryPort,
};
use domain::rejection::{RejectedMetric, RejectionDetail, RejectionOrigin};
use domain::testing::{InMemoryStore, test_activity, test_entry, test_reminder, test_user};
use domain::user::User;

use application::user::use_cases::clear_data;

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

fn everything() -> DateSpan {
    DateSpan::new(on("1970-01-01"), on("9999-12-31")).unwrap()
}

async fn an_account_with_everything_in_it(store: &Arc<InMemoryStore>) -> User {
    let user = test_user("alice");
    domain::ports::UserCommandPort::save(store.as_ref(), &user)
        .await
        .unwrap();

    MoodEntryCommandPort::save(store.as_ref(), &test_entry(user.id().clone(), Mood::Good))
        .await
        .unwrap();
    ActivityCommandPort::save(store.as_ref(), &test_activity(user.id().clone(), "gaming"))
        .await
        .unwrap();
    ReminderCommandPort::save(store.as_ref(), &test_reminder(user.id().clone()))
        .await
        .unwrap();
    DailyMetricCommandPort::save(
        store.as_ref(),
        &[DailyMetric::new(
            user.id().clone(),
            on("2026-08-20"),
            MetricValue::Steps(Steps::new(8_412).unwrap()),
            Source::Manual,
        )],
    )
    .await
    .unwrap();
    CycleStartCommandPort::record(store.as_ref(), user.id(), &on("2026-08-01"))
        .await
        .unwrap();
    RejectionCommandPort::record(
        store.as_ref(),
        &[RejectedMetric::new(
            user.id().clone(),
            RejectionOrigin::Import,
            RejectionDetail::new(None, Some(on("2026-08-19")), "steps", Some(-1)),
            String::from("steps cannot be negative"),
        )],
    )
    .await
    .unwrap();
    MediaOwnershipPort::remember(
        store.as_ref(),
        user.id(),
        (&domain::attachment::PhotoId::generate()).into(),
    )
    .await
    .unwrap();

    user
}

fn deps(store: &Arc<InMemoryStore>) -> clear_data::Deps {
    clear_data::Deps {
        cascade: store.clone(),
        media_storage: store.clone(),
        media_ownership: store.clone(),
    }
}

#[tokio::test]
async fn clearing_leaves_nothing_the_user_logged() {
    let store = Arc::new(InMemoryStore::new());
    let user = an_account_with_everything_in_it(&store).await;

    clear_data::execute(user.id().clone(), &deps(&store))
        .await
        .unwrap();

    assert_eq!(store.entry_count(), 0, "entries");
    assert_eq!(store.activity_count(), 0, "activities");
    assert_eq!(store.reminder_count(), 0, "reminders");

    let metrics = DailyMetricQueryPort::find_by_span(store.as_ref(), user.id(), &everything())
        .await
        .unwrap();
    assert!(metrics.is_empty(), "daily metrics");

    let starts = CycleStartQueryPort::find_by_user(store.as_ref(), user.id())
        .await
        .unwrap();
    assert!(starts.is_empty(), "cycle starts");

    let rejections = RejectionQueryPort::find_recent_by_user(store.as_ref(), user.id())
        .await
        .unwrap();
    assert!(rejections.is_empty(), "rejected metrics");

    let media = MediaOwnershipPort::owned_by(store.as_ref(), user.id())
        .await
        .unwrap();
    assert!(media.is_empty(), "media ownership");
}

#[tokio::test]
async fn clearing_keeps_the_account_itself() {
    let store = Arc::new(InMemoryStore::new());
    let user = an_account_with_everything_in_it(&store).await;

    clear_data::execute(user.id().clone(), &deps(&store))
        .await
        .unwrap();

    assert!(
        UserQueryPort::find_by_id(store.as_ref(), user.id())
            .await
            .unwrap()
            .is_some(),
        "clearing data is not deleting the account"
    );
}

#[tokio::test]
async fn clearing_one_account_leaves_another_alone() {
    let store = Arc::new(InMemoryStore::new());
    let alice = an_account_with_everything_in_it(&store).await;

    let bob = test_user("bob");
    domain::ports::UserCommandPort::save(store.as_ref(), &bob)
        .await
        .unwrap();
    MoodEntryCommandPort::save(store.as_ref(), &test_entry(bob.id().clone(), Mood::Rad))
        .await
        .unwrap();
    CycleStartCommandPort::record(store.as_ref(), bob.id(), &on("2026-07-01"))
        .await
        .unwrap();

    clear_data::execute(alice.id().clone(), &deps(&store))
        .await
        .unwrap();

    assert_eq!(store.entry_count(), 1, "bob's entry must survive");
    let starts = CycleStartQueryPort::find_by_user(store.as_ref(), bob.id())
        .await
        .unwrap();
    assert_eq!(starts.len(), 1, "bob's cycle start must survive");
}
