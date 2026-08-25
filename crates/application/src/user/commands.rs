use domain::user::{DisplayName, Email, Timezone, UserId, Username};

#[derive(Debug)]
pub struct RegisterCommand {
    pub username: Username,
    pub email: Email,
    pub password: String,
}

#[derive(Debug)]
pub struct LoginCommand {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UpdateProfileCommand {
    pub user_id: UserId,
    pub display_name: Option<DisplayName>,
    pub timezone: Option<Timezone>,
}

#[derive(Debug)]
pub struct ChangePasswordCommand {
    pub user_id: UserId,
    pub current_password: String,
    pub new_password: String,
}
