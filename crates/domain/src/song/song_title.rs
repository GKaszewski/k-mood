use crate::errors::DomainError;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct SongTitle(String);

impl SongTitle {
    pub fn new(title: impl Into<String>) -> Result<Self, DomainError> {
        let trimmed = title.into().trim().to_string();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidInput(
                "song title cannot be empty".into(),
            ));
        }
        Ok(Self(trimmed))
    }

    pub fn from_persistence(title: String) -> Self {
        Self(title)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
