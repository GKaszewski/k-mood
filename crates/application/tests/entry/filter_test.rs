use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::MoodEntryCommandPort;
use domain::testing::{InMemoryStore, test_activity, test_entry, test_user};

use application::entry::queries::{FilterByActivityQuery, FilterByMoodQuery};
use application::entry::use_cases::{filter_by_activity, filter_by_mood};

#[tokio::test]
async fn filters_by_mood() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");

    store
        .save(&test_entry(user.id().clone(), Mood::Good))
        .await
        .unwrap();
    store
        .save(&test_entry(user.id().clone(), Mood::Good))
        .await
        .unwrap();
    store
        .save(&test_entry(user.id().clone(), Mood::Bad))
        .await
        .unwrap();

    let deps = filter_by_mood::Deps {
        query: store.clone(),
    };

    let query = FilterByMoodQuery {
        user_id: user.id().clone(),
        mood: Mood::Good,
    };

    let entries = filter_by_mood::execute(query, &deps).await.unwrap();
    assert_eq!(entries.len(), 2);
}

#[tokio::test]
async fn filters_by_activity() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let activity = test_activity(user.id().clone(), "exercise");

    let mut tagged = test_entry(user.id().clone(), Mood::Good);
    tagged.set_activities(vec![activity.id().clone()]);
    store.save(&tagged).await.unwrap();

    store
        .save(&test_entry(user.id().clone(), Mood::Meh))
        .await
        .unwrap();

    let deps = filter_by_activity::Deps {
        query: store.clone(),
    };

    let query = FilterByActivityQuery {
        user_id: user.id().clone(),
        activity_id: activity.id().clone(),
    };

    let entries = filter_by_activity::execute(query, &deps).await.unwrap();
    assert_eq!(entries.len(), 1);
}
