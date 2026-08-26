use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecordingId(uuid::Uuid);

impl RecordingId {
    pub fn new(id: &str) -> Result<Self, DomainError> {
        id.trim()
            .parse::<uuid::Uuid>()
            .map(Self)
            .map_err(|_| DomainError::InvalidInput("recording id must be a UUID".into()))
    }

    pub fn from_uuid(id: uuid::Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> uuid::Uuid {
        self.0
    }
}
