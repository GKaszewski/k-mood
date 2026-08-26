use std::collections::HashMap;

use sqlx::SqlitePool;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::MoodEntryId;
use domain::errors::DomainError;
use domain::song::{AlbumName, ArtistName, RecordingId, Song, SongTitle};

use super::super::shared::db_err;

pub struct SqliteSongDimensionRepository {
    pool: SqlitePool,
}

impl SqliteSongDimensionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct SongRow {
    entry_id: String,
    title: String,
    artist: String,
    album: Option<String>,
    recording_id: Option<String>,
}

#[async_trait::async_trait]
impl domain::ports::EntryDimensionPort for SqliteSongDimensionRepository {
    async fn load(
        &self,
        entry_ids: &[MoodEntryId],
    ) -> Result<HashMap<MoodEntryId, DimensionValue>, DomainError> {
        if entry_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = vec!["?"; entry_ids.len()].join(",");
        let sql = format!(
            "SELECT entry_id, title, artist, album, recording_id FROM entry_song WHERE entry_id IN ({placeholders})"
        );

        let mut query = sqlx::query_as::<_, SongRow>(sqlx::AssertSqlSafe(sql));
        for id in entry_ids {
            query = query.bind(id.value().to_string());
        }

        let rows = query.fetch_all(&self.pool).await.map_err(db_err)?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let entry_id = row.entry_id.parse().ok()?;
                let song = Song::from_persistence(
                    SongTitle::from_persistence(row.title),
                    ArtistName::from_persistence(row.artist),
                    row.album.map(AlbumName::from_persistence),
                    row.recording_id
                        .and_then(|id| id.parse().ok())
                        .map(RecordingId::from_uuid),
                );
                Some((MoodEntryId::from_uuid(entry_id), DimensionValue::Song(song)))
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
            .find(|value| value.kind() == DimensionKind::Song)
        {
            Some(DimensionValue::Song(song)) => {
                sqlx::query(
                    "INSERT INTO entry_song (entry_id, title, artist, album, recording_id)
                     VALUES (?, ?, ?, ?, ?)
                     ON CONFLICT(entry_id) DO UPDATE SET
                        title = excluded.title, artist = excluded.artist,
                        album = excluded.album, recording_id = excluded.recording_id",
                )
                .bind(&id)
                .bind(song.title().value())
                .bind(song.artist().value())
                .bind(song.album().map(|album| album.value().to_string()))
                .bind(song.recording_id().map(|id| id.value().to_string()))
                .execute(&self.pool)
                .await
                .map_err(db_err)?;
            }
            _ => {
                sqlx::query("DELETE FROM entry_song WHERE entry_id = ?")
                    .bind(&id)
                    .execute(&self.pool)
                    .await
                    .map_err(db_err)?;
            }
        }

        Ok(())
    }
}
