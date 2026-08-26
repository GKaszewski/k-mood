use sqlx::SqlitePool;

use domain::entry::Date;
use domain::errors::DomainError;
use domain::user::{UserId, UserPreferences};

use super::super::shared::db_err;

pub struct SqliteCycleStartRepository {
    pool: SqlitePool,
}

impl SqliteCycleStartRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::CycleStartCommandPort for SqliteCycleStartRepository {
    async fn record(&self, user_id: &UserId, date: &Date) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO cycle_starts (user_id, date) VALUES (?, ?)
             ON CONFLICT(user_id, date) DO NOTHING",
        )
        .bind(user_id.value().to_string())
        .bind(date.to_string())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }

    async fn forget(&self, user_id: &UserId, date: &Date) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM cycle_starts WHERE user_id = ? AND date = ?")
            .bind(user_id.value().to_string())
            .bind(date.to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl domain::ports::CycleStartQueryPort for SqliteCycleStartRepository {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Date>, DomainError> {
        let rows: Vec<(String,)> =
            sqlx::query_as("SELECT date FROM cycle_starts WHERE user_id = ? ORDER BY date")
                .bind(user_id.value().to_string())
                .fetch_all(&self.pool)
                .await
                .map_err(db_err)?;

        Ok(rows
            .iter()
            .filter_map(|row| row.0.parse().ok().map(Date::from_persistence))
            .collect())
    }
}

pub struct SqliteUserPreferencesRepository {
    pool: SqlitePool,
}

impl SqliteUserPreferencesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::UserPreferencesCommandPort for SqliteUserPreferencesRepository {
    async fn save(&self, preferences: &UserPreferences) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO user_preferences (user_id, tracks_cycle) VALUES (?, ?)
             ON CONFLICT(user_id) DO UPDATE SET tracks_cycle = excluded.tracks_cycle",
        )
        .bind(preferences.user_id().value().to_string())
        .bind(preferences.tracks_cycle())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl domain::ports::UserPreferencesQueryPort for SqliteUserPreferencesRepository {
    async fn find_by_user(&self, user_id: &UserId) -> Result<Option<UserPreferences>, DomainError> {
        let row: Option<(bool,)> =
            sqlx::query_as("SELECT tracks_cycle FROM user_preferences WHERE user_id = ?")
                .bind(user_id.value().to_string())
                .fetch_optional(&self.pool)
                .await
                .map_err(db_err)?;

        Ok(row.map(|found| UserPreferences::from_persistence(user_id.clone(), found.0)))
    }
}
