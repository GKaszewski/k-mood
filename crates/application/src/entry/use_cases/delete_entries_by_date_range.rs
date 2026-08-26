use std::sync::Arc;

use domain::dimension::ComposedEntry;
use domain::entry::DateRange;
use domain::ports::{CascadeDeletePort, EntryDimensionPort, MediaStoragePort, MoodEntryQueryPort};
use domain::user::UserId;

use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::media::cleanup::delete_media_for;

pub struct Deps {
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub query: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub media_storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    user_id: UserId,
    range: &DateRange,
    deps: &Deps,
) -> Result<u64, ApplicationError> {
    let media = media_of_entries_about_to_be_deleted(&user_id, range, deps).await?;

    let deleted = deps
        .cascade
        .delete_entries_in_range(&user_id, range)
        .await?;

    delete_media_for(&media, &deps.media_storage).await;

    Ok(deleted.len() as u64)
}

async fn media_of_entries_about_to_be_deleted(
    user_id: &UserId,
    range: &DateRange,
    deps: &Deps,
) -> Result<Vec<ComposedEntry>, ApplicationError> {
    let doomed = deps.query.find_by_date_range(user_id, range).await?;

    EntryComposer::new(deps.dimensions.clone())
        .compose(doomed)
        .await
}
