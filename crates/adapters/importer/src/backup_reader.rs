use api_types::dimension::DimensionPayload;
use domain::dimension::DimensionValue;
use domain::errors::DomainError;
use domain::ports::{
    RestorableActivity, RestorableContents, RestorableEntry, RestorableMetric, RestorableReminder,
};

use super::kmood_backup::KmoodBackupReader;

pub struct KmoodBackupAdapter;

#[async_trait::async_trait]
impl domain::ports::BackupReaderPort for KmoodBackupAdapter {
    async fn read(&self, data: &[u8]) -> Result<RestorableContents, DomainError> {
        let read = KmoodBackupReader::read(data)?;
        let manifest = read.manifest;

        Ok(RestorableContents {
            entries: manifest
                .entries
                .into_iter()
                .map(|held| RestorableEntry {
                    mood: held.mood,
                    logged_at: held.logged_at,
                    dimensions: readable_dimensions(held.dimensions),
                })
                .collect(),
            metrics: manifest
                .metrics
                .into_iter()
                .map(|held| RestorableMetric {
                    date: held.date,
                    kind: held.kind,
                    value: held.value,
                    provider: held.provider,
                })
                .collect(),
            cycle_starts: manifest.cycle_starts,
            activities: manifest
                .activities
                .into_iter()
                .map(|held| RestorableActivity {
                    id: held.id,
                    name: held.name,
                    category: held.category,
                    archived: held.archived,
                })
                .collect(),
            reminders: manifest
                .reminders
                .into_iter()
                .map(|held| RestorableReminder {
                    enabled: held.enabled,
                    times: [
                        held.monday,
                        held.tuesday,
                        held.wednesday,
                        held.thursday,
                        held.friday,
                        held.saturday,
                        held.sunday,
                    ],
                })
                .collect(),
            tracks_cycle: manifest.tracks_cycle,
            photos: read.photos.into_iter().collect(),
            voice_memos: read.voice_memos.into_iter().collect(),
        })
    }
}

fn readable_dimensions(payloads: Vec<DimensionPayload>) -> Vec<DimensionValue> {
    payloads
        .into_iter()
        .filter_map(|payload| match payload.into_dimension() {
            Ok(dimension) => Some(dimension),
            Err(error) => {
                tracing::warn!(%error, "a backed-up dimension could not be read");
                None
            }
        })
        .collect()
}
