use crate::entry::MoodEntryId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobSubject {
    Entry(MoodEntryId),
}

impl JobSubject {
    pub fn key(&self) -> String {
        match self {
            Self::Entry(id) => id.value().to_string(),
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        key.parse()
            .ok()
            .map(MoodEntryId::from_uuid)
            .map(Self::Entry)
    }
}
