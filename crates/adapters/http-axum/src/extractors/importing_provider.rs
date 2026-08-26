use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;

use domain::provider::ProviderName;
use domain::user::UserId;

use application::api_token::use_cases::authenticate_api_token;

use crate::state::AppState;

use super::authenticated_user::{AuthRejection, bearer_token};

pub struct ImportingProvider {
    pub user_id: UserId,
    pub provider: ProviderName,
}

impl<S> FromRequestParts<S> for ImportingProvider
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let presented = bearer_token(parts)
            .ok_or_else(|| AuthRejection::new("missing or invalid authorization header"))?;

        let deps = authenticate_api_token::Deps {
            query: app_state.api_token_query,
            command: app_state.api_token_command,
            secrets: app_state.api_token_secrets,
        };

        let token = authenticate_api_token::execute(&presented, &deps)
            .await
            .map_err(|_| AuthRejection::new("importing needs an api token minted in settings"))?;

        Ok(Self {
            user_id: token.user_id().clone(),
            provider: token.name().clone(),
        })
    }
}
