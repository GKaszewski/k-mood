use std::sync::Arc;

use domain::attachment::{MediaUpload, VoiceMemoId};
use domain::ports::MediaStoragePort;

use crate::errors::ApplicationError;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(upload: MediaUpload, deps: &Deps) -> Result<VoiceMemoId, ApplicationError> {
    let voice_memo_id = deps.storage.store_voice_memo(upload).await?;
    Ok(voice_memo_id)
}
