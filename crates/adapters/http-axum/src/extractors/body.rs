use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{FromRequest, FromRequestParts, Query, Request};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};

use crate::errors::{INVALID_INPUT, refuse};

pub struct Body<T>(pub T);

pub struct Params<T>(pub T);

pub struct MalformedRequest(String);

impl IntoResponse for MalformedRequest {
    fn into_response(self) -> Response {
        refuse(StatusCode::BAD_REQUEST, INVALID_INPUT, self.0)
    }
}

impl From<JsonRejection> for MalformedRequest {
    fn from(rejection: JsonRejection) -> Self {
        Self(rejection.body_text())
    }
}

impl From<QueryRejection> for MalformedRequest {
    fn from(rejection: QueryRejection) -> Self {
        Self(rejection.body_text())
    }
}

impl<S, T> FromRequest<S> for Body<T>
where
    S: Send + Sync,
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = MalformedRequest;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(request, state).await?;

        Ok(Self(value))
    }
}

impl<S, T> FromRequestParts<S> for Params<T>
where
    S: Send + Sync,
    Query<T>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = MalformedRequest;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state).await?;

        Ok(Self(value))
    }
}
