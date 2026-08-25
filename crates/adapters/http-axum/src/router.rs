use axum::extract::DefaultBodyLimit;
use axum::http::HeaderValue;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::handlers::{activities, auth, entries, import_export, media, push, reminders, users};
use crate::openapi::ApiDoc;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = build_cors(&state.server_config.cors);
    let body_limit = DefaultBodyLimit::max(state.server_config.max_body_size);

    Router::new()
        .nest("/api/v1", api_routes())
        .route("/health", get(health))
        .route("/openapi.json", get(openapi_json))
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .fallback_service(crate::spa::serve_spa(&state.server_config.spa_dir))
        .layer(body_limit)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
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
        .nest("/push", push_routes())
        .nest("/data", data_routes())
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
        .route("/stats", get(entries::handle_stats))
        .route("/calendar", get(entries::handle_calendar))
        .route("/filter/mood/{mood}", get(entries::handle_filter_by_mood))
        .route(
            "/filter/activity/{id}",
            get(entries::handle_filter_by_activity),
        )
        .route(
            "/correlation/{id}",
            get(entries::handle_activity_correlation),
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
        .route("/subscribe", post(push::handle_subscribe))
        .route("/unsubscribe", post(push::handle_unsubscribe))
        .route("/test", post(push::handle_test))
}

fn data_routes() -> Router<AppState> {
    Router::new()
        .route("/export", get(import_export::handle_export))
        .route("/import", post(import_export::handle_import))
}
