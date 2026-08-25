use crate::errors::DomainError;
use crate::reminder::{Reminder, ReminderId};
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::ReminderCommandPort for InMemoryStore {
    async fn save(&self, reminder: &Reminder) -> Result<(), DomainError> {
        self.reminders
            .write()
            .unwrap()
            .insert(reminder.id().clone(), reminder.clone());
        Ok(())
    }

    async fn delete(&self, id: &ReminderId) -> Result<(), DomainError> {
        self.reminders.write().unwrap().remove(id);
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.reminders
            .write()
            .unwrap()
            .retain(|_, r| r.user_id() != user_id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::ReminderQueryPort for InMemoryStore {
    async fn find_by_id(&self, id: &ReminderId) -> Result<Option<Reminder>, DomainError> {
        Ok(self.reminders.read().unwrap().get(id).cloned())
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Reminder>, DomainError> {
        Ok(self
            .reminders
            .read()
            .unwrap()
            .values()
            .filter(|r| r.user_id() == user_id)
            .cloned()
            .collect())
    }

    async fn find_all_enabled(&self) -> Result<Vec<Reminder>, DomainError> {
        Ok(self
            .reminders
            .read()
            .unwrap()
            .values()
            .filter(|r| r.is_enabled())
            .cloned()
            .collect())
    }
}

#[async_trait::async_trait]
impl crate::ports::ReminderSenderPort for InMemoryStore {
    async fn send_reminder(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.sent_reminders.write().unwrap().push(user_id.clone());
        Ok(())
    }
}
