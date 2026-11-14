use axum::Json;
use axum::extract::State;

use api_types::params::ComparedSpansParams;
use api_types::responses::{ComparisonResponse, ErrorResponse};
use application::comparison::queries::ComparisonQuery;
use application::comparison::use_cases::compare_periods;
use domain::comparison::{ComparisonRules, MaximumSpan};

use crate::errors::ApiError;
use crate::extractors::{JournalReader, Params};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/comparisons", tag = "comparisons", security(("bearer" = [])),
    description = "Compares two date spans, A and B, of the caller's journal. Each span is \
                   normalised before comparing: a span reaching past today is clipped to today and \
                   reported with clipped = true, and shares and averages are taken over the days \
                   actually logged rather than the days in the span, so spans of different length \
                   compare fairly. A span wholly in the future, or longer than the configured \
                   maximum, is refused. Rows are grouped into overview, mood, activities, \
                   metrics, weather and cycle sections in a fixed order and are never ranked. \
                   Every row carries both values, the absolute and relative change, the sample \
                   behind each side, and a significance verdict. When the spans overlap the two \
                   samples are not independent, so no row is tested and every verdict is \
                   notTested. Otherwise a row is likelyReal or noise; notEnoughData when a side \
                   has fewer days than the configured minimum sample size, or when a share \
                   expects too few occurrences for its test; and notTested when nothing varies, \
                   such as a mood felt on no day of either span. A figure that cannot be computed is null, never \
                   NaN. The cycle section is null when no cycle rows apply.",
    params(ComparedSpansParams),
    responses((status = 200, body = ComparisonResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_compare(
    State(state): State<AppState>,
    JournalReader(user_id): JournalReader,
    Params(params): Params<ComparedSpansParams>,
) -> Result<Json<ComparisonResponse>, ApiError> {
    let (a, b) = params.into_spans()?;

    let deps = compare_periods::Deps {
        entries: state.entry_query,
        metrics: state.daily_metric_query,
        activities: state.activity_query,
        cycles: state.cycle_query,
        dimensions: state.dimensions.clone(),
        preferences: state.preferences_query,
        users: state.user_query,
    };

    let query = ComparisonQuery {
        user_id,
        a,
        b,
        maximum_span: MaximumSpan::of_days(state.analysis_config.maximum_comparison_days),
        rules: ComparisonRules {
            minimum_sample_size: state.analysis_config.comparison_minimum_sample_size,
            false_discovery_rate: state.analysis_config.false_discovery_rate,
        },
        top_music_count: state.analysis_config.top_music_count,
    };

    let compared = compare_periods::execute(query, &deps).await?;

    Ok(Json(compared.into()))
}
