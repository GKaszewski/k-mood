use crate::activity::ActivityId;
use crate::entry::{DateRange, Mood, MoodEntry, MoodEntryId};
use crate::errors::DomainError;
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::MoodEntryCommandPort for InMemoryStore {
    async fn save(&self, entry: &MoodEntry) -> Result<(), DomainError> {
        self.entries
            .write()
            .unwrap()
            .insert(entry.id().clone(), entry.clone());
        Ok(())
    }

    async fn save_batch(&self, entries: &[MoodEntry]) -> Result<(), DomainError> {
        let mut store = self.entries.write().unwrap();
        for entry in entries {
            store.insert(entry.id().clone(), entry.clone());
        }
        Ok(())
    }

    async fn delete(&self, id: &MoodEntryId) -> Result<(), DomainError> {
        self.entries.write().unwrap().remove(id);
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.entries
            .write()
            .unwrap()
            .retain(|_, e| e.user_id() != user_id);
        Ok(())
    }

    async fn delete_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<u64, DomainError> {
        let mut store = self.entries.write().unwrap();
        let before = store.len();
        store.retain(|_, e| {
            e.user_id() != user_id || e.logged_at() < range.start() || e.logged_at() > range.end()
        });
        Ok((before - store.len()) as u64)
    }

    async fn replace_activity(
        &self,
        user_id: &UserId,
        old_activity_id: &ActivityId,
        new_activity_id: &ActivityId,
    ) -> Result<u64, DomainError> {
        let entries = self.entries.read().unwrap();
        let mut assignments = self.entry_activities.write().unwrap();
        let mut count = 0u64;

        for (entry_id, activities) in assignments.iter_mut() {
            let owned_by_user = entries
                .get(entry_id)
                .is_some_and(|entry| entry.user_id() == user_id);
            if !owned_by_user || !activities.contains(old_activity_id) {
                continue;
            }
            for activity in activities.iter_mut() {
                if activity == old_activity_id {
                    *activity = new_activity_id.clone();
                }
            }
            count += 1;
        }

        Ok(count)
    }
}

#[async_trait::async_trait]
impl crate::ports::MoodEntryQueryPort for InMemoryStore {
    async fn find_by_id(&self, id: &MoodEntryId) -> Result<Option<MoodEntry>, DomainError> {
        Ok(self.entries.read().unwrap().get(id).cloned())
    }

    async fn find_by_user(
        &self,
        user_id: &UserId,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let mut entries: Vec<_> = self
            .entries
            .read()
            .unwrap()
            .values()
            .filter(|e| e.user_id() == user_id)
            .cloned()
            .collect();

        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(i64::MAX) as usize;
        entries = entries.into_iter().skip(offset).take(limit).collect();

        Ok(entries)
    }

    async fn find_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        Ok(self
            .entries
            .read()
            .unwrap()
            .values()
            .filter(|e| {
                e.user_id() == user_id
                    && e.logged_at() >= range.start()
                    && e.logged_at() <= range.end()
            })
            .cloned()
            .collect())
    }

    async fn find_by_mood(
        &self,
        user_id: &UserId,
        mood: Mood,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        Ok(self
            .entries
            .read()
            .unwrap()
            .values()
            .filter(|e| e.user_id() == user_id && e.mood() == mood)
            .cloned()
            .collect())
    }

    async fn find_by_activity(
        &self,
        user_id: &UserId,
        activity_id: &ActivityId,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        Ok(self
            .entries
            .read()
            .unwrap()
            .values()
            .filter(|e| {
                e.user_id() == user_id
                    && self
                        .entry_activities
                        .read()
                        .unwrap()
                        .get(e.id())
                        .is_some_and(|ids| ids.contains(activity_id))
            })
            .cloned()
            .collect())
    }
}
