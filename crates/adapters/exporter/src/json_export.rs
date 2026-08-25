use std::io::{Cursor, Write};

use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use domain::errors::DomainError;
use domain::ports::UserExport;

pub struct JsonExportAdapter;

#[async_trait::async_trait]
impl domain::ports::ExportPort for JsonExportAdapter {
    async fn export_user_data(&self, data: &UserExport) -> Result<Vec<u8>, DomainError> {
        let buf = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buf);
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        let json = build_data_json(data)?;
        zip.start_file("data.json", options).map_err(zip_err)?;
        zip.write_all(&json).map_err(io_err)?;

        for photo in &data.photos {
            zip.start_file(format!("photos/{}", photo.id), options)
                .map_err(zip_err)?;
            zip.write_all(&photo.data).map_err(io_err)?;
        }

        for memo in &data.voice_memos {
            zip.start_file(format!("voice_memos/{}", memo.id), options)
                .map_err(zip_err)?;
            zip.write_all(&memo.data).map_err(io_err)?;
        }

        let cursor = zip.finish().map_err(zip_err)?;
        Ok(cursor.into_inner())
    }
}

fn build_data_json(data: &UserExport) -> Result<Vec<u8>, DomainError> {
    let export = ExportData {
        version: "1.0",
        entries: data.entries.iter().map(EntryExport::from).collect(),
        activities: data.activities.iter().map(ActivityExport::from).collect(),
        reminder_count: data.reminders.len(),
    };

    serde_json::to_vec_pretty(&export)
        .map_err(|e| DomainError::InvalidInput(format!("json serialization failed: {e}")))
}

fn zip_err(e: zip::result::ZipError) -> DomainError {
    DomainError::InvalidInput(format!("zip error: {e}"))
}

fn io_err(e: std::io::Error) -> DomainError {
    DomainError::InvalidInput(format!("io error: {e}"))
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportData<'a> {
    version: &'a str,
    entries: Vec<EntryExport>,
    activities: Vec<ActivityExport>,
    reminder_count: usize,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct EntryExport {
    id: String,
    mood: u8,
    mood_label: String,
    logged_at: String,
    activities: Vec<String>,
    content: Option<String>,
    photos: Vec<String>,
    voice_memos: Vec<String>,
}

impl From<&domain::entry::MoodEntry> for EntryExport {
    fn from(entry: &domain::entry::MoodEntry) -> Self {
        Self {
            id: entry.id().value().to_string(),
            mood: entry.mood().value(),
            mood_label: format!("{:?}", entry.mood()),
            logged_at: entry.logged_at().to_rfc3339(),
            activities: entry
                .activities()
                .iter()
                .map(|a| a.value().to_string())
                .collect(),
            content: entry.content().map(|c| c.value().to_string()),
            photos: entry
                .photos()
                .iter()
                .map(|p| p.value().to_string())
                .collect(),
            voice_memos: entry
                .voice_memos()
                .iter()
                .map(|v| v.value().to_string())
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ActivityExport {
    id: String,
    name: String,
    category: Option<String>,
    archived: bool,
}

impl From<&domain::activity::Activity> for ActivityExport {
    fn from(activity: &domain::activity::Activity) -> Self {
        Self {
            id: activity.id().value().to_string(),
            name: activity.name().value().to_string(),
            category: activity.category().map(|c| c.value().to_string()),
            archived: activity.is_archived(),
        }
    }
}
