use application::push::commands::{SubscribePushCommand, UnsubscribePushCommand};
use domain::user::UserId;

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct PushSubscribeRequest {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

impl PushSubscribeRequest {
    pub fn into_command(self, user_id: UserId) -> SubscribePushCommand {
        SubscribePushCommand {
            user_id,
            endpoint: self.endpoint,
            p256dh: self.p256dh,
            auth: self.auth,
        }
    }
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct PushUnsubscribeRequest {
    pub endpoint: String,
}

impl PushUnsubscribeRequest {
    pub fn into_command(self) -> UnsubscribePushCommand {
        UnsubscribePushCommand {
            endpoint: self.endpoint,
        }
    }
}
