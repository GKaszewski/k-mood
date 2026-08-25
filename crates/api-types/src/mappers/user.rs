use domain::user::{DisplayName, Email, Timezone, User, UserId, Username};

use application::user::commands::{
    ChangePasswordCommand, LoginCommand, RegisterCommand, UpdateProfileCommand,
};

use crate::errors::ApiValidationError;
use crate::requests::{ChangePasswordRequest, LoginRequest, RegisterRequest, UpdateProfileRequest};
use crate::responses::UserResponse;

impl RegisterRequest {
    pub fn into_command(self) -> Result<RegisterCommand, ApiValidationError> {
        let username = Username::new(self.username)?;
        let email = Email::new(self.email)?;

        Ok(RegisterCommand {
            username,
            email,
            password: self.password,
        })
    }
}

impl LoginRequest {
    pub fn into_command(self) -> LoginCommand {
        LoginCommand {
            identifier: self.identifier,
            password: self.password,
        }
    }
}

impl UpdateProfileRequest {
    pub fn into_command(self, user_id: UserId) -> Result<UpdateProfileCommand, ApiValidationError> {
        let display_name = self.display_name.map(DisplayName::new).transpose()?;

        let timezone = self.timezone.map(Timezone::new).transpose()?;

        Ok(UpdateProfileCommand {
            user_id,
            display_name,
            timezone,
        })
    }
}

impl ChangePasswordRequest {
    pub fn into_command(self, user_id: UserId) -> ChangePasswordCommand {
        ChangePasswordCommand {
            user_id,
            current_password: self.current_password,
            new_password: self.new_password,
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id().value(),
            username: user.username().value().to_string(),
            email: user.email().value().to_string(),
            display_name: user.display_name().map(|d| d.value().to_string()),
            timezone: user.timezone().map(|t| t.value().to_string()),
            role: format!("{:?}", user.role()),
            created_at: *user.created_at(),
        }
    }
}
