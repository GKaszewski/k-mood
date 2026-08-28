use domain::api_token::{ApiToken, MintedApiToken};

use crate::responses::{ApiTokenResponse, MintedApiTokenResponse};

impl From<ApiToken> for ApiTokenResponse {
    fn from(token: ApiToken) -> Self {
        Self {
            id: token.id().value().to_string(),
            name: token.name().value().to_string(),
            scopes: token
                .scopes()
                .names()
                .iter()
                .map(|n| n.to_string())
                .collect(),
            created_at: *token.created_at(),
            last_used_at: token.last_used_at().copied(),
        }
    }
}

impl From<MintedApiToken> for MintedApiTokenResponse {
    fn from(minted: MintedApiToken) -> Self {
        Self {
            secret: minted.secret().to_string(),
            token: minted.token().clone().into(),
        }
    }
}
