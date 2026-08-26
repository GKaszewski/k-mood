use sqlx::SqlitePool;

use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::location::Coordinates;
use domain::ports::UnwatchedPlace;

use super::super::shared::db_err;

pub struct SqliteWeatherBacklogRepository {
    pool: SqlitePool,
}

impl SqliteWeatherBacklogRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UnwatchedPlaceRow {
    entry_id: String,
    latitude: f64,
    longitude: f64,
    logged_at: String,
}

#[async_trait::async_trait]
impl domain::ports::WeatherBacklogQueryPort for SqliteWeatherBacklogRepository {
    async fn find_places_without_weather(
        &self,
        most: usize,
    ) -> Result<Vec<UnwatchedPlace>, DomainError> {
        let rows: Vec<UnwatchedPlaceRow> = sqlx::query_as(
            "SELECT l.entry_id, l.latitude, l.longitude, e.logged_at
             FROM entry_location l
             JOIN mood_entries e ON e.id = l.entry_id
             LEFT JOIN entry_weather w ON w.entry_id = l.entry_id
             WHERE w.entry_id IS NULL
             ORDER BY e.logged_at DESC
             LIMIT ?",
        )
        .bind(i64::try_from(most).unwrap_or(i64::MAX))
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }
}

fn readable(row: &UnwatchedPlaceRow) -> Option<UnwatchedPlace> {
    Some(UnwatchedPlace {
        entry_id: MoodEntryId::from_uuid(row.entry_id.parse().ok()?),
        coordinates: Coordinates::from_persistence(row.latitude, row.longitude),
        logged_at: row.logged_at.parse().ok()?,
    })
}
