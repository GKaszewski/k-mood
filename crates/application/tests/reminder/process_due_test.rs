use std::sync::{Arc, Mutex};

use chrono::{Duration, Utc};

use domain::errors::DomainError;
use domain::ports::{ReminderCommandPort, ReminderSenderPort, UserCommandPort};
use domain::reminder::{DaySchedule, Reminder};
use domain::testing::{InMemoryStore, test_user};
use domain::user::{Timezone, UserId};

use application::reminder::use_cases::process_due_reminders;

/// Records every attempt and fails for one nominated user, the way a device
/// whose push subscription has gone stale does.
struct FlakySender {
    failing: UserId,
    attempts: Mutex<Vec<UserId>>,
}

impl FlakySender {
    fn new(failing: UserId) -> Self {
        Self {
            failing,
            attempts: Mutex::new(Vec::new()),
        }
    }

    fn attempts(&self) -> Vec<UserId> {
        self.attempts.lock().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl ReminderSenderPort for FlakySender {
    async fn send_reminder(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.attempts.lock().unwrap().push(user_id.clone());

        if user_id == &self.failing {
            return Err(DomainError::InvalidInput(
                "no push notification could be delivered".into(),
            ));
        }

        Ok(())
    }
}

/// A reminder due right now, so `should_send` lets it through.
fn due_now(user_id: UserId) -> Reminder {
    Reminder::new(user_id, DaySchedule::every_day_at(Utc::now().time()))
}

#[tokio::test]
async fn a_failing_send_does_not_abort_the_sweep() {
    let store = Arc::new(InMemoryStore::new());

    let mut broken = test_user("broken");
    broken.update_timezone(Some(Timezone::new("UTC").unwrap()));
    let mut healthy = test_user("healthy");
    healthy.update_timezone(Some(Timezone::new("UTC").unwrap()));

    UserCommandPort::save(&*store, &broken).await.unwrap();
    UserCommandPort::save(&*store, &healthy).await.unwrap();
    ReminderCommandPort::save(&*store, &due_now(broken.id().clone()))
        .await
        .unwrap();
    ReminderCommandPort::save(&*store, &due_now(healthy.id().clone()))
        .await
        .unwrap();

    let sender = Arc::new(FlakySender::new(broken.id().clone()));
    let deps = process_due_reminders::Deps {
        reminder_query: store.clone(),
        reminder_command: store.clone(),
        user_query: store.clone(),
        sender: sender.clone(),
        grace: Duration::minutes(30),
    };

    let sent = process_due_reminders::execute(&deps).await.unwrap();

    assert_eq!(sent, 1, "the healthy user should still have been counted");

    let attempts = sender.attempts();
    assert!(
        attempts.contains(broken.id()) && attempts.contains(healthy.id()),
        "both users should have been attempted regardless of order, got {attempts:?}"
    );
}

struct CountingSender {
    attempts: Mutex<Vec<UserId>>,
}

impl CountingSender {
    fn new() -> Self {
        Self {
            attempts: Mutex::new(Vec::new()),
        }
    }

    fn count(&self) -> usize {
        self.attempts.lock().unwrap().len()
    }
}

#[async_trait::async_trait]
impl ReminderSenderPort for CountingSender {
    async fn send_reminder(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.attempts.lock().unwrap().push(user_id.clone());

        Ok(())
    }
}

#[tokio::test]
async fn a_reminder_is_sent_once_however_often_the_sweep_runs() {
    let store = Arc::new(InMemoryStore::new());

    let mut user = test_user("nadia");
    user.update_timezone(Some(Timezone::new("UTC").unwrap()));
    UserCommandPort::save(&*store, &user).await.unwrap();
    ReminderCommandPort::save(&*store, &due_now(user.id().clone()))
        .await
        .unwrap();

    let sender = Arc::new(CountingSender::new());
    let deps = process_due_reminders::Deps {
        reminder_query: store.clone(),
        reminder_command: store.clone(),
        user_query: store.clone(),
        sender: sender.clone(),
        grace: Duration::minutes(30),
    };

    for _ in 0..5 {
        process_due_reminders::execute(&deps).await.unwrap();
    }

    assert_eq!(
        sender.count(),
        1,
        "five sweeps inside one grace window must still send one reminder"
    );
}

#[tokio::test]
async fn a_send_that_fails_is_retried_by_the_next_sweep() {
    let store = Arc::new(InMemoryStore::new());

    let mut user = test_user("broken");
    user.update_timezone(Some(Timezone::new("UTC").unwrap()));
    UserCommandPort::save(&*store, &user).await.unwrap();
    ReminderCommandPort::save(&*store, &due_now(user.id().clone()))
        .await
        .unwrap();

    let sender = Arc::new(FlakySender::new(user.id().clone()));
    let deps = process_due_reminders::Deps {
        reminder_query: store.clone(),
        reminder_command: store.clone(),
        user_query: store.clone(),
        sender: sender.clone(),
        grace: Duration::minutes(30),
    };

    process_due_reminders::execute(&deps).await.unwrap();
    process_due_reminders::execute(&deps).await.unwrap();

    assert_eq!(
        sender.attempts().len(),
        2,
        "nothing was delivered, so the occurrence is still owed"
    );
}
