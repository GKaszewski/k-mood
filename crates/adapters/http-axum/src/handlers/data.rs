use axum::Json;
use axum::extract::{Multipart, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use api_types::responses::RestoreOutcomeResponse;
use application::export::use_cases::{write_backup, write_extract};
use application::restore::commands::RestoreBackupCommand;
use application::restore::use_cases::restore_backup;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, extract_file_bytes};
use crate::state::AppState;

const BACKUP_FILENAME: &str = "k-mood-complete-backup.zip";
const EXTRACT_FILENAME: &str = "k-mood-shareable-journal.md";

#[utoipa::path(get, path = "/api/v1/data/backup", tag = "data", security(("bearer" = [])),
    description = "A complete backup: every entry with every dimension, every daily metric, \
                   every cycle start, the activity catalogue, reminders, preferences and all \
                   media. Restores through /data/restore. Keep it private — it holds everything \
                   the account knows.",
    responses((status = 200, description = "A zip archive", content_type = "application/zip"))
)]
pub async fn handle_backup(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    let deps = write_backup::Deps {
        entries: state.entry_query,
        dimensions: state.dimensions,
        activities: state.activity_query,
        reminders: state.reminder_query,
        metrics: state.daily_metric_query,
        cycles: state.cycle_query,
        preferences: state.preferences_query,
        media_storage: state.media_storage,
        writer: state.backup_writer,
    };

    let archive = write_backup::execute(user_id, &deps).await?;

    Ok(attachment("application/zip", BACKUP_FILENAME, archive))
}

#[utoipa::path(get, path = "/api/v1/data/extract", tag = "data", security(("bearer" = [])),
    description = "A shareable journal: the mood, what was written and what was tagged, as a \
                   readable markdown document. It carries no places, no health readings, no \
                   cycle records and no media, and it cannot be restored from. This is the one \
                   to hand to someone.",
    responses((status = 200, description = "A markdown document", content_type = "text/markdown"))
)]
pub async fn handle_extract(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    let deps = write_extract::Deps {
        entries: state.entry_query,
        dimensions: state.dimensions,
        activities: state.activity_query,
        writer: state.extract_writer,
    };

    let document = write_extract::execute(user_id, &deps).await?;

    Ok(attachment(
        "text/markdown; charset=utf-8",
        EXTRACT_FILENAME,
        document,
    ))
}

#[utoipa::path(post, path = "/api/v1/data/restore", tag = "data", security(("bearer" = [])),
    description = "Restores a complete backup into this account. Existing data is kept: a \
                   restore adds, it does not replace. Anything in the archive this build cannot \
                   read is reported rather than silently dropped.",
    responses((status = 200, body = RestoreOutcomeResponse))
)]
pub async fn handle_restore(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    multipart: Multipart,
) -> Result<Json<RestoreOutcomeResponse>, ApiError> {
    let data = extract_file_bytes(multipart).await?;

    let deps = restore_backup::Deps {
        reader: state.backup_reader,
        entry_command: state.entry_command,
        dimensions: state.dimensions,
        activity_command: state.activity_command,
        activity_query: state.activity_query,
        reminder_command: state.reminder_command,
        metrics: state.daily_metric_command,
        cycles: state.cycle_command,
        preferences_command: state.preferences_command,
        preferences_query: state.preferences_query,
        media_storage: state.media_storage,
    };

    let outcome = restore_backup::execute(RestoreBackupCommand { user_id, data }, &deps).await?;

    Ok(Json(outcome.into()))
}

fn attachment(content_type: &str, filename: &str, body: Vec<u8>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type.to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        body,
    )
}
