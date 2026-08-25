use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;

use api_types::mappers::correlation_response;
use api_types::requests::{
    CreateEntryRequest, DateRangeParams, ListEntriesParams, ReplaceActivityRequest,
    UpdateEntryRequest,
};
use api_types::responses::{
    BulkActionResponse, CalendarDayResponse, CorrelationResponse, EntryResponse, MoodStatsResponse,
};
use application::entry::queries::{FilterByActivityQuery, FilterByMoodQuery, MoodStatsQuery};
use application::entry::use_cases::{
    create_entry, delete_entries_by_date_range, delete_entry, filter_by_activity, filter_by_mood,
    get_activity_correlation, get_calendar, get_entry, get_mood_stats, list_entries,
    replace_activity, update_entry,
};
use domain::activity::ActivityId;
use domain::entry::{Mood, MoodEntryId};

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, PathId};
use crate::state::AppState;

#[utoipa::path(post, path = "/api/v1/entries", tag = "entries", security(("bearer" = [])),
    request_body = CreateEntryRequest,
    responses((status = 201, body = EntryResponse))
)]
pub async fn handle_create(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), ApiError> {
    let cmd = body.into_command(user_id, &state.entry_config)?;
    let deps = create_entry::Deps {
        entries: state.entry_command,
        events: state.event_publisher,
    };
    let entry = create_entry::execute(cmd, &deps).await?;
    Ok((StatusCode::CREATED, Json(EntryResponse::from(entry))))
}

#[utoipa::path(get, path = "/api/v1/entries/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Entry ID")),
    responses((status = 200, body = EntryResponse))
)]
pub async fn handle_get(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(entry_id): PathId<MoodEntryId>,
) -> Result<Json<EntryResponse>, ApiError> {
    let deps = get_entry::Deps {
        query: state.entry_query,
    };
    let entry = get_entry::execute(entry_id, user_id, &deps).await?;
    Ok(Json(EntryResponse::from(entry)))
}

#[utoipa::path(get, path = "/api/v1/entries", tag = "entries", security(("bearer" = [])),
    params(ListEntriesParams),
    responses((status = 200, body = Vec<EntryResponse>))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(params): Query<ListEntriesParams>,
) -> Result<Json<Vec<EntryResponse>>, ApiError> {
    let query = params.into_query(user_id)?;
    let deps = list_entries::Deps {
        query: state.entry_query,
    };
    let entries = list_entries::execute(query, &deps).await?;
    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

#[utoipa::path(patch, path = "/api/v1/entries/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Entry ID")),
    request_body = UpdateEntryRequest,
    responses((status = 200, body = EntryResponse))
)]
pub async fn handle_update(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(entry_id): PathId<MoodEntryId>,
    Json(body): Json<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, ApiError> {
    let cmd = body.into_command(entry_id, &state.entry_config)?;
    let deps = update_entry::Deps {
        command: state.entry_command,
        query: state.entry_query,
        media_storage: state.media_storage,
        events: state.event_publisher,
    };
    let entry = update_entry::execute(cmd, user_id, &deps).await?;
    Ok(Json(EntryResponse::from(entry)))
}

#[utoipa::path(delete, path = "/api/v1/entries/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Entry ID")),
    responses((status = 204))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(entry_id): PathId<MoodEntryId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_entry::Deps {
        command: state.entry_command,
        query: state.entry_query,
        events: state.event_publisher,
        media_storage: state.media_storage,
    };
    delete_entry::execute(entry_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/entries/filter/mood/{mood}", tag = "entries", security(("bearer" = [])),
    params(("mood" = u8, Path, description = "Mood value 1-5")),
    responses((status = 200, body = Vec<EntryResponse>))
)]
pub async fn handle_filter_by_mood(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Path(mood): Path<u8>,
) -> Result<Json<Vec<EntryResponse>>, ApiError> {
    let mood = Mood::try_from(mood)?;
    let query = FilterByMoodQuery { user_id, mood };
    let deps = filter_by_mood::Deps {
        query: state.entry_query,
    };
    let entries = filter_by_mood::execute(query, &deps).await?;
    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

#[utoipa::path(get, path = "/api/v1/entries/filter/activity/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Activity ID")),
    responses((status = 200, body = Vec<EntryResponse>))
)]
pub async fn handle_filter_by_activity(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(activity_id): PathId<ActivityId>,
) -> Result<Json<Vec<EntryResponse>>, ApiError> {
    let query = FilterByActivityQuery {
        user_id,
        activity_id,
    };
    let deps = filter_by_activity::Deps {
        query: state.entry_query,
    };
    let entries = filter_by_activity::execute(query, &deps).await?;
    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

#[utoipa::path(get, path = "/api/v1/entries/stats", tag = "entries", security(("bearer" = [])),
    params(ListEntriesParams),
    responses((status = 200, body = MoodStatsResponse))
)]
pub async fn handle_stats(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(params): Query<ListEntriesParams>,
) -> Result<Json<MoodStatsResponse>, ApiError> {
    let range = match (params.from, params.to) {
        (Some(from), Some(to)) => {
            let from = api_types::mappers::shared::parse_datetime(&from)?;
            let to = api_types::mappers::shared::parse_datetime(&to)?;
            Some(domain::entry::DateRange::new(from, to)?)
        }
        _ => None,
    };
    let query = MoodStatsQuery { user_id, range };
    let deps = get_mood_stats::Deps {
        query: state.entry_query,
    };
    let stats = get_mood_stats::execute(query, &deps).await?;
    Ok(Json(MoodStatsResponse::from(stats)))
}

