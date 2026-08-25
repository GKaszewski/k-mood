use crate::attachment::{MediaUpload, PhotoId, VoiceMemoId};
use crate::entry::{DateRange, MoodEntry};
use crate::errors::DomainError;
use crate::events::EventEnvelope;
use crate::ports::ImportedRow;
use crate::push::{PushSubscription, PushSubscriptionId};
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::EventPublisherPort for InMemoryStore {
    async fn publish(&self, envelope: EventEnvelope) -> Result<(), DomainError> {
        self.events.write().unwrap().push(envelope);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::MediaStoragePort for InMemoryStore {
    async fn store_photo(&self, _upload: MediaUpload) -> Result<PhotoId, DomainError> {
        Ok(PhotoId::generate())
    }

    async fn store_voice_memo(&self, _upload: MediaUpload) -> Result<VoiceMemoId, DomainError> {
        Ok(VoiceMemoId::generate())
    }

    async fn get_photo(
        &self,
        _id: &PhotoId,
    ) -> Result<Option<crate::ports::MediaFile>, DomainError> {
        Ok(None)
    }

    async fn get_voice_memo(
        &self,
        _id: &VoiceMemoId,
    ) -> Result<Option<crate::ports::MediaFile>, DomainError> {
        Ok(None)
    }

    async fn delete_photo(&self, _id: &PhotoId) -> Result<(), DomainError> {
        Ok(())
    }

    async fn delete_voice_memo(&self, _id: &VoiceMemoId) -> Result<(), DomainError> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::CascadeDeletePort for InMemoryStore {
    async fn delete_all_user_data(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.entries
            .write()
            .unwrap()
            .retain(|_, e| e.user_id() != user_id);
        self.activities
            .write()
            .unwrap()
            .retain(|_, a| a.user_id() != user_id);
        self.reminders
            .write()
            .unwrap()
            .retain(|_, r| r.user_id() != user_id);
        Ok(())
    }

    async fn delete_user_account(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.entries
            .write()
            .unwrap()
            .retain(|_, e| e.user_id() != user_id);
        self.activities
            .write()
            .unwrap()
            .retain(|_, a| a.user_id() != user_id);
        self.reminders
            .write()
            .unwrap()
            .retain(|_, r| r.user_id() != user_id);
        self.refresh_sessions
            .write()
            .unwrap()
            .retain(|s| s.user_id() != user_id);
        self.push_subscriptions
            .write()
            .unwrap()
            .retain(|_, s| s.user_id() != user_id);
        self.users.write().unwrap().remove(user_id);
        Ok(())
    }

    async fn delete_entries_in_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let mut entries = self.entries.write().unwrap();
        let mut removed = Vec::new();
        entries.retain(|_, e| {
            if e.user_id() == user_id
                && e.logged_at() >= range.start()
                && e.logged_at() <= range.end()
            {
                removed.push(e.clone());
                false
            } else {
                true
            }
        });
        Ok(removed)
    }
}

#[async_trait::async_trait]
impl crate::ports::PushSubscriptionCommandPort for InMemoryStore {
    async fn save(&self, subscription: &PushSubscription) -> Result<(), DomainError> {
        self.push_subscriptions
            .write()
            .unwrap()
            .insert(subscription.id().clone(), subscription.clone());
        Ok(())
    }

    async fn delete(&self, id: &PushSubscriptionId) -> Result<(), DomainError> {
        self.push_subscriptions.write().unwrap().remove(id);
        Ok(())
    }

    async fn delete_by_endpoint(&self, endpoint: &str) -> Result<(), DomainError> {
        self.push_subscriptions
            .write()
            .unwrap()
            .retain(|_, s| s.endpoint() != endpoint);
        Ok(())
    }

    async fn delete_all_by_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.push_subscriptions
            .write()
            .unwrap()
            .retain(|_, s| s.user_id() != user_id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::PushSubscriptionQueryPort for InMemoryStore {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<PushSubscription>, DomainError> {
        Ok(self
            .push_subscriptions
            .read()
            .unwrap()
            .values()
            .filter(|s| s.user_id() == user_id)
            .cloned()
            .collect())
    }

    async fn find_by_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Option<PushSubscription>, DomainError> {
        Ok(self
            .push_subscriptions
            .read()
            .unwrap()
            .values()
            .find(|s| s.endpoint() == endpoint)
            .cloned())
    }
}

#[async_trait::async_trait]
impl crate::ports::ExportPort for InMemoryStore {
    async fn export_user_data(
        &self,
        _data: &crate::ports::UserExport,
    ) -> Result<Vec<u8>, DomainError> {
        Ok(b"exported".to_vec())
    }
}

#[async_trait::async_trait]
impl crate::ports::ImportSourcePort for InMemoryStore {
    async fn read_entries(&self, _data: &[u8]) -> Result<Vec<ImportedRow>, DomainError> {
        Ok(Vec::new())
    }
}
