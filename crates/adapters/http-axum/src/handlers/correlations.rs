use axum::Json;
use axum::extract::State;

use api_types::params::DateSpanParams;
use api_types::responses::{CorrelationRowResponse, ErrorResponse};
use application::correlation::queries::CorrelationQuery;
use application::correlation::use_cases::get_correlations;

use crate::errors::ApiError;
use crate::extractors::{JournalReader, Params};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/correlations", tag = "correlations", security(("bearer" = [])),
    description = "Scores every metric kind, every active activity, and the moon as a control \
                   against the mean mood of each day in the span. Every strategy that fits the \
                   input is run and all of them are returned; agreement across them is the \
                   headline, not any single coefficient. Rows come back in a fixed order and are \
                   never ranked by strength. Below the configured minimum sample size a row \
                   carries its day count and no coefficient.",
    params(DateSpanParams),
    responses((status = 200, body = Vec<CorrelationRowResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    Params(params): Params<DateSpanParams>,
) -> Result<Json<Vec<CorrelationRowResponse>>, ApiError> {
    let span = params.into_span()?;

    let deps = get_correlations::Deps {
        entries: state.entry_query,
        metrics: state.daily_metric_query,
        activities: state.activity_query,
        cycles: state.cycle_query,
        weather_store: state.weather_store,
        activity_store: state.activity_store,
        preferences: state.preferences_query,
        users: state.user_query,
    };

    let query = CorrelationQuery {
        user_id,
        span,
        minimum_sample_size: state.analysis_config.minimum_sample_size,
        false_discovery_rate: state.analysis_config.false_discovery_rate,
    };

    let rows = get_correlations::execute(query, &deps).await?;

    Ok(Json(rows.into_iter().map(Into::into).collect()))
}
