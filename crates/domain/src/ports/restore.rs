use crate::attachment::ContentType;
use crate::errors::DomainError;

pub struct RestorableEntry {
    pub mood: u8,
    pub logged_at: String,
    pub dimensions: Vec<crate::dimension::DimensionValue>,
}

pub struct RestorableMetric {
    pub date: String,
    pub kind: String,
    pub value: i64,
    pub provider: Option<String>,
}

pub struct RestorableActivity {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub archived: bool,
}

pub struct RestorableReminder {
    pub enabled: bool,
    pub times: [Option<String>; 7],
}

pub struct RestorableContents {
    pub entries: Vec<RestorableEntry>,
    pub metrics: Vec<RestorableMetric>,
    pub cycle_starts: Vec<String>,
    pub activities: Vec<RestorableActivity>,
    pub reminders: Vec<RestorableReminder>,
    pub tracks_cycle: bool,
    pub photos: Vec<RestorableMedia>,
    pub voice_memos: Vec<RestorableMedia>,
}

pub struct RestorableMedia {
    pub id: String,
    pub data: Vec<u8>,
    pub content_type: Option<ContentType>,
}

#[async_trait::async_trait]
pub trait BackupReaderPort: Send + Sync {
    async fn read(&self, data: &[u8]) -> Result<RestorableContents, DomainError>;
}
