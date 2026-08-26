use std::sync::Arc;

use chrono::{FixedOffset, TimeZone};

use domain::dimension::DimensionKind;
use domain::entry::Mood;
use domain::ports::EntryDimensionPort;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{
    InMemoryDimensionStore, InMemoryStore, test_entry, test_logged_at, test_user,
};
use domain::user::UserId;

use application::entry::commands::UpdateEntryCommand;
use application::entry::use_cases::update_entry;

async fn setup() -> (
    Arc<InMemoryStore>,
    update_entry::Deps,
    domain::entry::MoodEntry,
    UserId,
) {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Meh);
    store.save(&entry).await.unwrap();

    let deps = update_entry::Deps {
        command: store.clone(),
        dimensions: vec![
            Arc::new(InMemoryDimensionStore::new(DimensionKind::Content))
                as Arc<dyn EntryDimensionPort>,
        ],
        query: store.clone(),
        media_storage: store.clone(),
        events: store.clone(),
    };

    (store, deps, entry, user.id().clone())
}

#[tokio::test]
async fn updates_mood() {
    let (store, deps, entry, user_id) = setup().await;

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Mood::Rad,
        logged_at: Some(test_logged_at()),
        dimensions: vec![],
    };

    let updated = update_entry::execute(cmd, user_id, &deps).await.unwrap();

    assert_eq!(updated.mood(), Mood::Rad);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn updating_nonexistent_entry_fails() {
    let store = Arc::new(InMemoryStore::new());
    let deps = update_entry::Deps {
        command: store.clone(),
        dimensions: vec![
            Arc::new(InMemoryDimensionStore::new(DimensionKind::Content))
                as Arc<dyn EntryDimensionPort>,
        ],
        query: store.clone(),
        media_storage: store.clone(),
        events: store.clone(),
    };

    let cmd = UpdateEntryCommand {
        entry_id: domain::entry::MoodEntryId::generate(),
        mood: Mood::Good,
        logged_at: Some(test_logged_at()),
        dimensions: vec![],
    };

    let result = update_entry::execute(cmd, UserId::generate(), &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_update_by_different_user() {
    let (_store, deps, entry, _user_id) = setup().await;

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Mood::Rad,
        logged_at: Some(test_logged_at()),
        dimensions: vec![],
    };

    let other_user = UserId::generate();
    let result = update_entry::execute(cmd, other_user, &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn an_update_carrying_no_date_leaves_the_entry_where_it_was() {
    let (_store, deps, entry, user_id) = setup().await;

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Mood::Rad,
        logged_at: None,
        dimensions: vec![],
    };

    let updated = update_entry::execute(cmd, user_id, &deps).await.unwrap();

    assert_eq!(updated.logged_at(), &test_logged_at());
}

#[tokio::test]
async fn an_update_carrying_a_date_moves_the_entry_to_it() {
    let (_store, deps, entry, user_id) = setup().await;

    let moved_to = FixedOffset::east_opt(3600)
        .unwrap()
        .with_ymd_and_hms(2024, 1, 2, 9, 30, 0)
        .unwrap();

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Mood::Rad,
        logged_at: Some(moved_to),
        dimensions: vec![],
    };

    let updated = update_entry::execute(cmd, user_id, &deps).await.unwrap();

    assert_eq!(updated.logged_at(), &moved_to);
}
