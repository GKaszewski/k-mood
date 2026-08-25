use crate::errors::DomainError;
use crate::reminder::{Reminder, ReminderId};
use crate::user::UserId;

#[async_trait::async_trait]
pub trait ReminderCommandPort: Send + Sync {
    async fn save(&self, reminder: &Reminder) -> Result<(), DomainError>;
    async fn delete(&self, id: &ReminderId) -> Result<(), DomainError>;
    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait ReminderQueryPort: Send + Sync {
    async fn find_by_id(&self, id: &ReminderId) -> Result<Option<Reminder>, DomainError>;
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Reminder>, DomainError>;
    async fn find_all_enabled(&self) -> Result<Vec<Reminder>, DomainError>;
}

#[async_trait::async_trait]
pub trait ReminderSenderPort: Send + Sync {
    async fn send_reminder(&self, user_id: &UserId) -> Result<(), DomainError>;
}
