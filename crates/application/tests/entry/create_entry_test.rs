use std::sync::Arc;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::Mood;
use domain::ports::EntryDimensionPort;
use domain::testing::{InMemoryDimensionStore, InMemoryStore, test_logged_at, test_user};

use application::entry::commands::CreateEntryCommand;
use application::entry::use_cases::create_entry;

fn deps() -> (Arc<InMemoryStore>, create_entry::Deps) {
    let store = Arc::new(InMemoryStore::new());
    let content: Arc<dyn EntryDimensionPort> =
        Arc::new(InMemoryDimensionStore::new(DimensionKind::Content));
    let activities: Arc<dyn EntryDimensionPort> =
        Arc::new(InMemoryDimensionStore::new(DimensionKind::Activities));

    let deps = create_entry::Deps {
        entries: store.clone(),
        dimensions: vec![content, activities],
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
        dimensions: vec![],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(entry.mood(), Mood::Good);
    assert_eq!(store.entry_count(), 1);
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn creates_entry_with_optional_dimensions() {
    let (store, deps) = deps();
    let user = test_user("bob");
    let activity = domain::testing::test_activity(user.id().clone(), "gaming");

    let cmd = CreateEntryCommand {
        user_id: user.id().clone(),
        mood: Mood::Rad,
        logged_at: test_logged_at(),
        dimensions: vec![
            DimensionValue::Content(domain::testing::test_content("great day")),
            DimensionValue::activities(vec![activity.id().clone()]),
        ],
    };

    let entry = create_entry::execute(cmd, &deps).await.unwrap();

    assert_eq!(entry.mood(), Mood::Rad);
    assert_eq!(store.entry_count(), 1);
}
