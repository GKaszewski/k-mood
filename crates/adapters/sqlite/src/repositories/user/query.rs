use sqlx::SqlitePool;

use domain::errors::DomainError;
use domain::user::{Email, User, UserId, Username};

use super::super::shared::db_err;
use super::rows::UserRow;

pub struct SqliteUserQueryRepository {
    pool: SqlitePool,
}

impl SqliteUserQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::UserQueryPort for SqliteUserQueryRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = ?")
            .bind(id.value().to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(row.map(UserRow::into_domain))
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = ?")
            .bind(username.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(row.map(UserRow::into_domain))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let row = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = ?")
            .bind(email.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(row.map(UserRow::into_domain))
    }
}
