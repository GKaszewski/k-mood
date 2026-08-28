use std::sync::Arc;

use domain::attachment::MediaRef;
use domain::dimension::ComposedEntry;
use domain::ports::{MediaOwnershipPort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;

pub async fn delete_media_for(entries: &[ComposedEntry], storage: &Arc<dyn MediaStoragePort>) {
    for entry in entries {
        for photo_id in entry.photos() {
            if let Err(error) = storage.delete_photo(photo_id).await {
                tracing::warn!(%error, "failed to delete photo blob");
            }
        }
        for memo_id in entry.voice_memos() {
            if let Err(error) = storage.delete_voice_memo(memo_id).await {
                tracing::warn!(%error, "failed to delete voice memo blob");
            }
        }
    }
}

pub async fn delete_everything_owned_by(
    owner: &UserId,
    storage: &Arc<dyn MediaStoragePort>,
    ownership: &Arc<dyn MediaOwnershipPort>,
) -> Result<usize, ApplicationError> {
    let held = ownership.owned_by(owner).await?;

    for media in &held {
        delete_one(*media, storage).await;
    }

    Ok(held.len())
}

async fn delete_one(media: MediaRef, storage: &Arc<dyn MediaStoragePort>) {
    if let Some(photo_id) = media.to_photo()
        && let Err(error) = storage.delete_photo(&photo_id).await
    {
        tracing::warn!(%error, %media, "failed to delete photo blob");
    }

    if let Some(memo_id) = media.to_voice_memo()
        && let Err(error) = storage.delete_voice_memo(&memo_id).await
    {
        tracing::warn!(%error, %media, "failed to delete voice memo blob");
    }
}
