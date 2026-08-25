use axum::Json;
use axum::extract::{Multipart, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use api_types::responses::MediaIdResponse;
use application::media::use_cases::{
    delete_photo, delete_voice_memo, upload_photo, upload_voice_memo,
};
use domain::attachment::{PhotoId, VoiceMemoId};
use domain::errors::DomainError;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, PathId, extract_media_upload};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/media/photos", tag = "media", security(("bearer" = [])),
    responses((status = 201, body = MediaIdResponse))
)]
pub async fn handle_upload_photo(
    State(state): State<AppState>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    multipart: Multipart,
) -> Result<(StatusCode, Json<MediaIdResponse>), ApiError> {
    let upload = extract_media_upload(multipart).await?;
    let deps = upload_photo::Deps {
        storage: state.media_storage,
    };
    let photo_id = upload_photo::execute(upload, &deps).await?;
    Ok((StatusCode::CREATED, Json(MediaIdResponse::from(photo_id))))
}

#[utoipa::path(post, path = "/api/v1/media/voice-memos", tag = "media", security(("bearer" = [])),
    responses((status = 201, body = MediaIdResponse))
)]
pub async fn handle_upload_voice_memo(
    State(state): State<AppState>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    multipart: Multipart,
) -> Result<(StatusCode, Json<MediaIdResponse>), ApiError> {
    let upload = extract_media_upload(multipart).await?;
    let deps = upload_voice_memo::Deps {
        storage: state.media_storage,
    };
    let voice_memo_id = upload_voice_memo::execute(upload, &deps).await?;
    Ok((
        StatusCode::CREATED,
        Json(MediaIdResponse::from(voice_memo_id)),
    ))
}

#[utoipa::path(get, path = "/api/v1/media/photos/{id}", tag = "media",
    params(("id" = String, Path)),
    responses((status = 200, description = "Photo binary"))
)]
pub async fn handle_serve_photo(
    State(state): State<AppState>,
    PathId(photo_id): PathId<PhotoId>,
) -> Result<impl IntoResponse, ApiError> {
    let file = state
        .media_storage
        .get_photo(&photo_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("photo not found".into()))?;

    Ok((
        [(header::CONTENT_TYPE, file.content_type.value().to_string())],
        file.data,
    ))
}

#[utoipa::path(get, path = "/api/v1/media/voice-memos/{id}", tag = "media",
    params(("id" = String, Path)),
    responses((status = 200, description = "Voice memo binary"))
)]
pub async fn handle_serve_voice_memo(
    State(state): State<AppState>,
    PathId(voice_memo_id): PathId<VoiceMemoId>,
) -> Result<impl IntoResponse, ApiError> {
    let file = state
        .media_storage
        .get_voice_memo(&voice_memo_id)
        .await?
        .ok_or_else(|| DomainError::NotFound("voice memo not found".into()))?;

    Ok((
        [(header::CONTENT_TYPE, file.content_type.value().to_string())],
        file.data,
    ))
}

#[utoipa::path(delete, path = "/api/v1/media/photos/{id}", tag = "media", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 204))
)]
pub async fn handle_delete_photo(
    State(state): State<AppState>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    PathId(photo_id): PathId<PhotoId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_photo::Deps {
        storage: state.media_storage,
    };
    delete_photo::execute(photo_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/media/voice-memos/{id}", tag = "media", security(("bearer" = [])),
    params(("id" = String, Path)),
    responses((status = 204))
)]
pub async fn handle_delete_voice_memo(
    State(state): State<AppState>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    PathId(voice_memo_id): PathId<VoiceMemoId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_voice_memo::Deps {
        storage: state.media_storage,
    };
    delete_voice_memo::execute(voice_memo_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}
