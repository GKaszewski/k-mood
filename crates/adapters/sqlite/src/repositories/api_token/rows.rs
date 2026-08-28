use domain::api_token::{ApiToken, ApiTokenData, ApiTokenId, TokenDigest, TokenScopes};
use domain::provider::ProviderName;
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct ApiTokenRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub digest: String,
    pub scopes: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

pub fn row_to_token(row: &ApiTokenRow) -> Option<ApiToken> {
    let last_used_at = match &row.last_used_at {
        None => None,
        Some(stamp) => Some(stamp.parse().ok()?),
    };

    Some(ApiToken::from_persistence(ApiTokenData {
        id: ApiTokenId::from_uuid(row.id.parse().ok()?),
        user_id: UserId::from_uuid(row.user_id.parse().ok()?),
        name: ProviderName::from_persistence(row.name.clone()),
        digest: TokenDigest::from_persistence(row.digest.clone()),
        scopes: TokenScopes::from_persistence(&row.scopes)?,
        created_at: row.created_at.parse().ok()?,
        last_used_at,
    }))
}

pub fn readable(row: &ApiTokenRow) -> Option<ApiToken> {
    let token = row_to_token(row);

    if token.is_none() {
        tracing::warn!(
            token_id = %row.id,
            scopes = %row.scopes,
            "skipped a stored api token this build cannot read"
        );
    }

    token
}
