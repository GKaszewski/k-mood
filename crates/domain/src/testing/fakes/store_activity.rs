use crate::activity::{Activity, ActivityId};
use crate::errors::DomainError;
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::ActivityCommandPort for InMemoryStore {
    async fn save(&self, activity: &Activity) -> Result<(), DomainError> {
        self.activities
            .write()
            .unwrap()
            .insert(activity.id().clone(), activity.clone());
        Ok(())
    }

    async fn delete(&self, id: &ActivityId) -> Result<(), DomainError> {
        self.activities.write().unwrap().remove(id);
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.activities
            .write()
            .unwrap()
            .retain(|_, a| a.user_id() != user_id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::ActivityQueryPort for InMemoryStore {
    async fn find_by_id(&self, id: &ActivityId) -> Result<Option<Activity>, DomainError> {
        Ok(self.activities.read().unwrap().get(id).cloned())
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Activity>, DomainError> {
        Ok(self
            .activities
            .read()
            .unwrap()
            .values()
            .filter(|a| a.user_id() == user_id)
            .cloned()
            .collect())
    }

    async fn find_active_by_user(&self, user_id: &UserId) -> Result<Vec<Activity>, DomainError> {
        Ok(self
            .activities
            .read()
            .unwrap()
            .values()
            .filter(|a| a.user_id() == user_id && !a.is_archived())
            .cloned()
            .collect())
    }
}
