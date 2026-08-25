use std::sync::Arc;

use chrono::NaiveTime;

use domain::ports::ReminderCommandPort;
use domain::reminder::DaySchedule;
use domain::testing::{InMemoryStore, test_reminder, test_user};

use application::reminder::commands::{CreateReminderCommand, UpdateReminderCommand};
use application::reminder::use_cases::{
    create_reminder, delete_reminder, get_reminder, list_reminders, update_reminder,
};

#[tokio::test]
async fn creates_reminder() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");

    let deps = create_reminder::Deps {
        reminders: store.clone(),
        events: store.clone(),
    };

    let time = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
    let cmd = CreateReminderCommand {
        user_id: user.id().clone(),
        schedule: DaySchedule::every_day_at(time),
    };

    let reminder = create_reminder::execute(cmd, &deps).await.unwrap();

    assert!(reminder.is_enabled());
    assert_eq!(store.published_events().len(), 1);
}

#[tokio::test]
async fn updates_and_disables_reminder() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let reminder = test_reminder(user.id().clone());
    store.save(&reminder).await.unwrap();

    let deps = update_reminder::Deps {
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
    };

    let cmd = UpdateReminderCommand {
        reminder_id: reminder.id().clone(),
        schedule: None,
        enabled: Some(false),
    };

    let updated = update_reminder::execute(cmd, user.id().clone(), &deps)
        .await
        .unwrap();
    assert!(!updated.is_enabled());
}

#[tokio::test]
async fn lists_reminders_for_user() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    store.save(&test_reminder(user.id().clone())).await.unwrap();
    store.save(&test_reminder(user.id().clone())).await.unwrap();

    let deps = list_reminders::Deps {
        query: store.clone(),
    };
    let reminders = list_reminders::execute(user.id().clone(), &deps)
        .await
        .unwrap();

    assert_eq!(reminders.len(), 2);
}

#[tokio::test]
async fn gets_single_reminder() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let reminder = test_reminder(user.id().clone());
    store.save(&reminder).await.unwrap();

    let deps = get_reminder::Deps {
        query: store.clone(),
    };
    let result = get_reminder::execute(reminder.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();

    assert_eq!(*result.id(), *reminder.id());
}

#[tokio::test]
async fn deletes_reminder() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");
    let reminder = test_reminder(user.id().clone());
    store.save(&reminder).await.unwrap();

    let deps = delete_reminder::Deps {
        command: store.clone(),
        query: store.clone(),
        events: store.clone(),
    };

    delete_reminder::execute(reminder.id().clone(), user.id().clone(), &deps)
        .await
        .unwrap();

    let list_deps = list_reminders::Deps {
        query: store.clone(),
    };
    let reminders = list_reminders::execute(user.id().clone(), &list_deps)
        .await
        .unwrap();
    assert!(reminders.is_empty());
}
