use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{InMemoryStore, test_entry, test_user};

use application::entry::queries::ListEntriesQuery;
use application::entry::use_cases::list_entries;

#[tokio::test]
async fn lists_entries_for_user() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");
    let bob = test_user("bob");

    store
        .save(&test_entry(alice.id().clone(), Mood::Good))
        .await
        .unwrap();
    store
        .save(&test_entry(alice.id().clone(), Mood::Rad))
        .await
        .unwrap();
    store
        .save(&test_entry(bob.id().clone(), Mood::Meh))
        .await
        .unwrap();

    let deps = list_entries::Deps {
        query: store.clone(),
    };

    let query = ListEntriesQuery {
        limit: None,
        offset: None,
        user_id: alice.id().clone(),
        range: None,
    };

    let entries = list_entries::execute(query, &deps).await.unwrap();
    assert_eq!(entries.len(), 2);
}

#[tokio::test]
async fn returns_empty_for_user_with_no_entries() {
    let store = Arc::new(InMemoryStore::new());
    let deps = list_entries::Deps {
        query: store.clone(),
    };

    let query = ListEntriesQuery {
        limit: None,
        offset: None,
        user_id: test_user("nobody").id().clone(),
        range: None,
    };

    let entries = list_entries::execute(query, &deps).await.unwrap();
    assert!(entries.is_empty());
}
