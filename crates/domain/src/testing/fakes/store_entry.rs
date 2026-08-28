use crate::activity::ActivityId;
use crate::entry::{DateRange, EntrySelection, MoodEntry, MoodEntryId, Pagination};
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

    async fn find_all_by_user(&self, user_id: &UserId) -> Result<Vec<MoodEntry>, DomainError> {
        Ok(self.newest_first(|entry| entry.user_id() == user_id))
    }

    async fn select(
        &self,
        selection: &EntrySelection,
        page: Pagination,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let matching = self.newest_first(|entry| self.matches(entry, selection));

        Ok(matching
            .into_iter()
            .skip(page.offset() as usize)
            .take(page.limit() as usize)
            .collect())
    }

    async fn count(&self, selection: &EntrySelection) -> Result<u64, DomainError> {
        Ok(self
            .newest_first(|entry| self.matches(entry, selection))
            .len() as u64)
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

    async fn count_tagged_with(&self, activity_id: &ActivityId) -> Result<u64, DomainError> {
        let tagged = self
            .entry_activities
            .read()
            .unwrap()
            .values()
            .filter(|ids| ids.contains(activity_id))
            .count();

        Ok(tagged as u64)
    }
}

impl InMemoryStore {
    fn newest_first(&self, wanted: impl Fn(&MoodEntry) -> bool) -> Vec<MoodEntry> {
        let mut found: Vec<MoodEntry> = self
            .entries
            .read()
            .unwrap()
            .values()
            .filter(|entry| wanted(entry))
            .cloned()
            .collect();

        found.sort_by(|left, right| right.logged_at().cmp(left.logged_at()));

        found
    }

    fn matches(&self, entry: &MoodEntry, selection: &EntrySelection) -> bool {
        if entry.user_id() != selection.user_id() {
            return false;
        }

        if let Some(range) = selection.range()
            && (entry.logged_at() < range.start() || entry.logged_at() > range.end())
        {
            return false;
        }

        if let Some(mood) = selection.mood()
            && entry.mood() != mood
        {
            return false;
        }

        if let Some(activity_id) = selection.activity()
            && !self
                .entry_activities
                .read()
                .unwrap()
                .get(entry.id())
                .is_some_and(|ids| ids.contains(activity_id))
        {
            return false;
        }

        if let Some(since) = selection.since()
            && *entry.updated_at() <= since
        {
            return false;
        }

        true
    }
}
