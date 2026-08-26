use std::collections::HashMap;

use sqlx::SqlitePool;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::provider::ProviderName;
use domain::weather::{Celsius, Condition, Weather};

use super::super::shared::db_err;

pub struct SqliteWeatherDimensionRepository {
    pool: SqlitePool,
}

impl SqliteWeatherDimensionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct WeatherRow {
    entry_id: String,
    condition: String,
    temperature: f64,
    observed_by: String,
}

#[async_trait::async_trait]
impl domain::ports::EntryDimensionPort for SqliteWeatherDimensionRepository {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        if entry_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = vec!["?"; entry_ids.len()].join(",");
        let sql = format!(
            "SELECT entry_id, condition, temperature, observed_by
             FROM entry_weather WHERE entry_id IN ({placeholders})"
        );

        let mut query = sqlx::query_as::<_, WeatherRow>(sqlx::AssertSqlSafe(sql));
        for id in entry_ids {
            query = query.bind(id.value().to_string());
        }

        let rows = query.fetch_all(&self.pool).await.map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }

    async fn save(
        &self,
        entry_id: &MoodEntryId,
        values: &[DimensionValue],
    ) -> Result<(), DomainError> {
        let observed = values
            .iter()
            .find(|value| value.kind() == DimensionKind::Weather);

        let Some(DimensionValue::Weather(weather)) = observed else {
            return Ok(());
        };

        sqlx::query(
            "INSERT INTO entry_weather (entry_id, condition, temperature, observed_by)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(entry_id) DO UPDATE SET
                condition = excluded.condition,
                temperature = excluded.temperature,
                observed_by = excluded.observed_by",
        )
        .bind(entry_id.value().to_string())
        .bind(weather.condition().name())
        .bind(weather.temperature().value())
        .bind(weather.observed_by().value())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }
}

fn readable(row: &WeatherRow) -> Option<(MoodEntryId, DimensionValue)> {
    let entry_id = MoodEntryId::from_uuid(row.entry_id.parse().ok()?);
    let condition = Condition::from_name(&row.condition);

    if condition.is_none() {
        tracing::warn!(
            entry_id = %row.entry_id,
            condition = %row.condition,
            "skipped stored weather this build cannot read"
        );
    }

    let weather = Weather::new(
        condition?,
        Celsius::from_persistence(row.temperature),
        ProviderName::from_persistence(row.observed_by.clone()),
    );

    Some((entry_id, DimensionValue::Weather(weather)))
}
