use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::MintApiTokenRequest;
use api_types::responses::{ApiTokenResponse, ErrorResponse, MintedApiTokenResponse};
use application::api_token::commands::MintApiTokenCommand;
use application::api_token::use_cases::{list_api_tokens, mint_api_token, revoke_api_token};
use domain::api_token::{ApiTokenId, TokenScopes};
use domain::provider::ProviderName;

use crate::errors::ApiError;
use crate::extractors::{Body, PathId, SessionUser};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/tokens", tag = "tokens", security(("bearer" = [])),
    description = "Lists this account's api tokens. Values are never returned; only the name, \
                   when it was minted, and when it was last used.",
    responses((status = 200, body = Vec<ApiTokenResponse>),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
) -> Result<Json<Vec<ApiTokenResponse>>, ApiError> {
    let deps = list_api_tokens::Deps {
        query: state.api_token_query,
    };

    let tokens = list_api_tokens::execute(user_id, &deps).await?;

    Ok(Json(tokens.into_iter().map(Into::into).collect()))
}

#[utoipa::path(post, path = "/api/v1/tokens", tag = "tokens", security(("bearer" = [])),
    description = "Mints a token granting the scopes named in the request, and nothing else. \
                   The value comes back once and is never retrievable again. The name becomes \
                   the Provider that the token's metric writes are attributed to, so it must be \
                   lowercase letters, digits and hyphens. Account operations — minting tokens, \
                   changing the password, backup and restore — always need a signed-in session \
                   and are never reachable with a token.",
    request_body = MintApiTokenRequest,
    responses((status = 201, body = MintedApiTokenResponse),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_mint(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    Body(body): Body<MintApiTokenRequest>,
) -> Result<(StatusCode, Json<MintedApiTokenResponse>), ApiError> {
    let deps = mint_api_token::Deps {
        command: state.api_token_command,
        secrets: state.api_token_secrets,
    };

    let minted = mint_api_token::execute(
        MintApiTokenCommand {
            user_id,
            name: ProviderName::new(body.name)?,
            scopes: TokenScopes::from_names(body.scopes.iter().map(String::as_str))?,
        },
        &deps,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(minted.into())))
}

#[utoipa::path(delete, path = "/api/v1/tokens/{id}", tag = "tokens", security(("bearer" = [])),
    description = "Revokes a token. It stops working at once.",
    params(("id" = String, Path, description = "Token ID")),
    responses((status = 204),
        (status = 401, description = "No credential, or one this endpoint does not accept", body = ErrorResponse),
        (status = 403, description = "A token that does not grant what this endpoint needs", body = ErrorResponse),
        (status = 400, description = "Malformed request", body = ErrorResponse),
        (status = 422, description = "Understood but refused", body = ErrorResponse))
)]
pub async fn handle_revoke(
    State(state): State<AppState>,
    SessionUser(user_id): SessionUser,
    PathId(token_id): PathId<ApiTokenId>,
) -> Result<StatusCode, ApiError> {
    let deps = revoke_api_token::Deps {
        command: state.api_token_command,
    };

    revoke_api_token::execute(user_id, token_id, &deps).await?;

    Ok(StatusCode::NO_CONTENT)
}
