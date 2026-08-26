use chrono::{DateTime, Utc};

use crate::user::UserId;

use super::{EncryptedCredential, ProviderConnectionId, ProviderName};

pub struct ProviderConnectionData {
    pub id: ProviderConnectionId,
    pub user_id: UserId,
    pub provider: ProviderName,
    pub credential: EncryptedCredential,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ProviderConnection {
    id: ProviderConnectionId,
    user_id: UserId,
    provider: ProviderName,
    credential: EncryptedCredential,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProviderConnection {
    pub fn new(user_id: UserId, provider: ProviderName, credential: EncryptedCredential) -> Self {
        let now = Utc::now();
        Self {
            id: ProviderConnectionId::generate(),
            user_id,
            provider,
            credential,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_persistence(data: ProviderConnectionData) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            provider: data.provider,
            credential: data.credential,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }
}

impl ProviderConnection {
    pub fn id(&self) -> &ProviderConnectionId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn provider(&self) -> &ProviderName {
        &self.provider
    }

    pub fn credential(&self) -> &EncryptedCredential {
        &self.credential
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl ProviderConnection {
    pub fn replace_credential(&mut self, credential: EncryptedCredential) {
        self.credential = credential;
        self.updated_at = Utc::now();
    }
}
