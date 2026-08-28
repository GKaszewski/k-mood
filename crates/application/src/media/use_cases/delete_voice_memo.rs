use std::sync::Arc;

use domain::attachment::VoiceMemoId;
use domain::ports::{MediaOwnershipPort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::media::custody::verify_custody;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
    pub ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    voice_memo_id: VoiceMemoId,
    caller_id: UserId,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let media = (&voice_memo_id).into();
    verify_custody(media, &caller_id, &deps.ownership).await?;

    deps.storage.delete_voice_memo(&voice_memo_id).await?;
    deps.ownership.forget(media).await?;

    Ok(())
}
