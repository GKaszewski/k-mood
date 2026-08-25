use std::sync::Arc;

use domain::entry::DateRange;
use domain::ports::{CascadeDeletePort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    range: &DateRange,
    deps: &Deps,
) -> Result<u64, ApplicationError> {
    let entries = deps
        .cascade
        .delete_entries_in_range(&user_id, range)
        .await?;

    for entry in &entries {
        for photo_id in entry.photos() {
            if let Err(e) = deps.media_storage.delete_photo(photo_id).await {
                tracing::warn!(%e, "failed to delete photo blob");
            }
        }
        for memo_id in entry.voice_memos() {
            if let Err(e) = deps.media_storage.delete_voice_memo(memo_id).await {
                tracing::warn!(%e, "failed to delete voice memo blob");
            }
        }
    }

    Ok(entries.len() as u64)
}
