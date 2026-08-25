use std::collections::HashMap;

use sqlx::SqlitePool;

use domain::activity::ActivityId;
use domain::attachment::{PhotoId, VoiceMemoId};
use domain::entry::{Content, Mood, MoodEntry, MoodEntryData, MoodEntryId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;

#[derive(sqlx::FromRow)]
pub struct EntryRow {
    pub id: String,
    pub user_id: String,
    pub mood: i32,
    pub logged_at: String,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(sqlx::FromRow)]
struct RelationRow {
    entry_id: String,
    related_id: String,
}

pub fn row_to_entry(
    row: EntryRow,
    activity_ids: Vec<String>,
    photo_ids: Vec<String>,
    voice_memo_ids: Vec<String>,
) -> Result<MoodEntry, DomainError> {
    Ok(MoodEntry::from_persistence(MoodEntryData {
        id: MoodEntryId::from_uuid(row.id.parse().unwrap()),
        user_id: UserId::from_uuid(row.user_id.parse().unwrap()),
        mood: Mood::try_from(row.mood as u8)?,
        logged_at: row.logged_at.parse().unwrap(),
        activities: activity_ids
            .into_iter()
            .map(|id| ActivityId::from_uuid(id.parse().unwrap()))
            .collect(),
        content: row.content.map(Content::from_persistence),
        photos: photo_ids
            .into_iter()
            .map(|id| PhotoId::from_uuid(id.parse().unwrap()))
            .collect(),
        voice_memos: voice_memo_ids
            .into_iter()
            .map(|id| VoiceMemoId::from_uuid(id.parse().unwrap()))
            .collect(),
        created_at: row.created_at.parse().unwrap(),
        updated_at: row.updated_at.parse().unwrap(),
    }))
}

pub async fn hydrate_single(pool: &SqlitePool, row: EntryRow) -> Result<MoodEntry, DomainError> {
    let entry_id = row.id.clone();

    let activities: Vec<RelationRow> = sqlx::query_as(
        "SELECT entry_id, activity_id AS related_id FROM entry_activities WHERE entry_id = ?",
    )
    .bind(&entry_id)
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    let photos: Vec<RelationRow> = sqlx::query_as(
        "SELECT entry_id, photo_id AS related_id FROM entry_photos WHERE entry_id = ?",
    )
    .bind(&entry_id)
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    let voice_memos: Vec<RelationRow> = sqlx::query_as(
        "SELECT entry_id, voice_memo_id AS related_id FROM entry_voice_memos WHERE entry_id = ?",
    )
    .bind(&entry_id)
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    row_to_entry(
        row,
        activities.into_iter().map(|r| r.related_id).collect(),
        photos.into_iter().map(|r| r.related_id).collect(),
        voice_memos.into_iter().map(|r| r.related_id).collect(),
    )
}

pub async fn hydrate_batch(
    pool: &SqlitePool,
    rows: Vec<EntryRow>,
) -> Result<Vec<MoodEntry>, DomainError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let entry_ids: Vec<String> = rows.iter().map(|r| r.id.clone()).collect();
    let placeholders = vec!["?"; entry_ids.len()].join(",");

    let activities = batch_load(
        pool,
        &format!("SELECT entry_id, activity_id AS related_id FROM entry_activities WHERE entry_id IN ({placeholders})"),
        &entry_ids,
    ).await?;

    let photos = batch_load(
        pool,
        &format!("SELECT entry_id, photo_id AS related_id FROM entry_photos WHERE entry_id IN ({placeholders})"),
        &entry_ids,
    ).await?;

    let voice_memos = batch_load(
        pool,
        &format!("SELECT entry_id, voice_memo_id AS related_id FROM entry_voice_memos WHERE entry_id IN ({placeholders})"),
        &entry_ids,
    ).await?;

    let mut entries = Vec::with_capacity(rows.len());
    for row in rows {
        let id = row.id.clone();
        entries.push(row_to_entry(
            row,
            activities.get(&id).cloned().unwrap_or_default(),
            photos.get(&id).cloned().unwrap_or_default(),
            voice_memos.get(&id).cloned().unwrap_or_default(),
        )?);
    }
    Ok(entries)
}

async fn batch_load(
    pool: &SqlitePool,
    sql: &str,
    entry_ids: &[String],
) -> Result<HashMap<String, Vec<String>>, DomainError> {
    let mut query = sqlx::query_as::<_, RelationRow>(sqlx::AssertSqlSafe(sql));
    for id in entry_ids {
        query = query.bind(id);
    }

    let rows = query.fetch_all(pool).await.map_err(db_err)?;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        map.entry(row.entry_id).or_default().push(row.related_id);
    }
    Ok(map)
}
