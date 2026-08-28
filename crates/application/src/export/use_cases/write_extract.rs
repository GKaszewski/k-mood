use std::sync::Arc;

use domain::ports::{
    ActivityQueryPort, EntryDimensionPort, ExtractWriterPort, MoodEntryQueryPort, SharedExtract,
};
use domain::user::UserId;

use crate::entry::composition::EntryComposer;
use crate::errors::ApplicationError;

pub struct Deps {
    pub entries: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub activities: Arc<dyn ActivityQueryPort>,
    pub writer: Arc<dyn ExtractWriterPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<Vec<u8>, ApplicationError> {
    let entries = EntryComposer::new(deps.dimensions.clone())
        .compose(deps.entries.find_all_by_user(&user_id).await?)
        .await?;

    let extract = SharedExtract {
        entries,
        activities: deps.activities.find_by_user(&user_id).await?,
    };

    tracing::info!(
        entries = extract.entries.len(),
        "writing a shareable extract"
    );

    Ok(deps.writer.write(&extract).await?)
}
