use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::LoginRequest;
use api_types::responses::{ErrorResponse, RefreshedResponse, SignedInResponse, UserResponse};
use application::auth::use_cases::{login, logout, refresh};

use crate::errors::ApiError;
use crate::extractors::Body;
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/auth/login", tag = "auth",
    request_body = LoginRequest,
    responses((status = 200, body = SignedInResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_login(
    State(state): State<AppState>,
    Body(body): Body<LoginRequest>,
) -> Result<Json<SignedInResponse>, ApiError> {
    let cmd = body.into_command();
    let deps = login::Deps {
        user_query: state.user_query,
        password_hasher: state.password_hasher,
        auth_service: state.auth_service,
        refresh_session_command: state.refresh_session_command,
        refresh_token_ttl_seconds: state.auth_config.refresh_token_ttl_seconds as i64,
    };

    let result = login::execute(cmd, &deps).await?;

    Ok(Json(SignedInResponse {
        access_token: result.access_token.token().to_string(),
        refresh_token: result.refresh_token,
        expires_at: *result.access_token.expires_at(),
        user: UserResponse::from(result.user),
    }))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", tag = "auth",
    request_body = RefreshRequest,
    responses((status = 200, body = RefreshedResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_refresh(
    State(state): State<AppState>,
    Body(body): Body<RefreshRequest>,
) -> Result<Json<RefreshedResponse>, ApiError> {
    let deps = refresh::Deps {
        auth_service: state.auth_service,
        refresh_session_command: state.refresh_session_command,
        refresh_session_query: state.refresh_session_query,
        refresh_token_ttl_seconds: state.auth_config.refresh_token_ttl_seconds as i64,
    };

    let result = refresh::execute(&body.refresh_token, &deps).await?;

    Ok(Json(RefreshedResponse {
        access_token: result.access_token.token().to_string(),
        refresh_token: result.refresh_token,
        expires_at: *result.access_token.expires_at(),
    }))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[utoipa::path(post, path = "/api/v1/auth/logout", tag = "auth",
    request_body = LogoutRequest,
    responses((status = 204, description = "Logged out"),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_logout(
    State(state): State<AppState>,
    Body(body): Body<LogoutRequest>,
) -> Result<StatusCode, ApiError> {
    let deps = logout::Deps {
        refresh_session_command: state.refresh_session_command,
    };
    logout::execute(&body.refresh_token, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
