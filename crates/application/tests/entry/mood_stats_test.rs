use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::{MoodEntryCommandPort, UserCommandPort};
use domain::testing::{InMemoryStore, test_entry_days_ago, test_user};
use domain::user::Timezone;

use application::entry::queries::MoodStatsQuery;
use application::entry::use_cases::get_mood_stats;

#[tokio::test]
async fn computes_stats_across_entries() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("alice", "Europe/Warsaw");

    MoodEntryCommandPort::save(
        &*store,
        &test_entry_days_ago(user.id().clone(), Mood::Good, 0),
    )
    .await
    .unwrap();
    MoodEntryCommandPort::save(
        &*store,
        &test_entry_days_ago(user.id().clone(), Mood::Rad, 1),
    )
    .await
    .unwrap();
    MoodEntryCommandPort::save(
        &*store,
        &test_entry_days_ago(user.id().clone(), Mood::Meh, 2),
    )
    .await
    .unwrap();

    UserCommandPort::save(&*store, &user).await.unwrap();

    let deps = get_mood_stats::Deps {
        query: store.clone(),
        users: store.clone(),
    };

    let query = MoodStatsQuery {
        user_id: user.id().clone(),
        range: None,
    };

    let stats = get_mood_stats::execute(query, &deps).await.unwrap();

    assert_eq!(stats.total_entries, 3);
    assert!(stats.average.is_some());
    assert_eq!(stats.current_streak, 3);
    assert!(!stats.frequency.is_empty());
}

#[tokio::test]
async fn a_user_with_no_entries_has_zero_stats() {
    let store = Arc::new(InMemoryStore::new());
    let user = user_in("newcomer", "Europe/Warsaw");
    UserCommandPort::save(&*store, &user).await.unwrap();

    let deps = get_mood_stats::Deps {
        query: store.clone(),
        users: store.clone(),
    };

    let query = MoodStatsQuery {
        user_id: user.id().clone(),
        range: None,
    };

    let stats = get_mood_stats::execute(query, &deps).await.unwrap();

    assert_eq!(stats.total_entries, 0);
    assert!(stats.average.is_none());
    assert_eq!(stats.current_streak, 0);
}

fn user_in(name: &str, timezone: &str) -> domain::user::User {
    let mut user = test_user(name);
    user.update_timezone(Some(Timezone::new(timezone).unwrap()));
    user
}
