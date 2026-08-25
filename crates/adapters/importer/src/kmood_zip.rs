use std::collections::HashMap;
use std::io::{Cursor, Read};

use zip::ZipArchive;

use domain::errors::DomainError;
use domain::ports::ImportedRow;

pub struct KmoodZipImportAdapter;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportData {
    entries: Vec<EntryData>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EntryData {
    mood: u8,
    logged_at: String,
    activities: Vec<String>,
    content: Option<String>,
    photos: Vec<String>,
    voice_memos: Vec<String>,
}

pub struct KmoodImportEntry {
    pub row: ImportedRow,
    pub photo_ids: Vec<String>,
    pub voice_memo_ids: Vec<String>,
}

pub struct KmoodImportResult {
    pub entries: Vec<KmoodImportEntry>,
    pub photos: HashMap<String, Vec<u8>>,
    pub voice_memos: HashMap<String, Vec<u8>>,
}

impl KmoodZipImportAdapter {
    pub fn extract(data: &[u8]) -> Result<KmoodImportResult, DomainError> {
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| DomainError::InvalidInput(format!("invalid zip file: {e}")))?;

        let json_data = read_file_from_zip(&mut archive, "data.json")?
            .ok_or_else(|| DomainError::InvalidInput("missing data.json in archive".into()))?;

        let export: ExportData = serde_json::from_slice(&json_data)
            .map_err(|e| DomainError::InvalidInput(format!("invalid data.json: {e}")))?;

        let mut photos = HashMap::new();
        let mut voice_memos = HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| DomainError::InvalidInput(format!("zip read error: {e}")))?;

            let name = file.name().to_string();

            if let Some(id) = name.strip_prefix("photos/")
                && !id.is_empty()
            {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)
                    .map_err(|e| DomainError::InvalidInput(format!("failed to read photo: {e}")))?;
                photos.insert(id.to_string(), buf);
            } else if let Some(id) = name.strip_prefix("voice_memos/")
                && !id.is_empty()
            {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).map_err(|e| {
                    DomainError::InvalidInput(format!("failed to read voice memo: {e}"))
                })?;
                voice_memos.insert(id.to_string(), buf);
            }
        }

        let entries: Vec<KmoodImportEntry> = export
            .entries
            .into_iter()
            .map(|e| {
                let (date, time) = split_datetime(&e.logged_at);
                KmoodImportEntry {
                    row: ImportedRow {
                        mood: e.mood,
                        date,
                        time,
                        activities: e.activities,
                        note: e.content,
                    },
                    photo_ids: e.photos,
                    voice_memo_ids: e.voice_memos,
                }
            })
            .collect();

        tracing::info!(
            entries = entries.len(),
            photos = photos.len(),
            voice_memos = voice_memos.len(),
            "extracted k-mood archive"
        );

        Ok(KmoodImportResult {
            entries,
            photos,
            voice_memos,
        })
    }
}

fn read_file_from_zip(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    name: &str,
) -> Result<Option<Vec<u8>>, DomainError> {
    let mut file = match archive.by_name(name) {
        Ok(f) => f,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(e) => return Err(DomainError::InvalidInput(format!("zip error: {e}"))),
    };

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| DomainError::InvalidInput(format!("failed to read {name}: {e}")))?;

    Ok(Some(buf))
}

fn split_datetime(rfc3339: &str) -> (String, String) {
    if let Some(t_pos) = rfc3339.find('T') {
        let date = rfc3339[..t_pos].to_string();
        let time = rfc3339[t_pos + 1..].to_string();
        (date, time)
    } else {
        (rfc3339.to_string(), "00:00".to_string())
    }
}
