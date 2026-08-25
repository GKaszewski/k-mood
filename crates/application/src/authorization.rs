use domain::errors::DomainError;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub fn verify_ownership(resource_owner: &UserId, caller: &UserId) -> Result<(), ApplicationError> {
    if resource_owner != caller {
        return Err(DomainError::Forbidden("access denied".into()).into());
    }
    Ok(())
}
