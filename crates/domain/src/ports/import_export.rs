use crate::activity::Activity;
use crate::entry::MoodEntry;
use crate::errors::DomainError;
use crate::reminder::Reminder;

pub struct MediaBlob {
    pub id: String,
    pub data: Vec<u8>,
}

pub struct UserExport {
    pub entries: Vec<MoodEntry>,
    pub activities: Vec<Activity>,
    pub reminders: Vec<Reminder>,
    pub photos: Vec<MediaBlob>,
    pub voice_memos: Vec<MediaBlob>,
}

#[async_trait::async_trait]
pub trait ExportPort: Send + Sync {
    async fn export_user_data(&self, data: &UserExport) -> Result<Vec<u8>, DomainError>;
}

#[async_trait::async_trait]
pub trait ImportSourcePort: Send + Sync {
    async fn read_entries(&self, data: &[u8]) -> Result<Vec<ImportedRow>, DomainError>;
}

pub struct ImportedRow {
    pub mood: u8,
    pub date: String,
    pub time: String,
    pub activities: Vec<String>,
    pub note: Option<String>,
}
