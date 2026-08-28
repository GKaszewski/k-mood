use axum::Json;
use axum::extract::State;

use api_types::responses::{
    ErrorResponse, MetricKindResponse, MoodResponse, ServerInfoResponse, ServerLimitsResponse,
    TokenScopeResponse,
};
use domain::api_token::TokenScope;
use domain::entry::Mood;
use domain::metric::MetricKind;
use domain::weather::Condition;

use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/server", tag = "server",
    description = "What this deployment is and what it allows, for a client deciding what to \
                   offer before anyone has signed in. Carries no secrets and needs no \
                   authentication. The vocabularies are the ones the API validates against, so \
                   a client can build its forms from here rather than hardcoding them.",
    responses((status = 200, body = ServerInfoResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_info(State(state): State<AppState>) -> Json<ServerInfoResponse> {
    Json(ServerInfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        registration_open: state.auth_config.allow_registration,
        push_enabled: state.push_config.enabled,
        weather_enabled: state.weather_enabled,
        now_playing_provider: state
            .now_playing
            .as_ref()
            .map(|playing| playing.provider().to_string()),
        access_token_ttl_seconds: state.auth_config.access_token_ttl_seconds,
        limits: ServerLimitsResponse {
            max_body_size: state.server_config.max_body_size,
            max_content_length: state.entry_config.max_content_length,
            max_photos: state.entry_config.max_photos,
            max_voice_memos: state.entry_config.max_voice_memos,
            max_activities_per_entry: state.entry_config.max_activities_per_entry,
            max_entries_per_page: state.entry_config.max_entries_per_page,
            max_import_days: state.import_config.maximum_days_per_import,
        },
        token_scopes: TokenScope::ALL
            .into_iter()
            .map(|scope| TokenScopeResponse {
                name: scope.name().to_string(),
                describes: scope.describes().to_string(),
            })
            .collect(),
        metric_kinds: MetricKind::ALL
            .into_iter()
            .map(|kind| {
                let (minimum, maximum) = kind.bounds();

                MetricKindResponse {
                    name: kind.name().to_string(),
                    unit: kind.unit().to_string(),
                    minimum,
                    maximum,
                }
            })
            .collect(),
        moods: Mood::ALL
            .into_iter()
            .map(|mood| MoodResponse {
                value: mood.value(),
                label: mood.label().to_string(),
            })
            .collect(),
        weather_conditions: Condition::ALL
            .into_iter()
            .map(|condition| condition.name().to_string())
            .collect(),
    })
}
