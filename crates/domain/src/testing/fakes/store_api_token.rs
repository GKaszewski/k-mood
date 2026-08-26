use crate::api_token::{ApiToken, ApiTokenId, TokenDigest};
use crate::errors::DomainError;
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::ApiTokenCommandPort for InMemoryStore {
    async fn save(&self, token: &ApiToken) -> Result<(), DomainError> {
        let mut stored = self.api_tokens.write().unwrap();

        let name_taken = stored.iter().any(|held: &ApiToken| {
            held.user_id() == token.user_id()
                && held.name() == token.name()
                && held.id() != token.id()
        });

        if name_taken {
            return Err(DomainError::Conflict(format!(
                "a token named {} already exists",
                token.name().value()
            )));
        }

        stored.retain(|held| held.id() != token.id());
        stored.push(token.clone());

        Ok(())
    }

    async fn revoke(&self, user_id: &UserId, id: &ApiTokenId) -> Result<(), DomainError> {
        let mut stored = self.api_tokens.write().unwrap();
        let held = stored.len();

        stored.retain(|token| token.user_id() != user_id || token.id() != id);

        if stored.len() == held {
            return Err(DomainError::NotFound("token not found".into()));
        }

        Ok(())
    }

    async fn mark_used(&self, id: &ApiTokenId) -> Result<(), DomainError> {
        let mut stored = self.api_tokens.write().unwrap();

        if let Some(token) = stored.iter_mut().find(|token| token.id() == id) {
            token.mark_used();
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::ApiTokenQueryPort for InMemoryStore {
    async fn find_by_digest(&self, digest: &TokenDigest) -> Result<Option<ApiToken>, DomainError> {
        Ok(self
            .api_tokens
            .read()
            .unwrap()
            .iter()
            .find(|token| token.digest() == digest)
            .cloned())
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ApiToken>, DomainError> {
        Ok(self
            .api_tokens
            .read()
            .unwrap()
            .iter()
            .filter(|token| token.user_id() == user_id)
            .cloned()
            .collect())
    }
}
