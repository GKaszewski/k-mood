use crate::errors::DomainError;
use crate::provider::{ProviderConnection, ProviderName};
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::ProviderConnectionCommandPort for InMemoryStore {
    async fn save(&self, connection: &ProviderConnection) -> Result<(), DomainError> {
        let mut stored = self.provider_connections.write().unwrap();
        stored.retain(|existing| {
            existing.user_id() != connection.user_id()
                || existing.provider() != connection.provider()
        });
        stored.push(connection.clone());
        Ok(())
    }

    async fn delete(&self, user_id: &UserId, provider: &ProviderName) -> Result<(), DomainError> {
        self.provider_connections
            .write()
            .unwrap()
            .retain(|existing| existing.user_id() != user_id || existing.provider() != provider);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::ProviderConnectionQueryPort for InMemoryStore {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ProviderConnection>, DomainError> {
        Ok(self
            .provider_connections
            .read()
            .unwrap()
            .iter()
            .filter(|existing| existing.user_id() == user_id)
            .cloned()
            .collect())
    }

    async fn find_by_user_and_provider(
        &self,
        user_id: &UserId,
        provider: &ProviderName,
    ) -> Result<Option<ProviderConnection>, DomainError> {
        Ok(self
            .provider_connections
            .read()
            .unwrap()
            .iter()
            .find(|existing| existing.user_id() == user_id && existing.provider() == provider)
            .cloned())
    }
}
