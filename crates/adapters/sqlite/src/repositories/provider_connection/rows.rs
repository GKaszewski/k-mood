use domain::errors::DomainError;
use domain::provider::{
    EncryptedCredential, ProviderConnection, ProviderConnectionData, ProviderConnectionId,
    ProviderName,
};
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct ProviderConnectionRow {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub credential: Vec<u8>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn row_to_connection(row: ProviderConnectionRow) -> Result<ProviderConnection, DomainError> {
    let malformed = || DomainError::InvalidInput("stored provider connection is malformed".into());

    Ok(ProviderConnection::from_persistence(
        ProviderConnectionData {
            id: ProviderConnectionId::from_uuid(row.id.parse().map_err(|_| malformed())?),
            user_id: UserId::from_uuid(row.user_id.parse().map_err(|_| malformed())?),
            provider: ProviderName::from_persistence(row.provider),
            credential: EncryptedCredential::from_persistence(row.credential),
            created_at: row.created_at.parse().map_err(|_| malformed())?,
            updated_at: row.updated_at.parse().map_err(|_| malformed())?,
        },
    ))
}
