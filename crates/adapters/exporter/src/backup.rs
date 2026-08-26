use std::io::{Cursor, Write};

use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use chrono::Weekday;

use api_types::dimension::DimensionPayload;
use domain::activity::Activity;
use domain::dimension::ComposedEntry;
use domain::errors::DomainError;
use domain::metric::DailyMetric;
use domain::ports::UserBackup;
use domain::reminder::Reminder;

use super::shared::{io_err, json_err, zip_err};

pub const BACKUP_FORMAT_VERSION: u32 = 2;
pub const BACKUP_MANIFEST: &str = "backup.json";

pub struct ZipBackupWriter;

#[async_trait::async_trait]
impl domain::ports::BackupWriterPort for ZipBackupWriter {
    async fn write(&self, backup: &UserBackup) -> Result<Vec<u8>, DomainError> {
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file(BACKUP_MANIFEST, options).map_err(zip_err)?;
        zip.write_all(&manifest(backup)?).map_err(io_err)?;

        for photo in &backup.media.photos {
            zip.start_file(format!("photos/{}", photo.id), options)
                .map_err(zip_err)?;
            zip.write_all(&photo.data).map_err(io_err)?;
        }

        for memo in &backup.media.voice_memos {
            zip.start_file(format!("voice_memos/{}", memo.id), options)
                .map_err(zip_err)?;
            zip.write_all(&memo.data).map_err(io_err)?;
        }

        Ok(zip.finish().map_err(zip_err)?.into_inner())
    }
}

fn manifest(backup: &UserBackup) -> Result<Vec<u8>, DomainError> {
    let manifest = BackupManifest {
        version: BACKUP_FORMAT_VERSION,
        entries: backup.entries.iter().map(BackedUpEntry::from).collect(),
        metrics: backup.metrics.iter().map(BackedUpMetric::from).collect(),
        cycle_starts: backup
            .cycle_starts
            .iter()
            .map(|start| start.date().to_string())
            .collect(),
        activities: backup
            .activities
            .iter()
            .map(BackedUpActivity::from)
            .collect(),
        reminders: backup
            .reminders
            .iter()
            .map(BackedUpReminder::from)
            .collect(),
        tracks_cycle: backup.preferences.tracks_cycle(),
    };

    serde_json::to_vec_pretty(&manifest).map_err(json_err)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifest {
    pub version: u32,
    pub entries: Vec<BackedUpEntry>,
    pub metrics: Vec<BackedUpMetric>,
    pub cycle_starts: Vec<String>,
    pub activities: Vec<BackedUpActivity>,
    pub reminders: Vec<BackedUpReminder>,
    pub tracks_cycle: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpEntry {
    pub mood: u8,
    pub logged_at: String,
    pub dimensions: Vec<DimensionPayload>,
}

impl From<&ComposedEntry> for BackedUpEntry {
    fn from(composed: &ComposedEntry) -> Self {
        Self {
            mood: composed.entry.mood().value(),
            logged_at: composed.entry.logged_at().to_rfc3339(),
            dimensions: composed
                .dimensions
                .iter()
                .map(DimensionPayload::from)
                .collect(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpMetric {
    pub date: String,
    pub kind: String,
    pub value: i64,
    pub provider: Option<String>,
}

impl From<&DailyMetric> for BackedUpMetric {
    fn from(metric: &DailyMetric) -> Self {
        Self {
            date: metric.date().to_string(),
            kind: metric.kind().name().to_string(),
            value: metric.value().count(),
            provider: metric
                .source()
                .provider()
                .map(|name| name.value().to_string()),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpActivity {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub archived: bool,
}

impl From<&Activity> for BackedUpActivity {
    fn from(activity: &Activity) -> Self {
        Self {
            id: activity.id().value().to_string(),
            name: activity.name().value().to_string(),
            category: activity.category().map(|c| c.value().to_string()),
            archived: activity.is_archived(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackedUpReminder {
    pub enabled: bool,
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub saturday: Option<String>,
    pub sunday: Option<String>,
}

impl From<&Reminder> for BackedUpReminder {
    fn from(reminder: &Reminder) -> Self {
        let at = |day| {
            reminder
                .schedule()
                .time_for(day)
                .map(|time| time.format("%H:%M").to_string())
        };

        Self {
            enabled: reminder.is_enabled(),
            monday: at(Weekday::Mon),
            tuesday: at(Weekday::Tue),
            wednesday: at(Weekday::Wed),
            thursday: at(Weekday::Thu),
            friday: at(Weekday::Fri),
            saturday: at(Weekday::Sat),
            sunday: at(Weekday::Sun),
        }
    }
}
