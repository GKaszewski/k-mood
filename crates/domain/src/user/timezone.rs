use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Timezone(String);

impl Timezone {
    pub fn new(tz: impl Into<String>) -> Result<Self, DomainError> {
        let trimmed = tz.into().trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidInput("timezone cannot be empty".into()));
        }
        if !trimmed.contains('/') {
            return Err(DomainError::InvalidInput(
                "timezone must be in IANA format (e.g. Europe/Warsaw)".into(),
            ));
        }
        Ok(Self(trimmed))
    }

    pub fn from_persistence(tz: String) -> Self {
        Self(tz)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
