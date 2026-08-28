use std::sync::Arc;

use domain::attachment::MediaRef;
use domain::errors::DomainError;
use domain::ports::MediaOwnershipPort;
use domain::user::UserId;

use crate::authorization::verify_ownership;
use crate::errors::ApplicationError;

pub async fn verify_custody(
    media: MediaRef,
    caller: &UserId,
    ownership: &Arc<dyn MediaOwnershipPort>,
) -> Result<(), ApplicationError> {
    let owner = ownership
        .owner_of(media)
        .await?
        .ok_or_else(|| DomainError::NotFound(format!("{media} not found")))?;

    verify_ownership(&owner, caller)
}
