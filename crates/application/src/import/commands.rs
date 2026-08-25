use domain::user::UserId;

#[derive(Debug)]
pub struct ImportCommand {
    pub user_id: UserId,
    pub data: Vec<u8>,
}
