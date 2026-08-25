use chrono::{Duration, Utc};

use crate::auth::{GeneratedToken, RefreshSession};
use crate::errors::DomainError;
use crate::user::UserId;

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::AuthServicePort for InMemoryStore {
    async fn generate_token(&self, user_id: &UserId) -> Result<GeneratedToken, DomainError> {
        let token = format!("fake-jwt-{}", user_id.value());
        let expires_at = Utc::now() + Duration::hours(1);
        Ok(GeneratedToken::new(token, expires_at))
    }

    async fn validate_token(&self, token: &str) -> Result<UserId, DomainError> {
        let uuid_str = token
            .strip_prefix("fake-jwt-")
            .ok_or_else(|| DomainError::Unauthorized("invalid token".into()))?;

        let uuid: uuid::Uuid = uuid_str
            .parse()
            .map_err(|_| DomainError::Unauthorized("invalid token".into()))?;

        Ok(UserId::from_uuid(uuid))
    }
}

#[async_trait::async_trait]
impl crate::ports::RefreshSessionCommandPort for InMemoryStore {
    async fn create(&self, session: &RefreshSession) -> Result<(), DomainError> {
        self.refresh_sessions.write().unwrap().push(session.clone());
        Ok(())
    }

    async fn revoke(&self, token: &str) -> Result<(), DomainError> {
        self.refresh_sessions
            .write()
            .unwrap()
            .retain(|s| s.token() != token);
        Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: &UserId) -> Result<(), DomainError> {
        self.refresh_sessions
            .write()
            .unwrap()
            .retain(|s| s.user_id() != user_id);
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, DomainError> {
        let mut sessions = self.refresh_sessions.write().unwrap();
        let before = sessions.len();
        sessions.retain(|s| !s.is_expired());
        Ok((before - sessions.len()) as u64)
    }
}

#[async_trait::async_trait]
impl crate::ports::RefreshSessionQueryPort for InMemoryStore {
    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshSession>, DomainError> {
        Ok(self
            .refresh_sessions
            .read()
            .unwrap()
            .iter()
            .find(|s| s.token() == token)
            .cloned())
    }
}
