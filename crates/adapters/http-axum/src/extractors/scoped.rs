use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;

use domain::api_token::TokenScope;
use domain::metric::Source;
use domain::user::UserId;

use crate::state::AppState;

use super::authentication::{
    AuthRejection, administrator, session_only, session_or_token_granting,
};

pub struct SessionUser(pub UserId);

pub struct Administrator(pub UserId);

pub struct JournalReader(pub UserId);

pub struct JournalWriter(pub UserId);

pub struct ProfileReader(pub UserId);

pub struct MetricWriter {
    pub user_id: UserId,
    pub source: Source,
}

impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        Ok(Self(session_only(parts, &state).await?))
    }
}

impl<S> FromRequestParts<S> for Administrator
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        Ok(Self(administrator(parts, &state).await?))
    }
}

macro_rules! scoped_extractor {
    ($name:ident, $scope:expr) => {
        impl<S> FromRequestParts<S> for $name
        where
            S: Send + Sync,
            AppState: FromRef<S>,
        {
            type Rejection = AuthRejection;

            async fn from_request_parts(
                parts: &mut Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                let state = AppState::from_ref(state);
                let bearer = session_or_token_granting(parts, &state, $scope).await?;

                Ok(Self(bearer.into_user_id()))
            }
        }
    };
}

scoped_extractor!(JournalReader, TokenScope::ReadJournal);
scoped_extractor!(JournalWriter, TokenScope::WriteJournal);
scoped_extractor!(ProfileReader, TokenScope::ReadProfile);

impl<S> FromRequestParts<S> for MetricWriter
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let bearer = session_or_token_granting(parts, &state, TokenScope::WriteMetrics).await?;

        Ok(Self {
            source: bearer.attributes_writes_to(),
            user_id: bearer.into_user_id(),
        })
    }
}
