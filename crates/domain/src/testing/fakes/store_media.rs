use crate::attachment::MediaRef;
use crate::errors::DomainError;
use crate::user::UserId;

use super::store::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::MediaOwnershipPort for InMemoryStore {
    async fn remember(&self, owner: &UserId, media: MediaRef) -> Result<(), DomainError> {
        self.media_owners
            .write()
            .unwrap()
            .insert(media, owner.clone());

        Ok(())
    }

    async fn owner_of(&self, media: MediaRef) -> Result<Option<UserId>, DomainError> {
        Ok(self.media_owners.read().unwrap().get(&media).cloned())
    }

    async fn owned_by(&self, owner: &UserId) -> Result<Vec<MediaRef>, DomainError> {
        Ok(self
            .media_owners
            .read()
            .unwrap()
            .iter()
            .filter(|(_, held)| *held == owner)
            .map(|(media, _)| *media)
            .collect())
    }

    async fn forget(&self, media: MediaRef) -> Result<(), DomainError> {
        self.media_owners.write().unwrap().remove(&media);

        Ok(())
    }

    async fn forget_all_by_user(&self, owner: &UserId) -> Result<(), DomainError> {
        self.media_owners
            .write()
            .unwrap()
            .retain(|_, held| held != owner);

        Ok(())
    }
}
