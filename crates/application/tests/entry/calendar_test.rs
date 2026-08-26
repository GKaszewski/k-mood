use std::sync::Arc;

use chrono::{DateTime, FixedOffset};

use domain::entry::{DateRange, Mood, MoodEntry};
use domain::ports::{MoodEntryCommandPort, UserCommandPort};
use domain::testing::{InMemoryStore, test_user};
use domain::user::{Timezone, User};

use application::entry::use_cases::get_calendar::{self, CalendarDay};

fn user_in(name: &str, timezone: &str) -> User {
    let mut user = test_user(name);
    user.update_timezone(Some(Timezone::new(timezone).unwrap()));
    user
}

fn at(instant: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(instant).unwrap()
}

async fn calendar_of(store: &Arc<InMemoryStore>, user: &User) -> Vec<CalendarDay> {
    let deps = get_calendar::Deps {
        query: store.clone(),
        dimensions: Vec::new(),
        cycles: store.clone(),
        preferences: store.clone(),
        users: store.clone(),
    };

    let range = DateRange::new(
        at("2026-08-01T00:00:00+00:00"),
        at("2026-08-31T23:59:59+00:00"),
    )
    .unwrap();

    get_calendar::execute(user.id().clone(), range, &deps)
        .await
        .unwrap()
}

async fn logged(store: &Arc<InMemoryStore>, user: &User, mood: Mood, instant: &str) {
    let entry = MoodEntry::new(user.id().clone(), mood, at(instant));
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();
}

#[tokio::test]
async fn a_day_of_one_awful_and_one_rad_does_not_report_rad() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    logged(&store, &user, Mood::Awful, "2026-08-20T09:00:00+02:00").await;
    logged(&store, &user, Mood::Rad, "2026-08-20T21:00:00+02:00").await;

    let days = calendar_of(&store, &user).await;

    assert_eq!(days.len(), 1);
    let mood = days[0].day_mood.expect("the day has entries");
    assert_eq!(mood.rounded(), Mood::Meh);
    assert!((mood.value() - 3.0).abs() < f64::EPSILON);
}

#[tokio::test]
async fn only_days_that_were_logged_appear() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    logged(&store, &user, Mood::Good, "2026-08-20T09:00:00+02:00").await;
    logged(&store, &user, Mood::Bad, "2026-08-24T09:00:00+02:00").await;

    let days = calendar_of(&store, &user).await;
    let dates: Vec<String> = days.iter().map(|day| day.date.to_string()).collect();

    assert_eq!(dates, ["2026-08-20", "2026-08-24"]);
}

#[tokio::test]
async fn a_late_evening_entry_belongs_to_the_day_the_user_was_living() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    logged(&store, &user, Mood::Good, "2026-08-20T23:30:00+00:00").await;

    let days = calendar_of(&store, &user).await;
    let dates: Vec<String> = days.iter().map(|day| day.date.to_string()).collect();

    assert_eq!(dates, ["2026-08-21"]);
}

#[tokio::test]
async fn every_entry_on_a_day_moves_its_mood() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    logged(&store, &user, Mood::Meh, "2026-08-20T08:00:00+02:00").await;
    logged(&store, &user, Mood::Meh, "2026-08-20T12:00:00+02:00").await;
    logged(&store, &user, Mood::Meh, "2026-08-20T16:00:00+02:00").await;
    logged(&store, &user, Mood::Meh, "2026-08-20T20:00:00+02:00").await;
    logged(&store, &user, Mood::Awful, "2026-08-20T23:00:00+02:00").await;

    let days = calendar_of(&store, &user).await;
    let mood = days[0].day_mood.expect("the day has entries");

    assert!(mood.value() < 3.0, "the awful entry was discarded");
}

async fn tracking_cycle_from(store: &Arc<InMemoryStore>, user: &User, starts: &[&str]) {
    let preference_deps = application::user::use_cases::set_preferences::Deps {
        command: store.clone(),
        query: store.clone(),
    };
    application::user::use_cases::set_preferences::execute(
        user.id().clone(),
        true,
        &preference_deps,
    )
    .await
    .unwrap();

    let cycle_deps = application::cycle::use_cases::record_cycle_start::Deps {
        command: store.clone(),
        preferences: store.clone(),
    };
    for start in starts {
        application::cycle::use_cases::record_cycle_start::execute(
            user.id().clone(),
            domain::entry::Date::from_persistence(start.parse().unwrap()),
            &cycle_deps,
        )
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn a_day_carries_no_cycle_day_when_the_cycle_is_not_tracked() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    tracking_cycle_from(&store, &user, &["2026-08-11"]).await;

    let turn_off = application::user::use_cases::set_preferences::Deps {
        command: store.clone(),
        query: store.clone(),
    };
    application::user::use_cases::set_preferences::execute(user.id().clone(), false, &turn_off)
        .await
        .unwrap();

    logged(&store, &user, Mood::Good, "2026-08-20T09:00:00+02:00").await;

    let days = calendar_of(&store, &user).await;

    assert!(
        days[0].cycle_day.is_none(),
        "a recorded start must stay invisible while tracking is off"
    );
}

#[tokio::test]
async fn a_tracked_day_carries_the_cycle_day_derived_for_it() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    tracking_cycle_from(&store, &user, &["2026-08-11"]).await;

    logged(&store, &user, Mood::Good, "2026-08-20T09:00:00+02:00").await;

    let days = calendar_of(&store, &user).await;

    assert_eq!(days[0].cycle_day.map(|day| day.value()), Some(10));
}

#[tokio::test]
async fn a_day_before_the_first_recorded_start_has_no_cycle_day() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    tracking_cycle_from(&store, &user, &["2026-08-25"]).await;

    logged(&store, &user, Mood::Good, "2026-08-20T09:00:00+02:00").await;

    let days = calendar_of(&store, &user).await;

    assert!(days[0].cycle_day.is_none());
}

#[tokio::test]
async fn correcting_a_start_changes_the_cycle_day_of_every_affected_date() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    tracking_cycle_from(&store, &user, &["2026-08-11"]).await;

    logged(&store, &user, Mood::Good, "2026-08-20T09:00:00+02:00").await;
    logged(&store, &user, Mood::Meh, "2026-08-22T09:00:00+02:00").await;

    let before: Vec<u16> = calendar_of(&store, &user)
        .await
        .iter()
        .filter_map(|day| day.cycle_day.map(|cycle| cycle.value()))
        .collect();

    let forget = application::cycle::use_cases::forget_cycle_start::Deps {
        command: store.clone(),
    };
    application::cycle::use_cases::forget_cycle_start::execute(
        user.id().clone(),
        domain::entry::Date::from_persistence("2026-08-11".parse().unwrap()),
        &forget,
    )
    .await
    .unwrap();
    tracking_cycle_from(&store, &user, &["2026-08-13"]).await;

    let after: Vec<u16> = calendar_of(&store, &user)
        .await
        .iter()
        .filter_map(|day| day.cycle_day.map(|cycle| cycle.value()))
        .collect();

    assert_eq!(before, [10, 12]);
    assert_eq!(after, [8, 10], "one correction moved both days");
}
