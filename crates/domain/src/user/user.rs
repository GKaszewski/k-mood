use chrono::{DateTime, Utc};

use super::{DisplayName, Email, PasswordHash, Timezone, UserId, UserRole, Username};

pub struct UserData {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub display_name: Option<DisplayName>,
    pub timezone: Option<Timezone>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    username: Username,
    email: Email,
    password_hash: PasswordHash,
    display_name: Option<DisplayName>,
    timezone: Option<Timezone>,
    role: UserRole,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: Username, email: Email, password_hash: PasswordHash) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::generate(),
            username,
            email,
            password_hash,
            display_name: None,
            timezone: None,
            role: UserRole::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_persistence(data: UserData) -> Self {
        Self {
            id: data.id,
            username: data.username,
            email: data.email,
            password_hash: data.password_hash,
            display_name: data.display_name,
            timezone: data.timezone,
            role: data.role,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn display_name(&self) -> Option<&DisplayName> {
        self.display_name.as_ref()
    }

    pub fn timezone(&self) -> Option<&Timezone> {
        self.timezone.as_ref()
    }

    pub fn role(&self) -> UserRole {
        self.role
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

impl User {
    pub fn update_display_name(&mut self, display_name: Option<DisplayName>) {
        self.display_name = display_name;
        self.touch();
    }

    pub fn update_timezone(&mut self, timezone: Option<Timezone>) {
        self.timezone = timezone;
        self.touch();
    }

    pub fn update_password(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash;
        self.touch();
    }

    pub fn promote_to_admin(&mut self) {
        self.role = UserRole::Admin;
        self.touch();
    }

    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}
