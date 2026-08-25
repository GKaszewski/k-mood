use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::{PushSubscribeRequest, PushUnsubscribeRequest};
use application::push::use_cases::{subscribe, unsubscribe};

use crate::errors::ApiError;
use crate::extractors::AuthenticatedUser;
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/push/vapid-key", tag = "push",
    responses((status = 200, description = "VAPID public key"))
)]
pub async fn handle_vapid_key(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if !state.push_config.enabled {
        return Err(domain::errors::DomainError::NotFound(
            "push notifications are disabled".into(),
        )
        .into());
    }
    let public_key = web_push_adapter::WebPushSender::public_key_base64(&state.push_config)?;
    Ok(Json(serde_json::json!({ "publicKey": public_key })))
}

#[utoipa::path(post, path = "/api/v1/push/subscribe", tag = "push", security(("bearer" = [])),
    request_body = PushSubscribeRequest,
    responses((status = 204))
)]
pub async fn handle_subscribe(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<PushSubscribeRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command(user_id);
    let deps = subscribe::Deps {
        push_command: state.push_subscription_command,
        push_query: state.push_subscription_query,
    };
    subscribe::execute(cmd, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/v1/push/unsubscribe", tag = "push", security(("bearer" = [])),
    request_body = PushUnsubscribeRequest,
    responses((status = 204))
)]
pub async fn handle_unsubscribe(
    State(state): State<AppState>,
    AuthenticatedUser(_user_id): AuthenticatedUser,
    Json(body): Json<PushUnsubscribeRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command();
    let deps = unsubscribe::Deps {
        push_command: state.push_subscription_command,
    };
    unsubscribe::execute(cmd, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/v1/push/test", tag = "push", security(("bearer" = [])),
    responses((status = 204))
)]
pub async fn handle_test(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<StatusCode, ApiError> {
    let sender = state
        .reminder_sender
        .as_ref()
        .ok_or_else(|| domain::errors::DomainError::InvalidInput("push not enabled".into()))?;

    sender.send_reminder(&user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
