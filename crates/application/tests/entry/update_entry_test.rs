use std::sync::Arc;

use chrono::{FixedOffset, TimeZone};

use domain::dimension::DimensionKind;
use domain::entry::Mood;
use domain::ports::EntryDimensionPort;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{
    InMemoryActivityDimension, InMemoryDimensionStore, InMemoryStore, test_entry, test_logged_at,
    test_user,
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
        activities: store.clone(),
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
        mood: Some(Mood::Rad),
        logged_at: Some(test_logged_at()),
        dimensions: Some(vec![]),
    };

    let updated = update_entry::execute(cmd, user_id, &deps).await.unwrap();

    assert_eq!(updated.mood(), Mood::Rad);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn updating_nonexistent_entry_fails() {
    let store = Arc::new(InMemoryStore::new());
    let deps = update_entry::Deps {
        activities: store.clone(),
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
        mood: Some(Mood::Good),
        logged_at: Some(test_logged_at()),
        dimensions: Some(vec![]),
    };

    let result = update_entry::execute(cmd, UserId::generate(), &deps).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_update_by_different_user() {
    let (_store, deps, entry, _user_id) = setup().await;

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Some(Mood::Rad),
        logged_at: Some(test_logged_at()),
        dimensions: Some(vec![]),
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
        mood: Some(Mood::Rad),
        logged_at: None,
        dimensions: Some(vec![]),
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
        mood: Some(Mood::Rad),
        logged_at: Some(moved_to),
        dimensions: Some(vec![]),
    };

    let updated = update_entry::execute(cmd, user_id, &deps).await.unwrap();

    assert_eq!(updated.logged_at(), &moved_to);
}

async fn an_entry_already_wearing(
    user: &domain::user::User,
    activity: &domain::activity::Activity,
) -> (
    Arc<InMemoryStore>,
    update_entry::Deps,
    domain::entry::MoodEntry,
) {
    let store = Arc::new(InMemoryStore::new());
    let entry = test_entry(user.id().clone(), Mood::Meh);
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();

    domain::ports::ActivityCommandPort::save(store.as_ref(), activity)
        .await
        .unwrap();

    let tags: Arc<InMemoryActivityDimension> =
        Arc::new(InMemoryActivityDimension::sharing(store.clone()));
    tags.save(
        entry.id(),
        &[domain::dimension::DimensionValue::activities(vec![
            activity.id().clone(),
        ])],
    )
    .await
    .unwrap();

    let deps = update_entry::Deps {
        activities: store.clone(),
        command: store.clone(),
        dimensions: vec![tags as Arc<dyn EntryDimensionPort>],
        query: store.clone(),
        media_storage: store.clone(),
        events: store.clone(),
    };

    (store, deps, entry)
}

#[tokio::test]
async fn an_entry_that_already_wears_an_archived_activity_can_still_be_edited() {
    let user = test_user("alice");
    let mut retired = domain::testing::test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();

    let (_store, deps, entry) = an_entry_already_wearing(&user, &retired).await;

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Some(Mood::Rad),
        logged_at: None,
        dimensions: Some(vec![domain::dimension::DimensionValue::activities(vec![
            retired.id().clone(),
        ])]),
    };

    let updated = update_entry::execute(cmd, entry.user_id().clone(), &deps)
        .await
        .expect("archived activities remain on the entries that already wear them");

    assert_eq!(updated.mood(), Mood::Rad);
}

#[tokio::test]
async fn an_archived_activity_cannot_be_added_to_an_entry_that_lacks_it() {
    let user = test_user("alice");
    let mut retired = domain::testing::test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();

    let worn = domain::testing::test_activity(user.id().clone(), "reading");
    let (store, deps, entry) = an_entry_already_wearing(&user, &worn).await;

    domain::ports::ActivityCommandPort::save(store.as_ref(), &retired)
        .await
        .unwrap();

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Some(Mood::Rad),
        logged_at: None,
        dimensions: Some(vec![domain::dimension::DimensionValue::activities(vec![
            worn.id().clone(),
            retired.id().clone(),
        ])]),
    };

    let refused = update_entry::execute(cmd, entry.user_id().clone(), &deps).await;

    assert!(
        matches!(
            refused,
            Err(application::errors::ApplicationError::Validation(_))
        ),
        "editing an entry is not a way past the archive, got {refused:?}"
    );
}

#[tokio::test]
async fn an_archived_activity_can_be_taken_off_an_entry() {
    let user = test_user("alice");
    let mut retired = domain::testing::test_activity(user.id().clone(), "fencing");
    retired.archive().unwrap();

    let (_store, deps, entry) = an_entry_already_wearing(&user, &retired).await;

    let cmd = UpdateEntryCommand {
        entry_id: entry.id().clone(),
        mood: Some(Mood::Rad),
        logged_at: None,
        dimensions: Some(vec![]),
    };

    update_entry::execute(cmd, entry.user_id().clone(), &deps)
        .await
        .expect("dropping a retired tag is always allowed");
}
