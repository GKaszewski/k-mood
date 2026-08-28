use std::sync::Arc;

use domain::attachment::{PhotoId, VoiceMemoId};
use domain::dimension::{ComposedEntry, DimensionValue, lookup};
use domain::ports::MediaStoragePort;

pub struct DroppedMedia {
    photos: Vec<PhotoId>,
    voice_memos: Vec<VoiceMemoId>,
}

impl DroppedMedia {
    pub fn between(previous: &ComposedEntry, kept: &[DimensionValue]) -> Self {
        let still_here_photos = lookup::photos_in(kept);
        let still_here_memos = lookup::voice_memos_in(kept);

        Self {
            photos: previous
                .photos()
                .iter()
                .filter(|photo_id| !still_here_photos.contains(photo_id))
                .cloned()
                .collect(),
            voice_memos: previous
                .voice_memos()
                .iter()
                .filter(|memo_id| !still_here_memos.contains(memo_id))
                .cloned()
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.photos.is_empty() && self.voice_memos.is_empty()
    }

    pub fn photos(&self) -> &[PhotoId] {
        &self.photos
    }

    pub fn voice_memos(&self) -> &[VoiceMemoId] {
        &self.voice_memos
    }
}

pub async fn delete_dropped(dropped: &DroppedMedia, storage: &Arc<dyn MediaStoragePort>) {
    for photo_id in dropped.photos() {
        if let Err(error) = storage.delete_photo(photo_id).await {
            tracing::warn!(%error, "failed to delete a photo dropped from an edit");
        }
    }

    for memo_id in dropped.voice_memos() {
        if let Err(error) = storage.delete_voice_memo(memo_id).await {
            tracing::warn!(%error, "failed to delete a voice memo dropped from an edit");
        }
    }
}