#[utoipa::path(get, path = "/api/v1/entries/calendar", tag = "entries", security(("bearer" = [])),
    params(DateRangeParams),
    responses((status = 200, body = Vec<CalendarDayResponse>))
)]
pub async fn handle_calendar(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(params): Query<DateRangeParams>,
) -> Result<Json<Vec<CalendarDayResponse>>, ApiError> {
    let range = params.into_date_range()?;
    let deps = get_calendar::Deps {
        query: state.entry_query,
    };
    let days = get_calendar::execute(user_id, range, &deps).await?;
    Ok(Json(
        days.into_iter().map(CalendarDayResponse::from).collect(),
    ))
}

#[utoipa::path(get, path = "/api/v1/entries/correlation/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Activity ID"), ListEntriesParams),
    responses((status = 200, body = CorrelationResponse))
)]
pub async fn handle_activity_correlation(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(activity_id): PathId<ActivityId>,
    Query(params): Query<ListEntriesParams>,
) -> Result<Json<CorrelationResponse>, ApiError> {
    let range = match (params.from, params.to) {
        (Some(from), Some(to)) => {
            let from = api_types::mappers::shared::parse_datetime(&from)?;
            let to = api_types::mappers::shared::parse_datetime(&to)?;
            Some(domain::entry::DateRange::new(from, to)?)
        }
        _ => None,
    };
    let deps = get_activity_correlation::Deps {
        query: state.entry_query,
    };
    let correlation =
        get_activity_correlation::execute(user_id, activity_id.clone(), range, &deps).await?;
    Ok(Json(correlation_response(activity_id, correlation)))
}

#[utoipa::path(delete, path = "/api/v1/entries/bulk/delete", tag = "entries", security(("bearer" = [])),
    params(DateRangeParams),
    responses((status = 200, body = BulkActionResponse))
)]
pub async fn handle_delete_by_date_range(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(params): Query<DateRangeParams>,
) -> Result<Json<BulkActionResponse>, ApiError> {
    let range = params.into_date_range()?;
    let deps = delete_entries_by_date_range::Deps {
        cascade: state.cascade,
        media_storage: state.media_storage,
    };
    let affected_count = delete_entries_by_date_range::execute(user_id, &range, &deps).await?;
    Ok(Json(BulkActionResponse { affected_count }))
}

#[utoipa::path(post, path = "/api/v1/entries/bulk/replace-activity", tag = "entries", security(("bearer" = [])),
    request_body = ReplaceActivityRequest,
    responses((status = 200, body = BulkActionResponse))
)]
pub async fn handle_replace_activity(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<ReplaceActivityRequest>,
) -> Result<Json<BulkActionResponse>, ApiError> {
    let (user_id, old_id, new_id) = body.into_parts(user_id)?;
    let deps = replace_activity::Deps {
        entry_command: state.entry_command,
        activity_query: state.activity_query,
    };
    let affected_count = replace_activity::execute(user_id, old_id, new_id, &deps).await?;
    Ok(Json(BulkActionResponse { affected_count }))
}
