use std::sync::Arc;

use domain::attachment::VoiceMemoId;
use domain::ports::MediaStoragePort;

use crate::errors::ApplicationError;

pub struct Deps {
    pub storage: Arc<dyn MediaStoragePort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(voice_memo_id: VoiceMemoId, deps: &Deps) -> Result<(), ApplicationError> {
    deps.storage.delete_voice_memo(&voice_memo_id).await?;
    Ok(())
}
