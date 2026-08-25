use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::{CreateActivityRequest, RenameActivityRequest, SetCategoryRequest};
use api_types::responses::ActivityResponse;
use application::activity::use_cases::{
    archive_activity, create_activity, delete_activity, get_activity, list_activities,
    rename_activity, set_category,
};
use domain::activity::ActivityId;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, PathId};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/activities", tag = "activities", security(("bearer" = [])),
    request_body = CreateActivityRequest,
    responses((status = 201, body = ActivityResponse))
)]
pub async fn handle_create(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<CreateActivityRequest>,
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
    responses((status = 200, body = ActivityResponse))
)]
pub async fn handle_get(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<Json<ActivityResponse>, ApiError> {
    let deps = get_activity::Deps {
        query: state.activity_query,
    };
    let activity = get_activity::execute(activity_id, user_id, &deps).await?;
    Ok(Json(ActivityResponse::from(activity)))
}

#[utoipa::path(get, path = "/api/v1/activities", tag = "activities", security(("bearer" = [])),
    responses((status = 200, body = Vec<ActivityResponse>))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<Vec<ActivityResponse>>, ApiError> {
    let deps = list_activities::Deps {
        query: state.activity_query,
    };
    let activities = list_activities::active_only(user_id, &deps).await?;
    Ok(Json(
        activities.into_iter().map(ActivityResponse::from).collect(),
    ))
}

#[utoipa::path(patch, path = "/api/v1/activities/{id}/name", tag = "activities", security(("bearer" = [])),
    params(("id" = String, Path)),
    request_body = RenameActivityRequest,
    responses((status = 204))
)]
pub async fn handle_rename(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(activity_id): PathId<ActivityId>,
    Json(body): Json<RenameActivityRequest>,
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
    responses((status = 204))
)]
pub async fn handle_set_category(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(activity_id): PathId<ActivityId>,
    Json(body): Json<SetCategoryRequest>,
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
    responses((status = 204))
)]
pub async fn handle_archive(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
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
    responses((status = 204))
)]
pub async fn handle_unarchive(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
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
    responses((status = 204))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_activity::Deps {
        command: state.activity_command,
        query: state.activity_query,
    };
    delete_activity::execute(activity_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
