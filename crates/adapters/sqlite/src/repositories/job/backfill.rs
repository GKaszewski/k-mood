use sqlx::SqlitePool;

use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::ports::UnidentifiedSong;
use domain::song::RecordingId;
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqliteRecordingBackfillRepository {
    pool: SqlitePool,
}

impl SqliteRecordingBackfillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct UnidentifiedSongRow {
    entry_id: String,
    user_id: String,
    title: String,
    artist: String,
}

#[async_trait::async_trait]
impl domain::ports::RecordingBackfillQueryPort for SqliteRecordingBackfillRepository {
    async fn find_songs_without_a_recording(
        &self,
        most: usize,
    ) -> Result<Vec<UnidentifiedSong>, DomainError> {
        let rows: Vec<UnidentifiedSongRow> = sqlx::query_as(
            "SELECT s.entry_id, e.user_id, s.title, s.artist
             FROM entry_song s
             JOIN mood_entries e ON e.id = s.entry_id
             WHERE s.recording_id IS NULL
             ORDER BY e.logged_at DESC
             LIMIT ?",
        )
        .bind(most_as_limit(most))
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }

    async fn find_song_without_a_recording(
        &self,
        entry_id: &MoodEntryId,
    ) -> Result<Option<UnidentifiedSong>, DomainError> {
        let row: Option<UnidentifiedSongRow> = sqlx::query_as(
            "SELECT s.entry_id, e.user_id, s.title, s.artist
             FROM entry_song s
             JOIN mood_entries e ON e.id = s.entry_id
             WHERE s.recording_id IS NULL AND s.entry_id = ?",
        )
        .bind(entry_id.value().to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.as_ref().and_then(readable))
    }

    async fn record_identity(
        &self,
        entry_id: &MoodEntryId,
        recording_id: &RecordingId,
    ) -> Result<(), DomainError> {
        sqlx::query("UPDATE entry_song SET recording_id = ? WHERE entry_id = ?")
            .bind(recording_id.value().to_string())
            .bind(entry_id.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}

fn most_as_limit(most: usize) -> i64 {
    i64::try_from(most).unwrap_or(i64::MAX)
}

fn readable(row: &UnidentifiedSongRow) -> Option<UnidentifiedSong> {
    Some(UnidentifiedSong {
        entry_id: MoodEntryId::from_uuid(row.entry_id.parse().ok()?),
        user_id: UserId::from_uuid(row.user_id.parse().ok()?),
        title: row.title.clone(),
        artist: row.artist.clone(),
    })
}
