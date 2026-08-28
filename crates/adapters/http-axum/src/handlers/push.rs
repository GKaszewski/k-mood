use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::{PushSubscribeRequest, PushUnsubscribeRequest};
use api_types::responses::{ErrorResponse, PushSubscriptionResponse, VapidKeyResponse};
use application::push::use_cases::{list_subscriptions, subscribe, unsubscribe};

use crate::errors::ApiError;
use crate::extractors::{Body, SessionUser};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/push/vapid-key", tag = "push",
    responses((status = 200, body = VapidKeyResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_vapid_key(
    State(state): State<AppState>,
) -> Result<Json<VapidKeyResponse>, ApiError> {
    if !state.push_config.enabled {
        return Err(domain::errors::DomainError::NotFound(
            "push notifications are disabled".into(),
        )
        .into());
    }
    let public_key = web_push_adapter::WebPushSender::public_key_base64(&state.push_config)?;

    Ok(Json(VapidKeyResponse { public_key }))
}

#[utoipa::path(get, path = "/api/v1/push/subscriptions", tag = "push", security(("bearer" = [])),
    description = "The devices this account has subscribed for reminders. Keys are never \
                   returned; a client sees only which endpoints exist and when each was added.",
    responses((status = 200, body = Vec<PushSubscriptionResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list_subscriptions(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
) -> Result<Json<Vec<PushSubscriptionResponse>>, ApiError> {
    let deps = list_subscriptions::Deps {
        push_query: state.push_subscription_query,
    };

    let held = list_subscriptions::execute(user_id, &deps).await?;

    Ok(Json(held.into_iter().map(Into::into).collect()))
}

#[utoipa::path(post, path = "/api/v1/push/subscribe", tag = "push", security(("bearer" = [])),
    request_body = PushSubscribeRequest,
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_subscribe(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Body(body): Body<PushSubscribeRequest>,
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
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_unsubscribe(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Body(body): Body<PushUnsubscribeRequest>,
) -> Result<StatusCode, ApiError> {
    let cmd = body.into_command(user_id);
    let deps = unsubscribe::Deps {
        push_command: state.push_subscription_command,
    };
    unsubscribe::execute(cmd, &deps).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(post, path = "/api/v1/push/test", tag = "push", security(("bearer" = [])),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_test(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
) -> Result<StatusCode, ApiError> {
    let sender = state
        .reminder_sender
        .as_ref()
        .ok_or_else(|| domain::errors::DomainError::InvalidInput("push not enabled".into()))?;

    sender.send_reminder(&user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
