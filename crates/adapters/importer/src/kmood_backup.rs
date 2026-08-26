use std::collections::HashMap;
use std::io::{Cursor, Read};

use zip::ZipArchive;

use domain::errors::DomainError;
use exporter::{BACKUP_MANIFEST, BackupManifest};

pub struct RestorableBackup {
    pub manifest: BackupManifest,
    pub photos: HashMap<String, Vec<u8>>,
    pub voice_memos: HashMap<String, Vec<u8>>,
}

pub struct KmoodBackupReader;

impl KmoodBackupReader {
    pub fn read(data: &[u8]) -> Result<RestorableBackup, DomainError> {
        let mut archive = ZipArchive::new(Cursor::new(data))
            .map_err(|error| DomainError::InvalidInput(format!("not a zip archive: {error}")))?;

        let manifest = read_named(&mut archive, BACKUP_MANIFEST)?.ok_or_else(|| {
            DomainError::InvalidInput(
                "this archive has no backup.json, so it is not a k-mood backup".into(),
            )
        })?;

        let manifest: BackupManifest = serde_json::from_slice(&manifest).map_err(|error| {
            DomainError::InvalidInput(format!("this backup cannot be read: {error}"))
        })?;

        let (photos, voice_memos) = read_media(&mut archive)?;

        tracing::info!(
            version = manifest.version,
            entries = manifest.entries.len(),
            metrics = manifest.metrics.len(),
            "read a k-mood backup"
        );

        Ok(RestorableBackup {
            manifest,
            photos,
            voice_memos,
        })
    }
}

type Media = (HashMap<String, Vec<u8>>, HashMap<String, Vec<u8>>);

fn read_media(archive: &mut ZipArchive<Cursor<&[u8]>>) -> Result<Media, DomainError> {
    let mut photos = HashMap::new();
    let mut voice_memos = HashMap::new();

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| DomainError::InvalidInput(format!("zip read error: {error}")))?;

        let name = file.name().to_string();

        let destination = match (
            name.strip_prefix("photos/"),
            name.strip_prefix("voice_memos/"),
        ) {
            (Some(id), _) if !id.is_empty() => (&mut photos, id.to_string()),
            (_, Some(id)) if !id.is_empty() => (&mut voice_memos, id.to_string()),
            _ => continue,
        };

        let (into, id) = destination;

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|error| {
            DomainError::InvalidInput(format!("could not read {name}: {error}"))
        })?;

        into.insert(id, bytes);
    }

    Ok((photos, voice_memos))
}

fn read_named(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    name: &str,
) -> Result<Option<Vec<u8>>, DomainError> {
    let mut file = match archive.by_name(name) {
        Ok(file) => file,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(error) => return Err(DomainError::InvalidInput(format!("zip error: {error}"))),
    };

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| DomainError::InvalidInput(format!("could not read {name}: {error}")))?;

    Ok(Some(bytes))
}
