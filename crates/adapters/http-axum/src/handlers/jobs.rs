use axum::Json;
use axum::extract::State;

use api_types::responses::{ErrorResponse, ExhaustedJobResponse};
use application::job::use_cases::list_exhausted_jobs;

use crate::errors::ApiError;
use crate::extractors::Administrator;
use crate::state::AppState;

const MOST_SHOWN: usize = 100;

#[utoipa::path(get, path = "/api/v1/jobs/exhausted", tag = "jobs", security(("bearer" = [])),
    description = "Background work that used every attempt and stopped being retried, with the \
                   reason the last attempt failed. Losing a job costs promptness and never data, \
                   so this is an operator's view of what is stale rather than what is missing. \
                   Needs an administrator's session.",
    responses((status = 200, body = Vec<ExhaustedJobResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list_exhausted(
    State(state): State<AppState>,
    Administrator(_who): Administrator,
) -> Result<Json<Vec<ExhaustedJobResponse>>, ApiError> {
    let deps = list_exhausted_jobs::Deps {
        jobs: state.job_view,
    };

    let stalled = list_exhausted_jobs::execute(MOST_SHOWN, &deps).await?;

    Ok(Json(stalled.into_iter().map(Into::into).collect()))
}
