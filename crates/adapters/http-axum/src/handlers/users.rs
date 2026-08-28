use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::{ChangePasswordRequest, RegisterRequest, UpdateProfileRequest};
use api_types::responses::{ErrorResponse, UserResponse};
use application::user::use_cases::{
    change_password, clear_data, delete_user, get_profile, register, update_profile,
};

use crate::errors::ApiError;
use crate::extractors::{Body, ProfileReader, SessionUser};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/users/register", tag = "users",
    request_body = RegisterRequest,
    responses((status = 201, body = UserResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_register(
    State(state): State<AppState>,
    Body(body): Body<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    if !state.auth_config.allow_registration {
        return Err(
            domain::errors::DomainError::Forbidden("registration is disabled".into()).into(),
        );
    }
    let cmd = body.into_command()?;
    let deps = register::Deps {
        user_command: state.user_command,
        user_query: state.user_query,
        activity_command: state.activity_command,
        password_hasher: state.password_hasher,
        events: state.event_publisher,
        preset: state.preset_config,
    };
    let user = register::execute(cmd, &deps).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

#[utoipa::path(get, path = "/api/v1/users/me", tag = "users", security(("bearer" = [])),
    responses((status = 200, body = UserResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_get_profile(
    State(state): State<AppState>,
    ProfileReader(user_id): ProfileReader,
) -> Result<Json<UserResponse>, ApiError> {
    let deps = get_profile::Deps {
        user_query: state.user_query,
    };
    let user = get_profile::execute(user_id, &deps).await?;
    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(patch, path = "/api/v1/users/me", tag = "users", security(("bearer" = [])),
    request_body = UpdateProfileRequest,
    responses((status = 200, body = UserResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_update_profile(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Body(body): Body<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let cmd = body.into_command(user_id)?;
    let deps = update_profile::Deps {
        user_command: state.user_command,
        user_query: state.user_query,
    };
    let user = update_profile::execute(cmd, &deps).await?;
    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(patch, path = "/api/v1/users/me/password", tag = "users", security(("bearer" = [])),
    request_body = ChangePasswordRequest,
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_change_password(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Body(body): Body<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command(user_id);
    let deps = change_password::Deps {
        refresh_sessions: state.refresh_session_command.clone(),
        user_command: state.user_command,
        user_query: state.user_query,
        password_hasher: state.password_hasher,
    };
    change_password::execute(cmd, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/users/me", tag = "users", security(("bearer" = [])),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
) -> Result<StatusCode, ApiError> {
    let deps = delete_user::Deps {
        user_query: state.user_query,
        cascade: state.cascade,
        media_storage: state.media_storage,
        media_ownership: state.media_ownership,
        events: state.event_publisher,
    };
    delete_user::execute(user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/users/me/data", tag = "users", security(("bearer" = [])),
    responses((status = 204, description = "All user data cleared"),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_clear_data(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
) -> Result<StatusCode, ApiError> {
    let deps = clear_data::Deps {
        cascade: state.cascade,
        media_storage: state.media_storage,
        media_ownership: state.media_ownership,
    };
    clear_data::execute(user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
