use domain::push::{PushSubscription, PushSubscriptionId};
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct PushSubscriptionRow {
    pub id: String,
    pub user_id: String,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: String,
}

impl PushSubscriptionRow {
    pub fn into_entity(self) -> PushSubscription {
        PushSubscription::from_persistence(
            PushSubscriptionId::from_uuid(self.id.parse().unwrap()),
            UserId::from_uuid(self.user_id.parse().unwrap()),
            self.endpoint,
            self.p256dh,
            self.auth,
            self.created_at.parse().unwrap(),
        )
    }
}
