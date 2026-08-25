use crate::errors::DomainError;
use crate::user::{Email, User, UserId, Username};

#[async_trait::async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
}

#[async_trait::async_trait]
pub trait UserQueryPort: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
}
