use domain::user::UserId;

#[derive(Debug)]
pub struct SubscribePushCommand {
    pub user_id: UserId,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

#[derive(Debug)]
pub struct UnsubscribePushCommand {
    pub user_id: UserId,
    pub endpoint: String,
}
