use crate::errors::DomainError;

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: impl Into<String>) -> Result<Self, DomainError> {
        let username = username.into().trim().to_string();

        if username.len() < MIN_LENGTH || username.len() > MAX_LENGTH {
            return Err(DomainError::InvalidInput(format!(
                "username must be between {MIN_LENGTH} and {MAX_LENGTH} characters"
            )));
        }

        let is_valid = username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.');

        if !is_valid {
            return Err(DomainError::InvalidInput(
                "username may only contain alphanumeric characters, underscores, and dots".into(),
            ));
        }

        Ok(Self(username))
    }

    pub fn from_persistence(username: String) -> Self {
        Self(username)
    }
}

impl Username {
    pub fn value(&self) -> &str {
        &self.0
    }
}
