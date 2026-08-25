use domain::activity::ActivityId;
use domain::user::UserId;

use application::import::use_cases::import_entries::ImportResult;

use crate::errors::ApiValidationError;
use crate::requests::ReplaceActivityRequest;
use crate::responses::{CorrelationResponse, ImportResultResponse};

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

pub fn correlation_response(
    activity_id: ActivityId,
    correlation: Option<f64>,
) -> CorrelationResponse {
    CorrelationResponse {
        activity_id: activity_id.value(),
        correlation,
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
