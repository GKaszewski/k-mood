use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::params::ListActivitiesParams;
use api_types::requests::{CreateActivityRequest, RenameActivityRequest, SetCategoryRequest};
use api_types::responses::{ActivityResponse, ErrorResponse};
use application::activity::use_cases::{
    archive_activity, create_activity, delete_activity, get_activity, list_activities,
    rename_activity, set_category,
};
use domain::activity::ActivityId;

use crate::errors::ApiError;
use crate::extractors::{Body, JournalReader, JournalWriter, Params, PathId};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/activities", tag = "activities", security(("bearer" = [])),
    request_body = CreateActivityRequest,
    responses((status = 201, body = ActivityResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_create(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    Body(body): Body<CreateActivityRequest>,
) -> Result<(StatusCode, Json<ActivityResponse>), ApiError> {
    let cmd = body.into_command(user_id)?;
    let deps = create_activity::Deps {
        activities: state.activity_command,
        events: state.event_publisher,
    };
    let activity = create_activity::execute(cmd, &deps).await?;
    Ok((StatusCode::CREATED, Json(ActivityResponse::from(activity))))
}

#[utoipa::path(get, path = "/api/v1/activities/{id}", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 200, body = ActivityResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_get(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<Json<ActivityResponse>, ApiError> {
    let deps = get_activity::Deps {
        query: state.activity_query,
    };
    let activity = get_activity::execute(activity_id, user_id, &deps).await?;
    Ok(Json(ActivityResponse::from(activity)))
}

#[utoipa::path(get, path = "/api/v1/activities", tag = "activities", security(("bearer" = [])),
    description = "This account's activity catalogue. Archived activities are left out unless \
                   `includeArchived` asks for them, because they cannot be tagged onto anything \
                   new — but they stay listable, so a client can show them and unarchive one.",
    params(ListActivitiesParams),
    responses((status = 200, body = Vec<ActivityResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    Params(params): Params<ListActivitiesParams>,
) -> Result<Json<Vec<ActivityResponse>>, ApiError> {
    let deps = list_activities::Deps {
        query: state.activity_query,
    };
    let activities = match params.include_archived.unwrap_or(false) {
        true => list_activities::all(user_id, &deps).await?,
        false => list_activities::active_only(user_id, &deps).await?,
    };
    Ok(Json(
        activities.into_iter().map(ActivityResponse::from).collect(),
    ))
}

#[utoipa::path(patch, path = "/api/v1/activities/{id}/name", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    request_body = RenameActivityRequest,
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_rename(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(activity_id): PathId<ActivityId>,
    Body(body): Body<RenameActivityRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command(activity_id)?;
    let deps = rename_activity::Deps {
        command: state.activity_command,
        query: state.activity_query,
        events: state.event_publisher,
    };
    rename_activity::execute(cmd, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(patch, path = "/api/v1/activities/{id}/category", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    request_body = SetCategoryRequest,
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_set_category(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(activity_id): PathId<ActivityId>,
    Body(body): Body<SetCategoryRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command(activity_id)?;
    let deps = set_category::Deps {
        command: state.activity_command,
        query: state.activity_query,
    };
    set_category::execute(cmd, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/v1/activities/{id}/archive", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_archive(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<StatusCode, ApiError> {
    let deps = archive_activity::Deps {
        command: state.activity_command,
        query: state.activity_query,
        events: state.event_publisher,
    };
    archive_activity::archive(activity_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/v1/activities/{id}/unarchive", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_unarchive(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<StatusCode, ApiError> {
    let deps = archive_activity::Deps {
        command: state.activity_command,
        query: state.activity_query,
        events: state.event_publisher,
    };
    archive_activity::unarchive(activity_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/activities/{id}", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_activity::Deps {
        entries: state.entry_query.clone(),
        command: state.activity_command,
        query: state.activity_query,
    };
    delete_activity::execute(activity_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
