use chrono::{DateTime, Utc};

use crate::provider::ProviderName;
use crate::user::UserId;

use super::{ApiTokenId, TokenDigest, TokenScope};

pub struct ApiTokenData {
    pub id: ApiTokenId,
    pub user_id: UserId,
    pub name: ProviderName,
    pub digest: TokenDigest,
    pub scope: TokenScope,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ApiToken {
    id: ApiTokenId,
    user_id: UserId,
    name: ProviderName,
    digest: TokenDigest,
    scope: TokenScope,
    created_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
}

impl ApiToken {
    pub fn new(user_id: UserId, name: ProviderName, digest: TokenDigest) -> Self {
        Self {
            id: ApiTokenId::generate(),
            user_id,
            name,
            digest,
            scope: TokenScope::WriteMetrics,
            created_at: Utc::now(),
            last_used_at: None,
        }
    }

    pub fn from_persistence(data: ApiTokenData) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            name: data.name,
            digest: data.digest,
            scope: data.scope,
            created_at: data.created_at,
            last_used_at: data.last_used_at,
        }
    }
}

impl ApiToken {
    pub fn id(&self) -> &ApiTokenId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn name(&self) -> &ProviderName {
        &self.name
    }

    pub fn digest(&self) -> &TokenDigest {
        &self.digest
    }

    pub fn scope(&self) -> TokenScope {
        self.scope
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn last_used_at(&self) -> Option<&DateTime<Utc>> {
        self.last_used_at.as_ref()
    }
}

impl ApiToken {
    pub fn mark_used(&mut self) {
        self.last_used_at = Some(Utc::now());
    }
}
