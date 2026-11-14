use axum::extract::DefaultBodyLimit;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use tower_governor::GovernorError;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::{GovernorLayer, key_extractor::SmartIpKeyExtractor};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::errors::{INTERNAL_ERROR, TOO_MANY_REQUESTS, refuse};
use crate::handlers::{
    activities, auth, comparisons, correlations, cycle, data, entries, import_export, jobs, media,
    metrics, providers, push, reminders, server, tokens, users,
};
use crate::openapi::ApiDoc;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = build_cors(&state.server_config.cors);
    let body_limit = DefaultBodyLimit::max(state.server_config.max_body_size);

    let answered_here = Router::new()
        .nest("/api/v1", api_routes())
        .route("/health", get(health))
        .route("/openapi.json", get(openapi_json))
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()));

    // The limit guards the journal, not the files the browser needs to show
    // it. One cold load of the client asks for a dozen or more hashed assets
    // at once, and counting those against the same budget as the API means a
    // hard refresh can leave the app unable to fetch its own code.
    rate_limited(answered_here, &state.server_config.rate_limit)
        .fallback_service(crate::spa::serve_spa(&state.server_config.spa_dir))
        .layer(body_limit)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// One second, in milliseconds.
const ONE_SECOND_IN_MILLIS: u64 = 1000;

/// How long to wait between replenishing one unit of the quota.
///
/// `GovernorConfigBuilder::per_second` takes the *interval* between
/// replenishments, not a rate, so feeding it a per-second figure means the
/// opposite of what the name says: `50` asks for one request every fifty
/// seconds rather than fifty every second. This converts the configured rate
/// into the interval the builder actually wants.
pub fn replenish_interval_millis(requests_per_second: u64) -> u64 {
    (ONE_SECOND_IN_MILLIS / requests_per_second.max(1)).max(1)
}

/// Applies the limit with `route_layer`, so it runs only for requests that
/// match a route here. Static client assets are served by the fallback and are
/// deliberately outside it.
pub fn rate_limited<S>(router: Router<S>, config: &config::RateLimitConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    if !config.enabled {
        tracing::warn!("rate limiting is switched off, so any client may poll as fast as it likes");

        return router;
    }

    match config.trust_forwarded_for {
        true => keyed_by_forwarded_ip(router, config),
        false => keyed_by_peer_ip(router, config),
    }
}

fn keyed_by_forwarded_ip<S>(router: Router<S>, config: &config::RateLimitConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let built = GovernorConfigBuilder::default()
        .per_millisecond(replenish_interval_millis(config.requests_per_second))
        .burst_size(config.burst)
        .key_extractor(SmartIpKeyExtractor)
        .finish();

    match built {
        Some(governor) => router.route_layer(GovernorLayer::new(governor).error_handler(slow_down)),
        None => unusable(router),
    }
}

fn keyed_by_peer_ip<S>(router: Router<S>, config: &config::RateLimitConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let built = GovernorConfigBuilder::default()
        .per_millisecond(replenish_interval_millis(config.requests_per_second))
        .burst_size(config.burst)
        .finish();

    match built {
        Some(governor) => router.route_layer(GovernorLayer::new(governor).error_handler(slow_down)),
        None => unusable(router),
    }
}

fn unusable<S>(router: Router<S>) -> Router<S> {
    tracing::error!("the configured rate limit is not usable, so none is applied");

    router
}

fn slow_down(error: GovernorError) -> axum::response::Response {
    let (status, code, message) = match error {
        GovernorError::TooManyRequests { .. } => (
            StatusCode::TOO_MANY_REQUESTS,
            TOO_MANY_REQUESTS,
            "too many requests: slow down and try again shortly".to_string(),
        ),
        GovernorError::UnableToExtractKey => (
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL_ERROR,
            "the caller could not be identified for rate limiting".to_string(),
        ),
        GovernorError::Other { msg, .. } => (
            StatusCode::INTERNAL_SERVER_ERROR,
            INTERNAL_ERROR,
            msg.unwrap_or_else(|| "rate limiting failed".to_string()),
        ),
    };

    refuse(status, code, message)
}

async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

