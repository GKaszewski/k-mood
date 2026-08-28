use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct PathId<T>(pub T);

pub struct PathIdRejection(String);

impl IntoResponse for PathIdRejection {
    fn into_response(self) -> Response {
        crate::errors::refuse(
            StatusCode::BAD_REQUEST,
            crate::errors::INVALID_INPUT,
            self.0,
        )
    }
}

impl<S, T> axum::extract::FromRequestParts<S> for PathId<T>
where
    S: Send + Sync,
    T: From<uuid::Uuid>,
{
    type Rejection = PathIdRejection;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Path(id_str) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|e| PathIdRejection(format!("invalid path parameter: {e}")))?;

        let uuid: uuid::Uuid = id_str
            .parse()
            .map_err(|_| PathIdRejection(format!("invalid UUID: {id_str}")))?;

        Ok(PathId(T::from(uuid)))
    }
}
