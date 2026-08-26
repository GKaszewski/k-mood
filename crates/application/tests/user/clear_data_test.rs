use std::sync::Arc;

use domain::entry::Mood;
use domain::ports::{ActivityCommandPort, MoodEntryCommandPort, ReminderCommandPort};
use domain::testing::{InMemoryStore, test_activity, test_entry, test_reminder, test_user};

use application::user::use_cases::clear_data;

#[tokio::test]
async fn clears_all_user_data() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let entry = test_entry(user.id().clone(), Mood::Good);
    let activity = test_activity(user.id().clone(), "gaming");
    let reminder = test_reminder(user.id().clone());

    MoodEntryCommandPort::save(&*store, &entry).await.unwrap();
    ActivityCommandPort::save(&*store, &activity).await.unwrap();
    ReminderCommandPort::save(&*store, &reminder).await.unwrap();

    let deps = clear_data::Deps {
        dimensions: vec![],
        entry_query: store.clone(),
        cascade: store.clone(),
        media_storage: store.clone(),
    };

    clear_data::execute(user.id().clone(), &deps).await.unwrap();

    assert_eq!(store.entry_count(), 0);
    assert_eq!(store.activity_count(), 0);
    assert_eq!(store.reminder_count(), 0);
}
