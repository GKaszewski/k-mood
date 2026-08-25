use std::sync::Arc;

use domain::ports::{
    ActivityQueryPort, ExportPort, MediaBlob, MediaStoragePort, MoodEntryQueryPort,
    ReminderQueryPort, UserExport,
};
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
    pub activity_query: Arc<dyn ActivityQueryPort>,
    pub reminder_query: Arc<dyn ReminderQueryPort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub exporter: Arc<dyn ExportPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<Vec<u8>, ApplicationError> {
    let entries = deps.entry_query.find_by_user(&user_id, None, None).await?;
    let activities = deps.activity_query.find_by_user(&user_id).await?;
    let reminders = deps.reminder_query.find_by_user(&user_id).await?;

    let mut photos = Vec::new();
    let mut voice_memos = Vec::new();

    for entry in &entries {
        for photo_id in entry.photos() {
            if let Some(file) = deps.media_storage.get_photo(photo_id).await? {
                photos.push(MediaBlob {
                    id: photo_id.value().to_string(),
                    data: file.data,
                });
            }
        }
        for voice_memo_id in entry.voice_memos() {
            if let Some(file) = deps.media_storage.get_voice_memo(voice_memo_id).await? {
                voice_memos.push(MediaBlob {
                    id: voice_memo_id.value().to_string(),
                    data: file.data,
                });
            }
        }
    }

    tracing::info!(
        entries = entries.len(),
        activities = activities.len(),
        photos = photos.len(),
        voice_memos = voice_memos.len(),
        "exporting user data"
    );

    let export = UserExport {
        entries,
        activities,
        reminders,
        photos,
        voice_memos,
    };

    let data = deps.exporter.export_user_data(&export).await?;
    Ok(data)
}
