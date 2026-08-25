use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GeneratedToken {
    token: String,
    expires_at: DateTime<Utc>,
}

impl GeneratedToken {
    pub fn new(token: String, expires_at: DateTime<Utc>) -> Self {
        Self { token, expires_at }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn expires_at(&self) -> &DateTime<Utc> {
        &self.expires_at
    }
}
