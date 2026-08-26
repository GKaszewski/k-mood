use crate::entry::Date;
use crate::errors::DomainError;
use crate::user::{UserId, UserPreferences};

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::CycleStartCommandPort for InMemoryStore {
    async fn record(&self, user_id: &UserId, date: &Date) -> Result<(), DomainError> {
        let mut stored = self.cycle_starts.write().unwrap();

        if !stored.iter().any(|held| held == &(user_id.clone(), *date)) {
            stored.push((user_id.clone(), *date));
        }

        Ok(())
    }

    async fn forget(&self, user_id: &UserId, date: &Date) -> Result<(), DomainError> {
        self.cycle_starts
            .write()
            .unwrap()
            .retain(|held| held.0 != *user_id || held.1 != *date);

        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::CycleStartQueryPort for InMemoryStore {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Date>, DomainError> {
        Ok(self
            .cycle_starts
            .read()
            .unwrap()
            .iter()
            .filter(|held| held.0 == *user_id)
            .map(|held| held.1)
            .collect())
    }
}

#[async_trait::async_trait]
impl crate::ports::UserPreferencesCommandPort for InMemoryStore {
    async fn save(&self, preferences: &UserPreferences) -> Result<(), DomainError> {
        let mut stored = self.preferences.write().unwrap();
        stored.retain(|held| held.user_id() != preferences.user_id());
        stored.push(preferences.clone());

        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::UserPreferencesQueryPort for InMemoryStore {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Option<UserPreferences>, DomainError> {
        Ok(self
            .preferences
            .read()
            .unwrap()
            .iter()
            .find(|held| held.user_id() == user_id)
            .cloned())
    }
}
