use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::{
    ActivityCommandPort, MoodEntryCommandPort, ReminderCommandPort, UserCommandPort,
};
use domain::testing::{InMemoryStore, test_activity, test_entry, test_reminder, test_user};
use domain::user::UserId;

use application::user::use_cases::delete_user;

fn deps(store: &Arc<InMemoryStore>) -> delete_user::Deps {
    delete_user::Deps {
        user_query: store.clone(),
        entry_query: store.clone(),
        cascade: store.clone(),
        media_storage: store.clone(),
        events: store.clone(),
    }
}

#[tokio::test]
async fn deletes_user_and_all_data() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Good);
    let activity = test_activity(user.id().clone(), "gaming");
    let reminder = test_reminder(user.id().clone());

    UserCommandPort::save(&*store, &user).await.unwrap();
    MoodEntryCommandPort::save(&*store, &entry).await.unwrap();
    ActivityCommandPort::save(&*store, &activity).await.unwrap();
    ReminderCommandPort::save(&*store, &reminder).await.unwrap();

    delete_user::execute(user.id().clone(), &deps(&store))
        .await
        .unwrap();

    assert_eq!(store.entry_count(), 0);
    assert_eq!(store.activity_count(), 0);
    assert_eq!(store.reminder_count(), 0);
    assert_eq!(store.user_count(), 0);
}

#[tokio::test]
async fn rejects_nonexistent_user() {
    let store = Arc::new(InMemoryStore::new());
    let result = delete_user::execute(UserId::generate(), &deps(&store)).await;
    assert!(result.is_err());
}
