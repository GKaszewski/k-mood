use std::sync::Arc;

use domain::attachment::{MediaUpload, VoiceMemoId};
use domain::ports::{MediaOwnershipPort, MediaStoragePort};
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
    pub ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps, upload))]
pub async fn execute(
    owner: UserId,
    upload: MediaUpload,
    deps: &Deps,
) -> Result<VoiceMemoId, ApplicationError> {
    let voice_memo_id = deps.storage.store_voice_memo(upload).await?;

    deps.ownership
        .remember(&owner, (&voice_memo_id).into())
        .await?;

    Ok(voice_memo_id)
}
