use std::sync::Arc;

use domain::dimension::ComposedEntry;
use domain::ports::MediaStoragePort;

pub async fn delete_media_for(entries: &[ComposedEntry], storage: &Arc<dyn MediaStoragePort>) {
    for entry in entries {
        for photo_id in entry.photos() {
            if let Err(e) = storage.delete_photo(photo_id).await {
                tracing::warn!(%e, "failed to delete photo blob");
            }
        }
        for memo_id in entry.voice_memos() {
            if let Err(e) = storage.delete_voice_memo(memo_id).await {
                tracing::warn!(%e, "failed to delete voice memo blob");
            }
        }
    }
}