fn build_cors(config: &config::CorsConfig) -> CorsLayer {
    let layer = CorsLayer::new().allow_methods(Any).allow_headers(Any);

    if config.allow_any_origin {
        return layer.allow_origin(Any);
    }

    let origins: Vec<HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();

    layer.allow_origin(AllowOrigin::list(origins))
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/entries", entry_routes())
        .nest("/activities", activity_routes())
        .nest("/users", user_routes())
        .nest("/reminders", reminder_routes())
        .nest("/media", media_routes())
        .nest("/metrics", metric_routes())
        .nest("/correlations", correlation_routes())
        .nest("/comparisons", comparison_routes())
        .nest("/tokens", token_routes())
        .nest("/cycle", cycle_routes())
        .nest("/providers", provider_routes())
        .nest("/push", push_routes())
        .nest("/data", data_routes())
        .route("/server", get(server::handle_info))
        .route("/jobs/exhausted", get(jobs::handle_list_exhausted))
}

fn cycle_routes() -> Router<AppState> {
    Router::new().route("/", get(cycle::handle_read)).route(
        "/{date}",
        put(cycle::handle_record).delete(cycle::handle_forget),
    )
}

fn token_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(tokens::handle_list).post(tokens::handle_mint))
        .route("/{id}", delete(tokens::handle_revoke))
}

fn correlation_routes() -> Router<AppState> {
    Router::new().route("/", get(correlations::handle_list))
}

fn comparison_routes() -> Router<AppState> {
    Router::new().route("/", get(comparisons::handle_compare))
}

fn metric_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(metrics::handle_list))
        .route("/import", post(metrics::handle_import))
        .route("/rejections", get(metrics::handle_rejections))
        .route("/{date}", put(metrics::handle_set))
}

fn provider_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(providers::handle_list))
        .route("/now-playing", get(providers::handle_now_playing))
        .route(
            "/{provider}",
            put(providers::handle_connect).delete(providers::handle_disconnect),
        )
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::handle_login))
        .route("/refresh", post(auth::handle_refresh))
        .route("/logout", post(auth::handle_logout))
}

fn entry_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(entries::handle_list).post(entries::handle_create))
        .route(
            "/{id}",
            get(entries::handle_get)
                .patch(entries::handle_update)
                .delete(entries::handle_delete),
        )
        .route("/bulk", post(entries::handle_create_many))
        .route("/stats", get(entries::handle_stats))
        .route("/trend", get(entries::handle_trend))
        .route("/calendar", get(entries::handle_calendar))
        .route("/filter/mood/{mood}", get(entries::handle_filter_by_mood))
        .route(
            "/filter/activity/{id}",
            get(entries::handle_filter_by_activity),
        )
        .route("/bulk/delete", delete(entries::handle_delete_by_date_range))
        .route(
            "/bulk/replace-activity",
            post(entries::handle_replace_activity),
        )
}

fn activity_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(activities::handle_list).post(activities::handle_create),
        )
        .route(
            "/{id}",
            get(activities::handle_get).delete(activities::handle_delete),
        )
        .route("/{id}/name", patch(activities::handle_rename))
        .route("/{id}/category", patch(activities::handle_set_category))
        .route("/{id}/archive", post(activities::handle_archive))
        .route("/{id}/unarchive", post(activities::handle_unarchive))
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(users::handle_register))
        .route(
            "/me",
            get(users::handle_get_profile)
                .patch(users::handle_update_profile)
                .delete(users::handle_delete),
        )
        .route("/me/password", patch(users::handle_change_password))
        .route("/me/data", delete(users::handle_clear_data))
        .route(
            "/me/preferences",
            get(cycle::handle_preferences).patch(cycle::handle_set_preferences),
        )
}

fn reminder_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(reminders::handle_list).post(reminders::handle_create),
        )
        .route(
            "/{id}",
            get(reminders::handle_get)
                .patch(reminders::handle_update)
                .delete(reminders::handle_delete),
        )
}

fn media_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(media::handle_list))
        .route("/photos", post(media::handle_upload_photo))
        .route(
            "/photos/{id}",
            get(media::handle_serve_photo).delete(media::handle_delete_photo),
        )
        .route("/voice-memos", post(media::handle_upload_voice_memo))
        .route(
            "/voice-memos/{id}",
            get(media::handle_serve_voice_memo).delete(media::handle_delete_voice_memo),
        )
}

fn push_routes() -> Router<AppState> {
    Router::new()
        .route("/vapid-key", get(push::handle_vapid_key))
        .route("/subscriptions", get(push::handle_list_subscriptions))
        .route("/subscribe", post(push::handle_subscribe))
        .route("/unsubscribe", post(push::handle_unsubscribe))
        .route("/test", post(push::handle_test))
}

fn data_routes() -> Router<AppState> {
    Router::new()
        .route("/backup", get(data::handle_backup))
        .route("/extract", get(data::handle_extract))
        .route("/restore", post(data::handle_restore))
        .route("/import", post(import_export::handle_import))
}
