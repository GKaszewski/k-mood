use domain::user::UserId;

pub struct RestoreBackupCommand {
    pub user_id: UserId,
    pub data: Vec<u8>,
}

impl std::fmt::Debug for RestoreBackupCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RestoreBackupCommand")
            .field("user_id", &self.user_id)
            .field("bytes", &self.data.len())
            .finish()
    }
}
