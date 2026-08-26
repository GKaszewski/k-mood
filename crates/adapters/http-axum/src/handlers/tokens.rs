use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use api_types::requests::MintApiTokenRequest;
use api_types::responses::{ApiTokenResponse, MintedApiTokenResponse};
use application::api_token::commands::MintApiTokenCommand;
use application::api_token::use_cases::{list_api_tokens, mint_api_token, revoke_api_token};
use domain::api_token::ApiTokenId;
use domain::provider::ProviderName;

use crate::errors::ApiError;
use crate::extractors::{AuthenticatedUser, PathId};
use crate::state::AppState;

#[utoipa::path(get, path = "/api/v1/tokens", tag = "tokens", security(("bearer" = [])),
    description = "Lists this account's api tokens. Values are never returned; only the name, \
                   when it was minted, and when it was last used.",
    responses((status = 200, body = Vec<ApiTokenResponse>))
)]
pub async fn handle_list(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<Vec<ApiTokenResponse>>, ApiError> {
    let deps = list_api_tokens::Deps {
        query: state.api_token_query,
    };

    let tokens = list_api_tokens::execute(user_id, &deps).await?;

    Ok(Json(tokens.into_iter().map(Into::into).collect()))
}

#[utoipa::path(post, path = "/api/v1/tokens", tag = "tokens", security(("bearer" = [])),
    description = "Mints a token for writing daily metrics. The value comes back once and is \
                   never retrievable again. The name becomes the Provider that the token's \
                   writes are attributed to, so it must be lowercase letters, digits and hyphens.",
    request_body = MintApiTokenRequest,
    responses((status = 201, body = MintedApiTokenResponse))
)]
pub async fn handle_mint(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    Json(body): Json<MintApiTokenRequest>,
) -> Result<(StatusCode, Json<MintedApiTokenResponse>), ApiError> {
    let deps = mint_api_token::Deps {
        command: state.api_token_command,
        secrets: state.api_token_secrets,
    };

    let minted = mint_api_token::execute(
        MintApiTokenCommand {
            user_id,
            name: ProviderName::new(body.name)?,
        },
        &deps,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(minted.into())))
}

#[utoipa::path(delete, path = "/api/v1/tokens/{id}", tag = "tokens", security(("bearer" = [])),
    description = "Revokes a token. It stops working at once.",
    params(("id" = String, Path, description = "Token ID")),
    responses((status = 204))
)]
pub async fn handle_revoke(
    State(state): State<AppState>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    PathId(token_id): PathId<ApiTokenId>,
) -> Result<StatusCode, ApiError> {
    let deps = revoke_api_token::Deps {
        command: state.api_token_command,
    };

    revoke_api_token::execute(user_id, token_id, &deps).await?;

    Ok(StatusCode::NO_CONTENT)
}
