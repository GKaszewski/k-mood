use domain::activity::ActivityId;
use domain::user::UserId;

use application::import::use_cases::import_entries::ImportResult;

use crate::errors::ApiValidationError;
use crate::requests::ReplaceActivityRequest;
use crate::responses::ImportResultResponse;

impl ReplaceActivityRequest {
    pub fn into_parts(
        self,
        user_id: UserId,
    ) -> Result<(UserId, ActivityId, ActivityId), ApiValidationError> {
        let old = super::shared::parse_uuid(&self.old_activity_id)?.into();
        let new = super::shared::parse_uuid(&self.new_activity_id)?.into();
        Ok((user_id, old, new))
    }
}

impl From<ImportResult> for ImportResultResponse {
    fn from(result: ImportResult) -> Self {
        Self {
            imported: result.imported,
            skipped: result.skipped,
            errors: result.errors,
        }
    }
}
