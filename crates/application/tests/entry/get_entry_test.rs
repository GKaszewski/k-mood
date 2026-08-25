use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{InMemoryStore, test_entry, test_user};
use domain::user::UserId;

use application::entry::use_cases::get_entry;

#[tokio::test]
async fn returns_existing_entry() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Good);
    store.save(&entry).await.unwrap();

    let deps = get_entry::Deps {
        query: store.clone(),
    };

    let result = get_entry::execute(entry.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();
    assert_eq!(result.mood(), Mood::Good);
}

#[tokio::test]
async fn returns_error_for_nonexistent_entry() {
    let store = Arc::new(InMemoryStore::new());
    let deps = get_entry::Deps {
        query: store.clone(),
    };

    let result = get_entry::execute(
        domain::entry::MoodEntryId::generate(),
        UserId::generate(),
        &deps,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_access_by_different_user() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Good);
    store.save(&entry).await.unwrap();

    let deps = get_entry::Deps {
        query: store.clone(),
    };

    let other_user = UserId::generate();
    let result = get_entry::execute(entry.id().clone(), other_user, &deps).await;
    assert!(result.is_err());
}
