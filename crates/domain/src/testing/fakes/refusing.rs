use crate::api_token::{ApiToken, ApiTokenId, TokenDigest};
use crate::errors::DomainError;
use crate::rejection::RejectedMetric;
use crate::user::UserId;

pub struct RefusingRejectionTrace;

#[async_trait::async_trait]
impl crate::ports::RejectionCommandPort for RefusingRejectionTrace {
    async fn record(&self, _rejections: &[RejectedMetric]) -> Result<(), DomainError> {
        Err(DomainError::InvalidInput(
            "the rejection trace is unwritable".into(),
        ))
    }
}

pub struct RefusingApiTokenStore;

#[async_trait::async_trait]
impl crate::ports::ApiTokenCommandPort for RefusingApiTokenStore {
    async fn save(&self, _token: &ApiToken) -> Result<(), DomainError> {
        Err(DomainError::InvalidInput("cannot write a token".into()))
    }

    async fn revoke(&self, _user_id: &UserId, _id: &ApiTokenId) -> Result<(), DomainError> {
        Err(DomainError::InvalidInput("cannot revoke a token".into()))
    }

    async fn mark_used(&self, _id: &ApiTokenId) -> Result<(), DomainError> {
        Err(DomainError::InvalidInput(
            "cannot record that a token was used".into(),
        ))
    }
}

pub struct OneTokenStore(pub ApiToken);

#[async_trait::async_trait]
impl crate::ports::ApiTokenQueryPort for OneTokenStore {
    async fn find_by_digest(&self, digest: &TokenDigest) -> Result<Option<ApiToken>, DomainError> {
        if self.0.digest() == digest {
            return Ok(Some(self.0.clone()));
        }

        Ok(None)
    }

    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<ApiToken>, DomainError> {
        if self.0.user_id() == user_id {
            return Ok(vec![self.0.clone()]);
        }

        Ok(Vec::new())
    }
}
