use chrono::{DateTime, Utc};

use crate::macros::uuid_id;
use crate::user::UserId;

uuid_id!(PushSubscriptionId);

#[derive(Debug, Clone)]
pub struct PushSubscription {
    id: PushSubscriptionId,
    user_id: UserId,
    endpoint: String,
    p256dh: String,
    auth: String,
    created_at: DateTime<Utc>,
}

impl PushSubscription {
    pub fn new(user_id: UserId, endpoint: String, p256dh: String, auth: String) -> Self {
        Self {
            id: PushSubscriptionId::generate(),
            user_id,
            endpoint,
            p256dh,
            auth,
            created_at: Utc::now(),
        }
    }

    pub fn from_persistence(
        id: PushSubscriptionId,
        user_id: UserId,
        endpoint: String,
        p256dh: String,
        auth: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            endpoint,
            p256dh,
            auth,
            created_at,
        }
    }

    pub fn renew(&mut self, user_id: UserId, p256dh: String, auth: String) {
        self.user_id = user_id;
        self.p256dh = p256dh;
        self.auth = auth;
    }

    pub fn id(&self) -> &PushSubscriptionId {
        &self.id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn p256dh(&self) -> &str {
        &self.p256dh
    }

    pub fn auth(&self) -> &str {
        &self.auth
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
