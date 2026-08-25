use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::{CreateReminderRequest, UpdateReminderRequest};
use api_types::responses::ReminderResponse;
use application::reminder::use_cases::{
    create_reminder, delete_reminder, get_reminder, list_reminders, update_reminder,
};
use domain::reminder::ReminderId;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, PathId};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/reminders", tag = "reminders", security(("bearer" = [])),
    request_body = CreateReminderRequest,
    responses((status = 201, body = ReminderResponse))
)]
pub async fn handle_create(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<CreateReminderRequest>,
) -> Result<(StatusCode, Json<ReminderResponse>), ApiError> {
    let cmd = body.into_command(user_id)?;
    let deps = create_reminder::Deps {
        reminders: state.reminder_command,
        events: state.event_publisher,
    };
    let reminder = create_reminder::execute(cmd, &deps).await?;
    Ok((StatusCode::CREATED, Json(ReminderResponse::from(reminder))))
}

#[utoipa::path(get, path = "/api/v1/reminders/{id}", tag = "reminders", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 200, body = ReminderResponse))
)]
pub async fn handle_get(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(reminder_id): PathId<ReminderId>,
) -> Result<Json<ReminderResponse>, ApiError> {
    let deps = get_reminder::Deps {
        query: state.reminder_query,
    };
    let reminder = get_reminder::execute(reminder_id, user_id, &deps).await?;
    Ok(Json(ReminderResponse::from(reminder)))
}

#[utoipa::path(get, path = "/api/v1/reminders", tag = "reminders", security(("bearer" = [])),
    responses((status = 200, body = Vec<ReminderResponse>))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<Vec<ReminderResponse>>, ApiError> {
    let deps = list_reminders::Deps {
        query: state.reminder_query,
    };
    let reminders = list_reminders::execute(user_id, &deps).await?;
    Ok(Json(
        reminders.into_iter().map(ReminderResponse::from).collect(),
    ))
}

#[utoipa::path(patch, path = "/api/v1/reminders/{id}", tag = "reminders", security(("bearer" = [])),
    params(("id" = String, Path)),
    request_body = UpdateReminderRequest,
    responses((status = 200, body = ReminderResponse))
)]
pub async fn handle_update(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(reminder_id): PathId<ReminderId>,
    Json(body): Json<UpdateReminderRequest>,
) -> Result<Json<ReminderResponse>, ApiError> {
    let cmd = body.into_command(reminder_id)?;
    let deps = update_reminder::Deps {
        command: state.reminder_command,
        query: state.reminder_query,
        events: state.event_publisher,
    };
    let reminder = update_reminder::execute(cmd, user_id, &deps).await?;
    Ok(Json(ReminderResponse::from(reminder)))
}

#[utoipa::path(delete, path = "/api/v1/reminders/{id}", tag = "reminders", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 204))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(reminder_id): PathId<ReminderId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_reminder::Deps {
        command: state.reminder_command,
        query: state.reminder_query,
        events: state.event_publisher,
    };
    delete_reminder::execute(reminder_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
