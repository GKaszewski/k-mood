use crate::activity::Activity;
use crate::cycle::CycleStartRestore;
use crate::dimension::ComposedEntry;
use crate::errors::DomainError;
use crate::metric::DailyMetric;
use crate::reminder::Reminder;
use crate::user::UserPreferences;

pub struct MediaBlob {
    pub id: String,
    pub data: Vec<u8>,
}

pub struct BackupMedia {
    pub photos: Vec<MediaBlob>,
    pub voice_memos: Vec<MediaBlob>,
}

pub struct UserBackup {
    pub entries: Vec<ComposedEntry>,
    pub metrics: Vec<DailyMetric>,
    pub cycle_starts: Vec<CycleStartRestore>,
    pub activities: Vec<Activity>,
    pub reminders: Vec<Reminder>,
    pub preferences: UserPreferences,
    pub media: BackupMedia,
}

pub struct SharedExtract {
    pub entries: Vec<ComposedEntry>,
    pub activities: Vec<Activity>,
}

#[async_trait::async_trait]
pub trait BackupWriterPort: Send + Sync {
    async fn write(&self, backup: &UserBackup) -> Result<Vec<u8>, DomainError>;
}

#[async_trait::async_trait]
pub trait ExtractWriterPort: Send + Sync {
    async fn write(&self, extract: &SharedExtract) -> Result<Vec<u8>, DomainError>;
}

#[async_trait::async_trait]
pub trait ImportSourcePort: Send + Sync {
    async fn read_entries(&self, data: &[u8]) -> Result<Vec<ImportedRow>, DomainError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedRow {
    pub mood: u8,
    pub date: String,
    pub time: String,
    pub activities: Vec<String>,
    pub note: Option<String>,
}
