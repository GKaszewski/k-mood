use std::collections::HashMap;

use sqlx::SqlitePool;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::{Content, MoodEntryId};
use domain::errors::DomainError;

use super::super::shared::db_err;

pub struct SqliteContentDimensionRepository {
    pool: SqlitePool,
}

impl SqliteContentDimensionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ContentRow {
    entry_id: String,
    content: String,
}

#[async_trait::async_trait]
impl domain::ports::EntryDimensionPort for SqliteContentDimensionRepository {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        if entry_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = vec!["?"; entry_ids.len()].join(",");
        let sql = format!(
            "SELECT entry_id, content FROM entry_content WHERE entry_id IN ({placeholders})"
        );

        let mut query = sqlx::query_as::<_, ContentRow>(sqlx::AssertSqlSafe(sql));
        for id in entry_ids {
            query = query.bind(id.value().to_string());
        }

        let rows = query.fetch_all(&self.pool).await.map_err(db_err)?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let id = row.entry_id.parse().ok()?;
                Some((
                    MoodEntryId::from_uuid(id),
                    DimensionValue::Content(Content::from_persistence(row.content)),
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
            .find(|value| value.kind() == DimensionKind::Content)
        {
            Some(DimensionValue::Content(content)) => {
                sqlx::query(
                    "INSERT INTO entry_content (entry_id, content) VALUES (?, ?)
                     ON CONFLICT(entry_id) DO UPDATE SET content = excluded.content",
                )
                .bind(&id)
                .bind(content.value())
                .execute(&self.pool)
                .await
                .map_err(db_err)?;
            }
            _ => {
                sqlx::query("DELETE FROM entry_content WHERE entry_id = ?")
                    .bind(&id)
                    .execute(&self.pool)
                    .await
                    .map_err(db_err)?;
            }
        }

        Ok(())
    }
}
