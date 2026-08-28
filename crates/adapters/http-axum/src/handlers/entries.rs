use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use api_types::params::{DateRangeParams, ListEntriesParams};
use api_types::requests::{
    CreateEntriesRequest, CreateEntryRequest, ReplaceActivityRequest, UpdateEntryRequest,
};
use api_types::responses::{
    BulkActionResponse, CalendarDayResponse, EntryPageResponse, EntryResponse, ErrorResponse,
    MoodStatsResponse,
};
use application::entry::composition::EntryComposer;
use application::entry::queries::MoodStatsQuery;
use application::entry::use_cases::{
    create_entries, create_entry, delete_entries_by_date_range, delete_entry, get_calendar,
    get_entry, get_mood_stats, list_entries, replace_activity, update_entry,
};
use domain::entry::{MoodEntry, MoodEntryId, Page};

use crate::errors::ApiError;
use crate::extractors::{Body, JournalReader, JournalWriter, Params, PathId};
use crate::state::AppState;

async fn compose(
    dimensions: Vec<std::sync::Arc<dyn domain::ports::EntryDimensionPort>>,
    entries: Vec<MoodEntry>,
) -> Result<Vec<EntryResponse>, ApiError> {
    let composed = EntryComposer::new(dimensions).compose(entries).await?;
    Ok(composed.into_iter().map(EntryResponse::from).collect())
}

async fn composed_page(
    dimensions: Vec<std::sync::Arc<dyn domain::ports::EntryDimensionPort>>,
    page: Page<MoodEntry>,
) -> Result<EntryPageResponse, ApiError> {
    let total = page.total();
    let limit = page.limit();
    let offset = page.offset();
    let has_more = page.more_after_this();

    Ok(EntryPageResponse {
        items: compose(dimensions, page.into_items()).await?,
        total,
        limit,
        offset,
        has_more,
    })
}

async fn compose_one(
    dimensions: Vec<std::sync::Arc<dyn domain::ports::EntryDimensionPort>>,
    entry: MoodEntry,
) -> Result<EntryResponse, ApiError> {
    let mut responses = compose(dimensions, vec![entry]).await?;
    Ok(responses.remove(0))
}

