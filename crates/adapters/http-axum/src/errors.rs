use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use api_types::errors::ApiValidationError;
use api_types::responses::ErrorResponse;
use application::errors::ApplicationError;
use domain::errors::DomainError;

pub const NOT_FOUND: &str = "NOT_FOUND";
pub const INVALID_INPUT: &str = "INVALID_INPUT";
pub const CONFLICT: &str = "CONFLICT";
pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
pub const FORBIDDEN: &str = "FORBIDDEN";
pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
pub const TOO_MANY_REQUESTS: &str = "TOO_MANY_REQUESTS";
pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";

pub struct ApiError(pub ApplicationError);

pub fn refuse(status: StatusCode, code: &str, message: impl Into<String>) -> Response {
    (status, Json(ErrorResponse::new(code, message))).into_response()
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self.0 {
            ApplicationError::Domain(domain_err) => domain_error_response(domain_err),
            ApplicationError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                VALIDATION_ERROR,
                msg.clone(),
            ),
        };

        refuse(status, code, message)
    }
}

impl From<ApplicationError> for ApiError {
    fn from(err: ApplicationError) -> Self {
        Self(err)
    }
}

impl From<ApiValidationError> for ApiError {
    fn from(err: ApiValidationError) -> Self {
        match err {
            ApiValidationError::Domain(e) => Self(ApplicationError::Domain(e)),
            ApiValidationError::Invalid(msg) => Self(ApplicationError::Validation(msg)),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        Self(ApplicationError::Domain(err))
    }
}

fn domain_error_response(err: &DomainError) -> (StatusCode, &'static str, String) {
    match err {
        DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, NOT_FOUND, msg.clone()),
        DomainError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, INVALID_INPUT, msg.clone()),
        DomainError::Conflict(msg) => (StatusCode::CONFLICT, CONFLICT, msg.clone()),
        DomainError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, UNAUTHORIZED, msg.clone()),
        DomainError::Forbidden(msg) => (StatusCode::FORBIDDEN, FORBIDDEN, msg.clone()),
    }
}
