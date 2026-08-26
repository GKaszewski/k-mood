use domain::errors::DomainError;
use domain::user::{
    DisplayName, Email, PasswordHash, Timezone, User, UserData, UserId, UserRole, Username,
};

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub timezone: Option<String>,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

impl UserRow {
    pub fn into_domain(self) -> Result<User, DomainError> {
        let malformed = || DomainError::InvalidInput("stored user row is malformed".into());

        let role = match self.role.as_str() {
            "Admin" => UserRole::Admin,
            _ => UserRole::User,
        };

        Ok(User::from_persistence(UserData {
            id: UserId::from_uuid(self.id.parse().map_err(|_| malformed())?),
            username: Username::from_persistence(self.username),
            email: Email::from_persistence(self.email),
            password_hash: PasswordHash::new(self.password_hash),
            display_name: self.display_name.map(DisplayName::from_persistence),
            timezone: self.timezone.as_deref().and_then(resolve_timezone),
            role,
            created_at: self.created_at.parse().map_err(|_| malformed())?,
            updated_at: self.updated_at.parse().map_err(|_| malformed())?,
        }))
    }
}

fn resolve_timezone(stored: &str) -> Option<Timezone> {
    match Timezone::from_persistence(stored) {
        Ok(timezone) => Some(timezone),
        Err(_) => {
            tracing::warn!(
                timezone = stored,
                "stored timezone is not in the IANA database, treating it as unset"
            );
            None
        }
    }
}
