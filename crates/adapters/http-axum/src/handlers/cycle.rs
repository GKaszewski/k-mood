use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use api_types::mappers::parse_date;
use api_types::requests::SetPreferencesRequest;
use api_types::responses::{CycleViewResponse, PreferencesResponse};
use application::cycle::use_cases::{forget_cycle_start, read_cycle, record_cycle_start};
use application::user::use_cases::set_preferences;

use crate::errors::ApiError;
use crate::extractors::AuthenticatedUser;
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/cycle", tag = "cycle", security(("bearer" = [])),
    description = "The recorded cycle starts and the cycle day derived for today. Cycle day is \
                   never stored: correcting a start corrects every day that depended on it. \
                   Returns nothing at all while cycle tracking is off.",
    responses((status = 200, body = CycleViewResponse))
)]
pub async fn handle_read(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<CycleViewResponse>, ApiError> {
    let deps = read_cycle::Deps {
        query: state.cycle_query,
        preferences: state.preferences_query,
        users: state.user_query,
    };

    let view = read_cycle::execute(user_id, &deps).await?;

    Ok(Json(view.into()))
}

#[utoipa::path(put, path = "/api/v1/cycle/{date}", tag = "cycle", security(("bearer" = [])),
    description = "Records that a cycle began on this date. Recording the same date twice \
                   records it once.",
    params(("date" = String, Path, description = "Calendar date, as YYYY-MM-DD")),
    responses((status = 204))
)]
pub async fn handle_record(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(date): Path<String>,
) -> Result<StatusCode, ApiError> {
    let deps = record_cycle_start::Deps {
        command: state.cycle_command,
        preferences: state.preferences_query,
    };

    record_cycle_start::execute(user_id, parse_date(&date)?, &deps).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/api/v1/cycle/{date}", tag = "cycle", security(("bearer" = [])),
    description = "Forgets a recorded start. Every day that derived its cycle day from it \
                   changes at once.",
    params(("date" = String, Path, description = "Calendar date, as YYYY-MM-DD")),
    responses((status = 204))
)]
pub async fn handle_forget(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(date): Path<String>,
) -> Result<StatusCode, ApiError> {
    let deps = forget_cycle_start::Deps {
        command: state.cycle_command,
    };

    forget_cycle_start::execute(user_id, parse_date(&date)?, &deps).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(patch, path = "/api/v1/users/me/preferences", tag = "users", security(("bearer" = [])),
    description = "Turns optional features on or off. Cycle tracking is off until turned on, \
                   and turning it off hides the cycle without forgetting what was recorded.",
    request_body = SetPreferencesRequest,
    responses((status = 200, body = PreferencesResponse))
)]
pub async fn handle_set_preferences(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<SetPreferencesRequest>,
) -> Result<Json<PreferencesResponse>, ApiError> {
    let deps = set_preferences::Deps {
        command: state.preferences_command,
        query: state.preferences_query,
    };

    let preferences = set_preferences::execute(user_id, body.tracks_cycle, &deps).await?;

    Ok(Json(preferences.into()))
}

#[utoipa::path(get, path = "/api/v1/users/me/preferences", tag = "users", security(("bearer" = [])),
    responses((status = 200, body = PreferencesResponse))
)]
pub async fn handle_preferences(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<PreferencesResponse>, ApiError> {
    let preferences =
        application::user::preferences::preferences_of(&user_id, &state.preferences_query).await?;

    Ok(Json(preferences.into()))
}
