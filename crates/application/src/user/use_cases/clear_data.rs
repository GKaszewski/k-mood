use std::sync::Arc;

use domain::ports::{CascadeDeletePort, EntryDimensionPort, MediaStoragePort, MoodEntryQueryPort};
use domain::user::UserId;

use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;
use crate::media::cleanup::delete_media_for;

pub struct Deps {
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<(), ApplicationError> {
    let entries = deps.entry_query.find_by_user(&user_id, None, None).await?;

    let composed = EntryComposer::new(deps.dimensions.clone())
        .compose(entries)
        .await?;
    delete_media_for(&composed, &deps.media_storage).await;

    deps.cascade.delete_all_user_data(&user_id).await?;

    tracing::info!(%user_id, "all user data cleared");
    Ok(())
}
