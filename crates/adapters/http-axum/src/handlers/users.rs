use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::{ChangePasswordRequest, RegisterRequest, UpdateProfileRequest};
use api_types::responses::UserResponse;
use application::user::use_cases::{
    change_password, clear_data, delete_user, get_profile, register, update_profile,
};

use crate::errors::ApiError;
use crate::extractors::AuthenticatedUser;
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/users/register", tag = "users",
    request_body = RegisterRequest,
    responses((status = 201, body = UserResponse))
)]
pub async fn handle_register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
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
    responses((status = 200, body = UserResponse))
)]
pub async fn handle_get_profile(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<UserResponse>, ApiError> {
    let deps = get_profile::Deps {
        user_query: state.user_query,
    };
    let user = get_profile::execute(user_id, &deps).await?;
    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(patch, path = "/api/v1/users/me", tag = "users", security(("bearer" = [])),
    request_body = UpdateProfileRequest,
    responses((status = 200, body = UserResponse))
)]
pub async fn handle_update_profile(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<UpdateProfileRequest>,
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
    responses((status = 204))
)]
pub async fn handle_change_password(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command(user_id);
    let deps = change_password::Deps {
        user_command: state.user_command,
        user_query: state.user_query,
        password_hasher: state.password_hasher,
    };
    change_password::execute(cmd, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/users/me", tag = "users", security(("bearer" = [])),
    responses((status = 204))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<StatusCode, ApiError> {
    let deps = delete_user::Deps {
        user_query: state.user_query,
        entry_query: state.entry_query,
        cascade: state.cascade,
        media_storage: state.media_storage,
        events: state.event_publisher,
    };
    delete_user::execute(user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/users/me/data", tag = "users", security(("bearer" = [])),
    responses((status = 204, description = "All user data cleared"))
)]
pub async fn handle_clear_data(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<StatusCode, ApiError> {
    let deps = clear_data::Deps {
        entry_query: state.entry_query,
        cascade: state.cascade,
        media_storage: state.media_storage,
    };
    clear_data::execute(user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
