use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::LoginRequest;
use api_types::responses::UserResponse;
use application::auth::use_cases::{login, logout, refresh};

use crate::errors::ApiError;
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/auth/login", tag = "auth",
    request_body = LoginRequest,
    responses((status = 200, description = "Login successful"))
)]
pub async fn handle_login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let cmd = body.into_command();
    let deps = login::Deps {
        user_query: state.user_query,
        password_hasher: state.password_hasher,
        auth_service: state.auth_service,
        refresh_session_command: state.refresh_session_command,
        refresh_token_ttl_seconds: state.auth_config.refresh_token_ttl_seconds as i64,
    };

    let result = login::execute(cmd, &deps).await?;
    let user_response = UserResponse::from(result.user);

    Ok(Json(serde_json::json!({
        "accessToken": result.access_token.token(),
        "refreshToken": result.refresh_token,
        "expiresAt": result.access_token.expires_at(),
        "user": user_response,
    })))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", tag = "auth",
    request_body = RefreshRequest,
    responses((status = 200, description = "Token refreshed"))
)]
pub async fn handle_refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let deps = refresh::Deps {
        auth_service: state.auth_service,
        refresh_session_command: state.refresh_session_command,
        refresh_session_query: state.refresh_session_query,
        refresh_token_ttl_seconds: state.auth_config.refresh_token_ttl_seconds as i64,
    };

    let result = refresh::execute(&body.refresh_token, &deps).await?;

    Ok(Json(serde_json::json!({
        "accessToken": result.access_token.token(),
        "refreshToken": result.refresh_token,
        "expiresAt": result.access_token.expires_at(),
    })))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[utoipa::path(post, path = "/api/v1/auth/logout", tag = "auth",
    request_body = LogoutRequest,
    responses((status = 204, description = "Logged out"))
)]
pub async fn handle_logout(
    State(state): State<AppState>,
    Json(body): Json<LogoutRequest>,
) -> Result<StatusCode, ApiError> {
    let deps = logout::Deps {
        refresh_session_command: state.refresh_session_command,
    };
    logout::execute(&body.refresh_token, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
