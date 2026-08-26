use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;

use api_types::mappers::parse_date;
use api_types::requests::{DateSpanParams, ImportDailyMetricsRequest, SetDailyMetricsRequest};
use api_types::responses::{DailyMetricResponse, ImportOutcomeResponse, RejectionResponse};
use application::import::commands::ImportDailyMetricsCommand;
use application::import::use_cases::import_daily_metrics;
use application::metric::commands::SetDailyMetricsCommand;
use application::metric::use_cases::{list_daily_metrics, set_daily_metrics};

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, ImportingProvider, MetricWriter};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/metrics", tag = "metrics", security(("bearer" = [])),
    params(DateSpanParams),
    responses((status = 200, body = Vec<DailyMetricResponse>))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Query(params): Query<DateSpanParams>,
) -> Result<Json<Vec<DailyMetricResponse>>, ApiError> {
    let span = params.into_span()?;

    let deps = list_daily_metrics::Deps {
        metrics: state.daily_metric_query,
    };

    let metrics = list_daily_metrics::execute(user_id, span, &deps).await?;

    Ok(Json(metrics.into_iter().map(Into::into).collect()))
}

#[utoipa::path(put, path = "/api/v1/metrics/{date}", tag = "metrics", security(("bearer" = [])),
    description = "States the given metrics for one day. A null value clears that kind instead, \
                   after which a later provider import may report it again. Every kind may appear \
                   only once per request.",
    params(("date" = String, Path, description = "Calendar date, as YYYY-MM-DD")),
    request_body = SetDailyMetricsRequest,
    responses((status = 204))
)]
pub async fn handle_set(
    State(state): State<AppState>,
    writer: MetricWriter,
    Path(date): Path<String>,
    Json(body): Json<SetDailyMetricsRequest>,
) -> Result<StatusCode, ApiError> {
    let date = parse_date(&date)?;

    let changes = body
        .metrics
        .into_iter()
        .map(|payload| payload.into_change())
        .collect::<Result<Vec<_>, _>>()?;

    let deps = set_daily_metrics::Deps {
        metrics: state.daily_metric_command,
        users: state.user_query,
    };

    set_daily_metrics::execute(
        SetDailyMetricsCommand {
            user_id: writer.user_id,
            date,
            changes,
            source: writer.source,
        },
        &deps,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/v1/metrics/import", tag = "metrics", security(("bearer" = [])),
    description = "Accepts a batch of days from an automation, authenticated by an api token and \
                   nothing else. A payload carrying only some of the eight kinds is normal. Every \
                   reading is judged on its own: the valid ones are stored and the rest are \
                   rejected and written to a trace the account holder can read, so one bad value \
                   never costs a night of good data. Values are never clamped. A reading the user \
                   has stated by hand is reported as superseding the imported one, which is not a \
                   rejection. Only a payload carrying more days than the configured limit is \
                   refused outright.",
    request_body = ImportDailyMetricsRequest,
    responses((status = 200, body = ImportOutcomeResponse))
)]
pub async fn handle_import(
    State(state): State<AppState>,
    importer: ImportingProvider,
    Json(body): Json<ImportDailyMetricsRequest>,
) -> Result<Json<ImportOutcomeResponse>, ApiError> {
    let deps = import_daily_metrics::Deps {
        metrics: state.daily_metric_command,
        rejections: state.rejection_command,
    };

    let outcome = import_daily_metrics::execute(
        ImportDailyMetricsCommand {
            user_id: importer.user_id,
            provider: importer.provider,
            days: body.into_days(),
            maximum_days: state.import_config.maximum_days_per_import,
        },
        &deps,
    )
    .await?;

    Ok(Json(outcome.into()))
}

#[utoipa::path(get, path = "/api/v1/metrics/rejections", tag = "metrics", security(("bearer" = [])),
    description = "Readings that could not be used, most recent first, whether they arrived \
                   broken from an importer or were stored by an older build and can no longer be \
                   read. Only the most recent are kept.",
    responses((status = 200, body = Vec<RejectionResponse>))
)]
pub async fn handle_rejections(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<Vec<RejectionResponse>>, ApiError> {
    let rejections = state.rejection_query.find_recent_by_user(&user_id).await?;

    Ok(Json(rejections.into_iter().map(Into::into).collect()))
}
