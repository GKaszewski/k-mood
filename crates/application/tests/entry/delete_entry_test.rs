use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{InMemoryStore, test_entry, test_user};
use domain::user::UserId;

use application::entry::use_cases::delete_entry;

#[tokio::test]
async fn deletes_existing_entry() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Good);
    store.save(&entry).await.unwrap();

    let deps = delete_entry::Deps {
        dimensions: vec![],
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
        media_storage: store.clone(),
    };

    delete_entry::execute(entry.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();

    assert_eq!(store.entry_count(), 0);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn deleting_nonexistent_entry_fails() {
    let store = Arc::new(InMemoryStore::new());
    let deps = delete_entry::Deps {
        dimensions: vec![],
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
        media_storage: store.clone(),
    };

    let result = delete_entry::execute(
        domain::entry::MoodEntryId::generate(),
        UserId::generate(),
        &deps,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_delete_by_different_user() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Good);
    store.save(&entry).await.unwrap();

    let deps = delete_entry::Deps {
        dimensions: vec![],
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
        media_storage: store.clone(),
    };

    let other_user = UserId::generate();
    let result = delete_entry::execute(entry.id().clone(), other_user, &deps).await;
    assert!(result.is_err());
    assert_eq!(store.entry_count(), 1);
}
