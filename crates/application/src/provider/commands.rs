use domain::provider::ProviderName;
use domain::user::UserId;

pub struct ConnectProviderCommand {
    pub user_id: UserId,
    pub provider: ProviderName,
    pub credential: Vec<u8>,
}

impl std::fmt::Debug for ConnectProviderCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectProviderCommand")
            .field("user_id", &self.user_id)
            .field("provider", &self.provider)
            .field("credential", &"<redacted>")
            .finish()
    }
}
