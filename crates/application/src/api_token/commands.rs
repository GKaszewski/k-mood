use domain::provider::ProviderName;
use domain::user::UserId;

#[derive(Debug)]
pub struct MintApiTokenCommand {
    pub user_id: UserId,
    pub name: ProviderName,
}
