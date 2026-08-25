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
    pub fn into_domain(self) -> User {
        let role = match self.role.as_str() {
            "Admin" => UserRole::Admin,
            _ => UserRole::User,
        };

        User::from_persistence(UserData {
            id: UserId::from_uuid(self.id.parse().unwrap()),
            username: Username::from_persistence(self.username),
            email: Email::from_persistence(self.email),
            password_hash: PasswordHash::new(self.password_hash),
            display_name: self.display_name.map(DisplayName::from_persistence),
            timezone: self.timezone.map(Timezone::from_persistence),
            role,
            created_at: self.created_at.parse().unwrap(),
            updated_at: self.updated_at.parse().unwrap(),
        })
    }
}
