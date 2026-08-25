use std::sync::Arc;

use domain::entry::Mood;
use domain::testing::{test_entry, test_logged_at, test_user};

use application::entry::commands::CreateEntryCommand;
use application::entry::use_cases::create_entry;

fn deps() -> (Arc<domain::testing::InMemoryStore>, create_entry::Deps) {
    let store = Arc::new(domain::testing::InMemoryStore::new());
    let deps = create_entry::Deps {
        entries: store.clone(),
        events: store.clone(),
    };
    (store, deps)
}

#[tokio::test]
async fn creates_entry_with_required_fields() {
    let (store, deps) = deps();
    let user = test_user("alice");

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Good,
        logged_at: test_logged_at(),
        activities: vec![],
        content: None,
        photos: vec![],
        voice_memos: vec![],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(entry.mood(), Mood::Good);
    assert_eq!(store.entry_count(), 1);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn creates_entry_with_all_optional_fields() {
    let (store, deps) = deps();
    let user = test_user("bob");
    let activity = domain::testing::test_activity(user.id().clone(), "gaming");

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Rad,
        logged_at: test_logged_at(),
        activities: vec![activity.id().clone()],
        content: Some(domain::testing::test_content("great day")),
        photos: vec![],
        voice_memos: vec![],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(entry.mood(), Mood::Rad);
    assert_eq!(entry.activities().len(), 1);
    assert!(entry.content().is_some());
    assert_eq!(store.entry_count(), 1);
}
