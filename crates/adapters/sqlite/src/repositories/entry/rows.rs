use sqlx::SqlitePool;

use domain::entry::{Mood, MoodEntry, MoodEntryData, MoodEntryId};
use domain::errors::DomainError;
use domain::user::UserId;

#[derive(sqlx::FromRow)]
pub struct EntryRow {
    pub id: String,
    pub user_id: String,
    pub mood: i32,
    pub logged_at: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn row_to_entry(row: EntryRow) -> Result<MoodEntry, DomainError> {
    let parse_failed = || DomainError::InvalidInput("stored entry row is malformed".into());

    Ok(MoodEntry::from_persistence(MoodEntryData {
        id: MoodEntryId::from_uuid(row.id.parse().map_err(|_| parse_failed())?),
        user_id: UserId::from_uuid(row.user_id.parse().map_err(|_| parse_failed())?),
        mood: Mood::try_from(row.mood as u8)?,
        logged_at: row.logged_at.parse().map_err(|_| parse_failed())?,
        created_at: row.created_at.parse().map_err(|_| parse_failed())?,
        updated_at: row.updated_at.parse().map_err(|_| parse_failed())?,
    }))
}

pub async fn hydrate_single(_pool: &SqlitePool, row: EntryRow) -> Result<MoodEntry, DomainError> {
    row_to_entry(row)
}

pub async fn hydrate_batch(
    _pool: &SqlitePool,
    rows: Vec<EntryRow>,
) -> Result<Vec<MoodEntry>, DomainError> {
    rows.into_iter().map(row_to_entry).collect()
}
