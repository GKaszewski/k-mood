use chrono::Utc;
use sqlx::SqlitePool;

use domain::attachment::{MediaKind, MediaRef, PhotoId, VoiceMemoId};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::db_err;

pub struct SqliteMediaOwnershipRepository {
    pool: SqlitePool,
}

impl SqliteMediaOwnershipRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::MediaOwnershipPort for SqliteMediaOwnershipRepository {
    async fn remember(&self, owner: &UserId, media: MediaRef) -> Result<(), DomainError> {
        sqlx::query(
            "INSERT INTO media_owners (kind, media_id, user_id, created_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(kind, media_id) DO UPDATE SET user_id = excluded.user_id",
        )
        .bind(media.kind().name())
        .bind(media.id().to_string())
        .bind(owner.value().to_string())
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(())
    }

    async fn owner_of(&self, media: MediaRef) -> Result<Option<UserId>, DomainError> {
        let found: Option<(String,)> =
            sqlx::query_as("SELECT user_id FROM media_owners WHERE kind = ? AND media_id = ?")
                .bind(media.kind().name())
                .bind(media.id().to_string())
                .fetch_optional(&self.pool)
                .await
                .map_err(db_err)?;

        Ok(found
            .and_then(|(user_id,)| user_id.parse().ok())
            .map(UserId::from_uuid))
    }

    async fn owned_by(&self, owner: &UserId) -> Result<Vec<MediaRef>, DomainError> {
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT kind, media_id FROM media_owners WHERE user_id = ?")
                .bind(owner.value().to_string())
                .fetch_all(&self.pool)
                .await
                .map_err(db_err)?;

        Ok(rows.iter().filter_map(readable).collect())
    }

    async fn forget(&self, media: MediaRef) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM media_owners WHERE kind = ? AND media_id = ?")
            .bind(media.kind().name())
            .bind(media.id().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }

    async fn forget_all_by_user(&self, owner: &UserId) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM media_owners WHERE user_id = ?")
            .bind(owner.value().to_string())
            .execute(&self.pool)
            .await
            .map_err(db_err)?;

        Ok(())
    }
}

fn readable(row: &(String, String)) -> Option<MediaRef> {
    let (kind, media_id) = row;

    match MediaKind::from_name(kind)? {
        MediaKind::Photo => Some(MediaRef::from(&PhotoId::from_uuid(media_id.parse().ok()?))),
        MediaKind::VoiceMemo => Some(MediaRef::from(&VoiceMemoId::from_uuid(
            media_id.parse().ok()?,
        ))),
    }
}
