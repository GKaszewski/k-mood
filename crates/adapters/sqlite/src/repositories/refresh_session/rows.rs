use domain::auth::{RefreshSession, RefreshSessionId};
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct RefreshSessionRow {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: String,
    pub created_at: String,
}

impl RefreshSessionRow {
    pub fn into_domain(self) -> RefreshSession {
        RefreshSession::from_persistence(
            RefreshSessionId::from_uuid(self.id.parse().unwrap()),
            UserId::from_uuid(self.user_id.parse().unwrap()),
            self.token,
            self.expires_at.parse().unwrap(),
            self.created_at.parse().unwrap(),
        )
    }
}
