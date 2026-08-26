use std::collections::HashMap;

use sqlx::SqlitePool;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::location::Coordinates;

use super::super::shared::db_err;

pub struct SqliteLocationDimensionRepository {
    pool: SqlitePool,
}

impl SqliteLocationDimensionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct LocationRow {
    entry_id: String,
    latitude: f64,
    longitude: f64,
}

#[async_trait::async_trait]
impl domain::ports::EntryDimensionPort for SqliteLocationDimensionRepository {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        if entry_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = vec!["?"; entry_ids.len()].join(",");
        let sql = format!(
            "SELECT entry_id, latitude, longitude FROM entry_location WHERE entry_id IN ({placeholders})"
        );

        let mut query = sqlx::query_as::<_, LocationRow>(sqlx::AssertSqlSafe(sql));
        for id in entry_ids {
            query = query.bind(id.value().to_string());
        }

        let rows = query.fetch_all(&self.pool).await.map_err(db_err)?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let entry_id = row.entry_id.parse().ok()?;
                Some((
                    MoodEntryId::from_uuid(entry_id),
                    DimensionValue::Location(Coordinates::from_persistence(
                        row.latitude,
                        row.longitude,
                    )),
                ))
            })
            .collect())
    }

    async fn save(
        &self,
        entry_id: &MoodEntryId,
        values: &[DimensionValue],
    ) -> Result<(), DomainError> {
        let id = entry_id.value().to_string();

        match values
            .iter()
            .find(|value| value.kind() == DimensionKind::Location)
        {
            Some(DimensionValue::Location(coordinates)) => {
                sqlx::query(
                    "INSERT INTO entry_location (entry_id, latitude, longitude) VALUES (?, ?, ?)
                     ON CONFLICT(entry_id) DO UPDATE SET
                        latitude = excluded.latitude, longitude = excluded.longitude",
                )
                .bind(&id)
                .bind(coordinates.latitude().value())
                .bind(coordinates.longitude().value())
                .execute(&self.pool)
                .await
                .map_err(db_err)?;
            }
            _ => {
                sqlx::query("DELETE FROM entry_location WHERE entry_id = ?")
                    .bind(&id)
                    .execute(&self.pool)
                    .await
                    .map_err(db_err)?;
            }
        }

        Ok(())
    }
}
