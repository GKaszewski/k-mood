use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};

use application::errors::ApplicationError;
use domain::api_token::{ApiToken, TokenScope};
use domain::errors::DomainError;
use domain::metric::Source;
use domain::user::UserId;

use application::api_token::use_cases::authenticate_api_token;

use crate::errors::{FORBIDDEN, UNAUTHORIZED, refuse};
use crate::state::AppState;

pub struct AuthRejection {
    status: StatusCode,
    code: &'static str,
    reason: String,
}

impl AuthRejection {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: UNAUTHORIZED,
            reason: reason.into(),
        }
    }

    pub fn forbidding(reason: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: FORBIDDEN,
            reason: reason.into(),
        }
    }
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        refuse(self.status, self.code, self.reason)
    }
}

pub enum Bearer {
    Session(UserId),
    Token(ApiToken),
}

impl Bearer {
    pub fn user_id(&self) -> &UserId {
        match self {
            Self::Session(user_id) => user_id,
            Self::Token(token) => token.user_id(),
        }
    }

    pub fn into_user_id(self) -> UserId {
        match self {
            Self::Session(user_id) => user_id,
            Self::Token(token) => token.user_id().clone(),
        }
    }

    pub fn attributes_writes_to(&self) -> Source {
        match self {
            Self::Session(_) => Source::Manual,
            Self::Token(token) => Source::Provider(token.name().clone()),
        }
    }
}

pub fn bearer_token(parts: &Parts) -> Option<String> {
    let header = parts.headers.get("authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;

    Some(token.to_string())
}

pub async fn session_only(parts: &Parts, state: &AppState) -> Result<UserId, AuthRejection> {
    let presented = bearer_token(parts)
        .ok_or_else(|| AuthRejection::new("missing or invalid authorization header"))?;

    state
        .auth_service
        .validate_token(&presented)
        .await
        .map_err(|_| {
            AuthRejection::new("this endpoint needs a signed-in session, not an api token")
        })
}

pub async fn administrator(parts: &Parts, state: &AppState) -> Result<UserId, AuthRejection> {
    let user_id = session_only(parts, state).await?;

    let user = state
        .user_query
        .find_by_id(&user_id)
        .await
        .map_err(|_| AuthRejection::new("this account could not be read"))?
        .ok_or_else(|| AuthRejection::new("this account no longer exists"))?;

    if !user.is_admin() {
        return Err(AuthRejection::forbidding(
            "this endpoint is for whoever runs the server",
        ));
    }

    Ok(user_id)
}

pub async fn session_or_token_granting(
    parts: &Parts,
    state: &AppState,
    needed: TokenScope,
) -> Result<Bearer, AuthRejection> {
    let presented = bearer_token(parts)
        .ok_or_else(|| AuthRejection::new("missing or invalid authorization header"))?;

    if let Ok(user_id) = state.auth_service.validate_token(&presented).await {
        return Ok(Bearer::Session(user_id));
    }

    let deps = authenticate_api_token::Deps {
        query: state.api_token_query.clone(),
        command: state.api_token_command.clone(),
        secrets: state.api_token_secrets.clone(),
    };

    let token = authenticate_api_token::execute(&presented, needed, &deps)
        .await
        .map_err(refusal_for)?;

    Ok(Bearer::Token(token))
}

fn refusal_for(error: ApplicationError) -> AuthRejection {
    match error {
        ApplicationError::Domain(DomainError::Forbidden(reason)) => {
            AuthRejection::forbidding(reason)
        }
        other => AuthRejection::new(other.to_string()),
    }
}
