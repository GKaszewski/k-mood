use std::sync::Arc;

use domain::cycle::CycleStartRestore;
use domain::dimension::ComposedEntry;
use domain::entry::DateSpan;
use domain::ports::{
    ActivityQueryPort, BackupMedia, BackupWriterPort, CycleStartQueryPort, DailyMetricQueryPort,
    EntryDimensionPort, MediaBlob, MediaStoragePort, MoodEntryQueryPort, ReminderQueryPort,
    UserPreferencesQueryPort,
};
use domain::user::UserId;

use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

pub struct Deps {
    pub entries: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub activities: Arc<dyn ActivityQueryPort>,
    pub reminders: Arc<dyn ReminderQueryPort>,
    pub metrics: Arc<dyn DailyMetricQueryPort>,
    pub cycles: Arc<dyn CycleStartQueryPort>,
    pub preferences: Arc<dyn UserPreferencesQueryPort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub writer: Arc<dyn BackupWriterPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<Vec<u8>, ApplicationError> {
    let entries = EntryComposer::new(deps.dimensions.clone())
        .compose(deps.entries.find_by_user(&user_id, None, None).await?)
        .await?;

    let media = gather_media(&entries, deps).await?;

    let backup = domain::ports::UserBackup {
        metrics: deps.metrics.find_by_span(&user_id, &everything()).await?,
        cycle_starts: deps
            .cycles
            .find_by_user(&user_id)
            .await?
            .into_iter()
            .map(CycleStartRestore::on)
            .collect(),
        activities: deps.activities.find_by_user(&user_id).await?,
        reminders: deps.reminders.find_by_user(&user_id).await?,
        preferences: preferences_of(&user_id, &deps.preferences).await?,
        entries,
        media,
    };

    tracing::info!(
        entries = backup.entries.len(),
        metrics = backup.metrics.len(),
        cycle_starts = backup.cycle_starts.len(),
        activities = backup.activities.len(),
        reminders = backup.reminders.len(),
        photos = backup.media.photos.len(),
        voice_memos = backup.media.voice_memos.len(),
        "writing a complete backup"
    );

    Ok(deps.writer.write(&backup).await?)
}

fn everything() -> DateSpan {
    let first = domain::entry::Date::from_persistence(
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default(),
    );
    let last = domain::entry::Date::from_persistence(
        chrono::NaiveDate::from_ymd_opt(9999, 12, 31).unwrap_or_default(),
    );

    DateSpan::new(first, last).expect("1970 precedes 9999")
}

async fn gather_media(
    entries: &[ComposedEntry],
    deps: &Deps,
) -> Result<BackupMedia, ApplicationError> {
    let mut photos = Vec::new();
    let mut voice_memos = Vec::new();

    for entry in entries {
        for photo_id in entry.photos() {
            if let Some(file) = deps.media_storage.get_photo(photo_id).await? {
                photos.push(MediaBlob {
                    id: photo_id.value().to_string(),
                    data: file.data,
                });
            }
        }
        for memo_id in entry.voice_memos() {
            if let Some(file) = deps.media_storage.get_voice_memo(memo_id).await? {
                voice_memos.push(MediaBlob {
                    id: memo_id.value().to_string(),
                    data: file.data,
                });
            }
        }
    }

    Ok(BackupMedia {
        photos,
        voice_memos,
    })
}
