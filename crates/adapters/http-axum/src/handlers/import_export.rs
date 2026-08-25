use axum::Json;
use axum::extract::{Multipart, State};
use axum::http::header;
use axum::response::IntoResponse;

use api_types::responses::ImportResultResponse;
use application::export::use_cases::export_user_data;
use application::import::commands::ImportCommand;
use application::import::use_cases::import_entries;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, extract_file_bytes};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/data/export", tag = "data", security(("bearer" = [])),
    responses((status = 200, description = "ZIP archive with user data"))
)]
pub async fn handle_export(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    let deps = export_user_data::Deps {
        entry_query: state.entry_query,
        activity_query: state.activity_query,
        reminder_query: state.reminder_query,
        media_storage: state.media_storage,
        exporter: state.export_port.clone(),
    };
    let data = export_user_data::execute(user_id, &deps).await?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/zip"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"k-mood-export.zip\"",
            ),
        ],
        data,
    ))
}

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
        preset: state.preset_config,
    };
    let result = import_entries::execute(cmd, &deps).await?;
    Ok(Json(ImportResultResponse::from(result)))
}
