use crate::errors::DomainError;
use crate::user::{Email, User, UserId, Username};

use super::InMemoryStore;

#[async_trait::async_trait]
impl crate::ports::UserCommandPort for InMemoryStore {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        self.users
            .write()
            .unwrap()
            .insert(user.id().clone(), user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        self.users.write().unwrap().remove(id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::ports::UserQueryPort for InMemoryStore {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        Ok(self.users.read().unwrap().get(id).cloned())
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        Ok(self
            .users
            .read()
            .unwrap()
            .values()
            .find(|u| u.username() == username)
            .cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        Ok(self
            .users
            .read()
            .unwrap()
            .values()
            .find(|u| u.email() == email)
            .cloned())
    }
}
