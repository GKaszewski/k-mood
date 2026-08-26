use std::sync::Arc;

use domain::entry::Date;
use domain::ports::UserCommandPort;
use domain::testing::{InMemoryStore, test_user};
use domain::user::{Timezone, User};

use application::cycle::use_cases::read_cycle::CycleView;
use application::cycle::use_cases::{forget_cycle_start, read_cycle, record_cycle_start};
use application::user::use_cases::set_preferences;

async fn a_user_in_warsaw(store: &Arc<InMemoryStore>) -> User {
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    user
}

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

async fn turn_tracking(store: &Arc<InMemoryStore>, user: &User, on: bool) {
    let deps = set_preferences::Deps {
        command: store.clone(),
        query: store.clone(),
    };

    set_preferences::execute(user.id().clone(), on, &deps)
        .await
        .unwrap();
}

async fn record(
    store: &Arc<InMemoryStore>,
    user: &User,
    day: &str,
) -> Result<(), application::errors::ApplicationError> {
    let deps = record_cycle_start::Deps {
        command: store.clone(),
        preferences: store.clone(),
    };

    record_cycle_start::execute(user.id().clone(), on(day), &deps).await
}

async fn forget(store: &Arc<InMemoryStore>, user: &User, day: &str) {
    let deps = forget_cycle_start::Deps {
        command: store.clone(),
    };

    forget_cycle_start::execute(user.id().clone(), on(day), &deps)
        .await
        .unwrap();
}

async fn read(store: &Arc<InMemoryStore>, user: &User) -> CycleView {
    let deps = read_cycle::Deps {
        query: store.clone(),
        preferences: store.clone(),
        users: store.clone(),
    };

    read_cycle::execute(user.id().clone(), &deps).await.unwrap()
}

#[tokio::test]
async fn tracking_is_off_until_it_is_turned_on() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let view = read(&store, &user).await;

    assert!(!view.tracking);
    assert!(view.starts.is_empty());
    assert!(view.today.is_none());
}

#[tokio::test]
async fn nothing_can_be_recorded_while_tracking_is_off() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;

    let refused = record(&store, &user, "2026-01-01").await.unwrap_err();

    assert!(refused.to_string().contains("off"), "got {refused}");
    assert!(read(&store, &user).await.starts.is_empty());
}

#[tokio::test]
async fn a_recorded_start_comes_back_in_order() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;
    turn_tracking(&store, &user, true).await;

    record(&store, &user, "2026-01-29").await.unwrap();
    record(&store, &user, "2026-01-01").await.unwrap();

    let view = read(&store, &user).await;

    assert_eq!(
        view.starts
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>(),
        ["2026-01-01", "2026-01-29"]
    );
}

#[tokio::test]
async fn recording_the_same_day_twice_records_it_once() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;
    turn_tracking(&store, &user, true).await;

    record(&store, &user, "2026-01-01").await.unwrap();
    record(&store, &user, "2026-01-01").await.unwrap();

    assert_eq!(read(&store, &user).await.starts.len(), 1);
}

#[tokio::test]
async fn a_start_can_be_taken_back() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;
    turn_tracking(&store, &user, true).await;
    record(&store, &user, "2026-01-01").await.unwrap();

    forget(&store, &user, "2026-01-01").await;

    assert!(read(&store, &user).await.starts.is_empty());
}

#[tokio::test]
async fn turning_tracking_off_hides_the_cycle_without_forgetting_it() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in_warsaw(&store).await;
    turn_tracking(&store, &user, true).await;
    record(&store, &user, "2026-01-01").await.unwrap();

    turn_tracking(&store, &user, false).await;
    let hidden = read(&store, &user).await;

    turn_tracking(&store, &user, true).await;
    let shown = read(&store, &user).await;

    assert!(!hidden.tracking);
    assert!(
        hidden.starts.is_empty(),
        "nothing is offered while it is off"
    );
    assert_eq!(shown.starts.len(), 1, "and nothing was thrown away");
}

#[tokio::test]
async fn one_accounts_cycle_is_not_anothers() {
    let store = Arc::new(InMemoryStore::new());
    let mine = a_user_in_warsaw(&store).await;

    let mut theirs = test_user("bob");
    theirs.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &theirs)
        .await
        .unwrap();

    turn_tracking(&store, &mine, true).await;
    turn_tracking(&store, &theirs, true).await;
    record(&store, &mine, "2026-01-01").await.unwrap();

    assert_eq!(read(&store, &mine).await.starts.len(), 1);
    assert!(read(&store, &theirs).await.starts.is_empty());
}
