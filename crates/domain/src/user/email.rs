use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, DomainError> {
        let email = email.into().trim().to_lowercase();

        email_address::EmailAddress::parse_with_options(&email, email_address::Options::default())
            .map_err(|_| DomainError::InvalidInput(format!("invalid email address: {email}")))?;

        Ok(Self(email))
    }

    pub fn from_persistence(email: String) -> Self {
        Self(email)
    }
}

impl Email {
    pub fn value(&self) -> &str {
        &self.0
    }
}
