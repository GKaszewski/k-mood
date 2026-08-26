use axum::Json;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};

use domain::user::UserId;

use crate::state::AppState;

pub struct AuthenticatedUser(pub UserId);

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        let body = serde_json::json!({ "error": self.0 });
        (StatusCode::UNAUTHORIZED, Json(body)).into_response()
    }
}

pub struct AuthRejection(String);

impl AuthRejection {
    pub fn new(reason: impl Into<String>) -> Self {
        Self(reason.into())
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = bearer_token(parts)
            .ok_or_else(|| AuthRejection("missing or invalid authorization header".into()))?;

        let user_id = app_state
            .auth_service
            .validate_token(&token)
            .await
            .map_err(|_| AuthRejection("invalid or expired token".into()))?;

        Ok(AuthenticatedUser(user_id))
    }
}

pub fn bearer_token(parts: &Parts) -> Option<String> {
    let header = parts.headers.get("authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    Some(token.to_string())
}

use axum::extract::FromRef;
