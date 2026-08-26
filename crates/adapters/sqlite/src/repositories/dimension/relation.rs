use std::collections::HashMap;

use sqlx::SqlitePool;
use uuid::Uuid;

use domain::activity::ActivityId;
use domain::attachment::{PhotoId, VoiceMemoId};
use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::MoodEntryId;
use domain::errors::DomainError;

use super::super::shared::db_err;

pub struct SqliteRelationDimensionRepository {
    pool: SqlitePool,
    table: &'static str,
    column: &'static str,
    kind: DimensionKind,
}

impl SqliteRelationDimensionRepository {
    pub fn activities(pool: SqlitePool) -> Self {
        Self {
            pool,
            table: "entry_activities",
            column: "activity_id",
            kind: DimensionKind::Activities,
        }
    }

    pub fn photos(pool: SqlitePool) -> Self {
        Self {
            pool,
            table: "entry_photos",
            column: "photo_id",
            kind: DimensionKind::Photos,
        }
    }

    pub fn voice_memos(pool: SqlitePool) -> Self {
        Self {
            pool,
            table: "entry_voice_memos",
            column: "voice_memo_id",
            kind: DimensionKind::VoiceMemos,
        }
    }

    fn to_value(&self, ids: Vec<Uuid>) -> DimensionValue {
        match self.kind {
            DimensionKind::Activities => {
                DimensionValue::Activities(ids.into_iter().map(ActivityId::from_uuid).collect())
            }
            DimensionKind::Photos => {
                DimensionValue::Photos(ids.into_iter().map(PhotoId::from_uuid).collect())
            }
            DimensionKind::VoiceMemos => {
                DimensionValue::VoiceMemos(ids.into_iter().map(VoiceMemoId::from_uuid).collect())
            }
            DimensionKind::Content
            | DimensionKind::Location
            | DimensionKind::Song
            | DimensionKind::Weather => {
                unreachable!("relation repository serves only id-list dimensions")
            }
        }
    }
}

fn related_ids(value: &DimensionValue) -> Vec<String> {
    match value {
        DimensionValue::Activities(ids) => ids.iter().map(|id| id.value().to_string()).collect(),
        DimensionValue::Photos(ids) => ids.iter().map(|id| id.value().to_string()).collect(),
        DimensionValue::VoiceMemos(ids) => ids.iter().map(|id| id.value().to_string()).collect(),
        DimensionValue::Content(_)
        | DimensionValue::Location(_)
        | DimensionValue::Song(_)
        | DimensionValue::Weather(_) => Vec::new(),
    }
}

#[derive(sqlx::FromRow)]
struct RelationRow {
    entry_id: String,
    related_id: String,
}

#[async_trait::async_trait]
impl domain::ports::EntryDimensionPort for SqliteRelationDimensionRepository {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        if entry_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = vec!["?"; entry_ids.len()].join(",");
        let sql = format!(
            "SELECT entry_id, {} AS related_id FROM {} WHERE entry_id IN ({placeholders})",
            self.column, self.table
        );

        let mut query = sqlx::query_as::<_, RelationRow>(sqlx::AssertSqlSafe(sql));
        for id in entry_ids {
            query = query.bind(id.value().to_string());
        }

        let rows = query.fetch_all(&self.pool).await.map_err(db_err)?;

        let mut grouped: HashMap<MoodEntryId, Vec<Uuid>> = HashMap::new();
        for row in rows {
            let (Ok(entry_id), Ok(related_id)) =
                (row.entry_id.parse::<Uuid>(), row.related_id.parse::<Uuid>())
            else {
                continue;
            };
            grouped
                .entry(MoodEntryId::from_uuid(entry_id))
                .or_default()
                .push(related_id);
        }

        Ok(grouped
            .into_iter()
            .map(|(entry_id, ids)| (entry_id, self.to_value(ids)))
            .collect())
    }

    async fn save(
        &self,
        entry_id: &MoodEntryId,
        values: &[DimensionValue],
    ) -> Result<(), DomainError> {
        let id = entry_id.value().to_string();

        let mut tx = self.pool.begin().await.map_err(db_err)?;

        sqlx::query(sqlx::AssertSqlSafe(format!(
            "DELETE FROM {} WHERE entry_id = ?",
            self.table
        )))
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        if let Some(value) = values.iter().find(|value| value.kind() == self.kind) {
            let insert = format!(
                "INSERT OR IGNORE INTO {} (entry_id, {}) VALUES (?, ?)",
                self.table, self.column
            );
            for related in related_ids(value) {
                sqlx::query(sqlx::AssertSqlSafe(insert.clone()))
                    .bind(&id)
                    .bind(related)
                    .execute(&mut *tx)
                    .await
                    .map_err(db_err)?;
            }
        }

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }
}
