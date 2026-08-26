use crate::errors::DomainError;

use super::{AlbumName, ArtistName, RecordingId, SongTitle};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Song {
    title: SongTitle,
    artist: ArtistName,
    album: Option<AlbumName>,
    recording_id: Option<RecordingId>,
}

impl Song {
    pub fn new(
        title: impl Into<String>,
        artist: impl Into<String>,
        album: Option<String>,
        recording_id: Option<RecordingId>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            title: SongTitle::new(title)?,
            artist: ArtistName::new(artist)?,
            album: album.and_then(|name| AlbumName::new(name).ok()),
            recording_id,
        })
    }

    pub fn from_persistence(
        title: SongTitle,
        artist: ArtistName,
        album: Option<AlbumName>,
        recording_id: Option<RecordingId>,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            recording_id,
        }
    }
}

impl Song {
    pub fn title(&self) -> &SongTitle {
        &self.title
    }

    pub fn artist(&self) -> &ArtistName {
        &self.artist
    }

    pub fn album(&self) -> Option<&AlbumName> {
        self.album.as_ref()
    }

    pub fn recording_id(&self) -> Option<&RecordingId> {
        self.recording_id.as_ref()
    }
}
