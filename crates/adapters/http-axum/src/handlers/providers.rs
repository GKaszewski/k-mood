use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use api_types::dimension::DimensionPayload;
use api_types::requests::ConnectProviderRequest;
use api_types::responses::{ErrorResponse, ProviderConnectionResponse};
use application::provider::commands::ConnectProviderCommand;
use application::provider::use_cases::{
    connect_provider, disconnect_provider, get_now_playing, list_connections,
};
use domain::dimension::DimensionValue;
use domain::errors::DomainError;
use domain::provider::ProviderName;

use crate::errors::ApiError;
use crate::extractors::{Body, JournalReader, ProfileReader, SessionUser};
use crate::state::AppState;

fn cipher(
    state: &AppState,
) -> Result<std::sync::Arc<dyn domain::provider::CredentialCipher>, ApiError> {
    state.credential_cipher.clone().ok_or_else(|| {
        DomainError::InvalidInput(
            "provider connections are unavailable: no credential encryption key is configured"
                .into(),
        )
        .into()
    })
}

#[utoipa::path(get, path = "/api/v1/providers", tag = "providers", security(("bearer" = [])),
    responses((status = 200, body = Vec<ProviderConnectionResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    ProfileReader(user_id): ProfileReader,
) -> Result<Json<Vec<ProviderConnectionResponse>>, ApiError> {
    let deps = list_connections::Deps {
        query: state.provider_connection_query,
    };
    let connections = list_connections::execute(user_id, &deps).await?;

    Ok(Json(connections.into_iter().map(Into::into).collect()))
}

#[utoipa::path(put, path = "/api/v1/providers/{provider}", tag = "providers", security(("bearer" = [])),
    params(("provider" = String, Path, description = "Provider name")),
    request_body = ConnectProviderRequest,
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_connect(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Path(provider): Path<String>,
    Body(body): Body<ConnectProviderRequest>,
) -> Result<StatusCode, ApiError> {
    let cipher = cipher(&state)?;
    let provider = ProviderName::new(provider)?;

    let credential = serde_json::to_vec(&body.credential)
        .map_err(|_| DomainError::InvalidInput("credential must be a JSON object".into()))?;

    let deps = connect_provider::Deps {
        command: state.provider_connection_command,
        cipher,
    };

    connect_provider::execute(
        ConnectProviderCommand {
            user_id,
            provider,
            credential,
        },
        &deps,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/providers/{provider}", tag = "providers", security(("bearer" = [])),
    params(("provider" = String, Path, description = "Provider name")),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_disconnect(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Path(provider): Path<String>,
) -> Result<StatusCode, ApiError> {
    let provider = ProviderName::new(provider)?;

    let deps = disconnect_provider::Deps {
        command: state.provider_connection_command,
    };

    disconnect_provider::execute(user_id, provider, &deps).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/providers/now-playing", tag = "providers", security(("bearer" = [])),
    responses((status = 200, body = Option<DimensionPayload>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_now_playing(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
) -> Result<Json<Option<DimensionPayload>>, ApiError> {
    let cipher = cipher(&state)?;

    let now_playing = state.now_playing.clone().ok_or_else(|| -> ApiError {
        DomainError::InvalidInput("no music provider is configured".into()).into()
    })?;

    let deps = get_now_playing::Deps {
        query: state.provider_connection_query,
        cipher,
        now_playing,
        recordings: state.recording_lookup,
    };

    let song = get_now_playing::execute(user_id, &deps).await?;

    Ok(Json(song.map(|song| {
        DimensionPayload::from(&DimensionValue::Song(song))
    })))
}
