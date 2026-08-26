use std::sync::Arc;

use domain::activity::ActivityId;
use domain::entry::Mood;
use domain::ports::{ActivityCommandPort, MoodEntryCommandPort, MoodEntryQueryPort};
use domain::testing::{InMemoryStore, test_activity, test_entry, test_user};

use application::entry::use_cases::replace_activity;

#[tokio::test]
async fn replaces_activity_on_entries() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let old = test_activity(user.id().clone(), "gaming");
    let new = test_activity(user.id().clone(), "video games");
    ActivityCommandPort::save(&*store, &old).await.unwrap();
    ActivityCommandPort::save(&*store, &new).await.unwrap();

    let entry = test_entry(user.id().clone(), Mood::Good);
    MoodEntryCommandPort::save(&*store, &entry).await.unwrap();
    store.assign_activities(entry.id(), vec![old.id().clone()]);

    let deps = replace_activity::Deps {
        entry_command: store.clone(),
        activity_query: store.clone(),
    };

    replace_activity::execute(user.id().clone(), old.id().clone(), new.id().clone(), &deps)
        .await
        .unwrap();

    let now_tagged_new = store.find_by_activity(user.id(), new.id()).await.unwrap();
    let still_tagged_old = store.find_by_activity(user.id(), old.id()).await.unwrap();

    assert_eq!(now_tagged_new.len(), 1);
    assert!(still_tagged_old.is_empty());
}

#[tokio::test]
async fn rejects_nonexistent_target_activity() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");

    let deps = replace_activity::Deps {
        entry_command: store.clone(),
        activity_query: store.clone(),
    };

    let result = replace_activity::execute(
        user.id().clone(),
        ActivityId::generate(),
        ActivityId::generate(),
        &deps,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn rejects_target_owned_by_different_user() {
    let store = Arc::new(InMemoryStore::new());
    let alice = test_user("alice");
    let bob = test_user("bob");
    let bobs_activity = test_activity(bob.id().clone(), "gaming");
    ActivityCommandPort::save(&*store, &bobs_activity)
        .await
        .unwrap();

    let deps = replace_activity::Deps {
        entry_command: store.clone(),
        activity_query: store.clone(),
    };

    let result = replace_activity::execute(
        alice.id().clone(),
        ActivityId::generate(),
        bobs_activity.id().clone(),
        &deps,
    )
    .await;
    assert!(result.is_err());
}
