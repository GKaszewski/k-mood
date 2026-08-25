use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{InMemoryStore, test_entry_days_ago, test_user};

use application::entry::queries::MoodStatsQuery;
use application::entry::use_cases::get_mood_stats;

#[tokio::test]
async fn computes_stats_across_entries() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");

    store
        .save(&test_entry_days_ago(user.id().clone(), Mood::Good, 0))
        .await
        .unwrap();
    store
        .save(&test_entry_days_ago(user.id().clone(), Mood::Rad, 1))
        .await
        .unwrap();
    store
        .save(&test_entry_days_ago(user.id().clone(), Mood::Meh, 2))
        .await
        .unwrap();

    let deps = get_mood_stats::Deps {
        query: store.clone(),
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
async fn empty_entries_give_zero_stats() {
    let store = Arc::new(InMemoryStore::new());
    let deps = get_mood_stats::Deps {
        query: store.clone(),
    };

    let query = MoodStatsQuery {
        user_id: test_user("nobody").id().clone(),
        range: None,
    };

    let stats = get_mood_stats::execute(query, &deps).await.unwrap();

    assert_eq!(stats.total_entries, 0);
    assert!(stats.average.is_none());
    assert_eq!(stats.current_streak, 0);
}