#[utoipa::path(post, path = "/api/v1/entries", tag = "entries", security(("bearer" = [])),
    request_body = CreateEntryRequest,
    responses((status = 201, body = EntryResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_create(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    Body(body): Body<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), ApiError> {
    let cmd = body.into_command(user_id, &state.entry_config)?;
    let dimensions = state.dimensions.clone();
    let deps = create_entry::Deps {
        activities: state.activity_query.clone(),
        entries: state.entry_command,
        dimensions: state.dimensions.clone(),
        events: state.event_publisher,
    };
    let entry = create_entry::execute(cmd, &deps).await?;
    Ok((
        StatusCode::CREATED,
        Json(compose_one(dimensions, entry).await?),
    ))
}

#[utoipa::path(post, path = "/api/v1/entries/bulk", tag = "entries", security(("bearer" = [])),
    description = "Creates many entries in one request, for a client syncing what it collected \
                   while offline. Either every entry is written or none is: one activity the \
                   account does not own refuses the whole batch.",
    request_body = CreateEntriesRequest,
    responses((status = 201, body = Vec<EntryResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_create_many(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    Body(body): Body<CreateEntriesRequest>,
) -> Result<(StatusCode, Json<Vec<EntryResponse>>), ApiError> {
    let wanted = body
        .entries
        .into_iter()
        .map(|one| one.into_command(user_id.clone(), &state.entry_config))
        .collect::<Result<Vec<_>, _>>()?;

    let dimensions = state.dimensions.clone();
    let deps = create_entries::Deps {
        entries: state.entry_command,
        dimensions: state.dimensions.clone(),
        activities: state.activity_query.clone(),
        events: state.event_publisher,
    };

    let created = create_entries::execute(user_id, wanted, &deps).await?;

    Ok((
        StatusCode::CREATED,
        Json(compose(dimensions, created).await?),
    ))
}

#[utoipa::path(get, path = "/api/v1/entries/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Entry ID")),
    responses((status = 200, body = EntryResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_get(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    PathId(entry_id): PathId<MoodEntryId>,
) -> Result<Json<EntryResponse>, ApiError> {
    let dimensions = state.dimensions.clone();
    let deps = get_entry::Deps {
        query: state.entry_query,
    };
    let entry = get_entry::execute(entry_id, user_id, &deps).await?;
    Ok(Json(compose_one(dimensions, entry).await?))
}

#[utoipa::path(get, path = "/api/v1/entries", tag = "entries", security(("bearer" = [])),
    params(ListEntriesParams),
    description = "One page of entries, newest first, narrowed by any combination of the \
                   parameters. `total` counts everything the selection matches, not the page, \
                   so a client can page with confidence. `updatedSince` returns only entries \
                   changed after that instant, which is what a polling client should use rather \
                   than refetching the journal.",
    responses((status = 200, body = EntryPageResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    Params(params): Params<ListEntriesParams>,
) -> Result<Json<EntryPageResponse>, ApiError> {
    let query = params.into_query(user_id, &state.entry_config)?;
    let dimensions = state.dimensions.clone();
    let deps = list_entries::Deps {
        query: state.entry_query,
    };

    let page = list_entries::execute(query, &deps).await?;

    Ok(Json(composed_page(dimensions, page).await?))
}

#[utoipa::path(patch, path = "/api/v1/entries/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Entry ID")),
    request_body = UpdateEntryRequest,
    responses((status = 200, body = EntryResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_update(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(entry_id): PathId<MoodEntryId>,
    Body(body): Body<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, ApiError> {
    let cmd = body.into_command(entry_id, &state.entry_config)?;
    let dimensions = state.dimensions.clone();
    let deps = update_entry::Deps {
        activities: state.activity_query.clone(),
        command: state.entry_command,
        dimensions: state.dimensions.clone(),
        query: state.entry_query,
        media_storage: state.media_storage,
        events: state.event_publisher,
    };
    let entry = update_entry::execute(cmd, user_id, &deps).await?;
    Ok(Json(compose_one(dimensions, entry).await?))
}

#[utoipa::path(delete, path = "/api/v1/entries/{id}", tag = "entries", security(("bearer" = [])),
    params(("id" = String, Path, description = "Entry ID")),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_delete(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    PathId(entry_id): PathId<MoodEntryId>,
) -> Result<StatusCode, ApiError> {
    let deps = delete_entry::Deps {
        dimensions: state.dimensions.clone(),
        command: state.entry_command,
        query: state.entry_query,
        events: state.event_publisher,
        media_storage: state.media_storage,
    };
    delete_entry::execute(entry_id, user_id, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/entries/filter/mood/{mood}", tag = "entries", security(("bearer" = [])),
    description = "A page of entries of one mood. The same selection is available on \
                   GET /entries as the `mood` parameter, alongside every other narrowing.",
    params(("mood" = u8, Path, description = "Mood value 1-5"), ListEntriesParams),
    responses((status = 200, body = EntryPageResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_filter_by_mood(
    State(state): State<AppState>,
    reader: JournalReader,
    Path(mood): Path<u8>,
    Params(params): Params<ListEntriesParams>,
) -> Result<Json<EntryPageResponse>, ApiError> {
    let narrowed = ListEntriesParams {
        mood: Some(mood),
        ..params
    };

    handle_list(State(state), reader, Params(narrowed)).await
}

#[utoipa::path(get, path = "/api/v1/entries/filter/activity/{id}", tag = "entries", security(("bearer" = [])),
    description = "A page of entries tagged with one activity. The same selection is available \
                   on GET /entries as the `activity` parameter.",
    params(("id" = String, Path, description = "Activity ID"), ListEntriesParams),
    responses((status = 200, body = EntryPageResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_filter_by_activity(
    State(state): State<AppState>,
    reader: JournalReader,
    Path(activity_id): Path<String>,
    Params(params): Params<ListEntriesParams>,
) -> Result<Json<EntryPageResponse>, ApiError> {
    let narrowed = ListEntriesParams {
        activity: Some(activity_id),
        ..params
    };

    handle_list(State(state), reader, Params(narrowed)).await
}

#[utoipa::path(get, path = "/api/v1/entries/stats", tag = "entries", security(("bearer" = [])),
    params(ListEntriesParams),
    responses((status = 200, body = MoodStatsResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_stats(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    Params(params): Params<ListEntriesParams>,
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
        users: state.user_query.clone(),
        query: state.entry_query,
    };
    let stats = get_mood_stats::execute(query, &deps).await?;
    Ok(Json(MoodStatsResponse::from(stats)))
}

#[utoipa::path(get, path = "/api/v1/entries/calendar", tag = "entries", security(("bearer" = [])),
    params(DateRangeParams),
    responses((status = 200, body = Vec<CalendarDayResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_calendar(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    Params(params): Params<DateRangeParams>,
) -> Result<Json<Vec<CalendarDayResponse>>, ApiError> {
    let range = params.into_date_range()?;
    let deps = get_calendar::Deps {
        users: state.user_query.clone(),
        query: state.entry_query,
        dimensions: state.dimensions.clone(),
        cycles: state.cycle_query,
        preferences: state.preferences_query,
    };
    let days = get_calendar::execute(user_id, range, &deps).await?;
    Ok(Json(
        days.into_iter().map(CalendarDayResponse::from).collect(),
    ))
}

#[utoipa::path(delete, path = "/api/v1/entries/bulk/delete", tag = "entries", security(("bearer" = [])),
    params(DateRangeParams),
    responses((status = 200, body = BulkActionResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_delete_by_date_range(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    Params(params): Params<DateRangeParams>,
) -> Result<Json<BulkActionResponse>, ApiError> {
    let range = params.into_date_range()?;
    let deps = delete_entries_by_date_range::Deps {
        query: state.entry_query.clone(),
        dimensions: state.dimensions.clone(),
        cascade: state.cascade,
        media_storage: state.media_storage,
    };
    let affected_count = delete_entries_by_date_range::execute(user_id, &range, &deps).await?;
    Ok(Json(BulkActionResponse { affected_count }))
}

#[utoipa::path(post, path = "/api/v1/entries/bulk/replace-activity", tag = "entries", security(("bearer" = [])),
    request_body = ReplaceActivityRequest,
    responses((status = 200, body = BulkActionResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_replace_activity(
    State(state): State<AppState>,
    JournalWriter(user_id): JournalWriter,
    Body(body): Body<ReplaceActivityRequest>,
) -> Result<Json<BulkActionResponse>, ApiError> {
    let (user_id, old_id, new_id) = body.into_parts(user_id)?;
    let deps = replace_activity::Deps {
        entry_command: state.entry_command,
        activity_query: state.activity_query,
    };
    let affected_count = replace_activity::execute(user_id, old_id, new_id, &deps).await?;
    Ok(Json(BulkActionResponse { affected_count }))
}
