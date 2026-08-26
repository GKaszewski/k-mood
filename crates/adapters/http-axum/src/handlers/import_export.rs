use axum::Json;
use axum::extract::{Multipart, State};

use api_types::responses::ImportResultResponse;
use application::import::commands::ImportCommand;
use application::import::use_cases::import_entries;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, extract_file_bytes};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/data/import", tag = "data", security(("bearer" = [])),
    responses((status = 200, body = ImportResultResponse))
)]
pub async fn handle_import(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    multipart: Multipart,
) -> Result<Json<ImportResultResponse>, ApiError> {
    let data = extract_file_bytes(multipart).await?;

    let cmd = ImportCommand { user_id, data };
    let deps = import_entries::Deps {
        source: state.import_source.clone(),
        entry_command: state.entry_command,
        entry_query: state.entry_query,
        activity_command: state.activity_command,
        activity_query: state.activity_query,
        dimensions: state.dimensions.clone(),
        users: state.user_query,
        preset: state.preset_config,
    };
    let result = import_entries::execute(cmd, &deps).await?;
    Ok(Json(ImportResultResponse::from(result)))
}
