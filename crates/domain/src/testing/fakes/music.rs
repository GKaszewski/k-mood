use crate::errors::DomainError;
use crate::ports::{NowPlayingPort, RecordingLookupPort};
use crate::song::{RecordingId, Song};

pub struct FakeNowPlaying {
    provider: String,
    playing: Option<Song>,
    failing_with: Option<String>,
}

impl FakeNowPlaying {
    pub fn playing(provider: &str, song: Song) -> Self {
        Self {
            provider: provider.to_string(),
            playing: Some(song),
            failing_with: None,
        }
    }

    pub fn silent(provider: &str) -> Self {
        Self {
            provider: provider.to_string(),
            playing: None,
            failing_with: None,
        }
    }

    pub fn failing(provider: &str, reason: impl Into<String>) -> Self {
        Self {
            provider: provider.to_string(),
            playing: None,
            failing_with: Some(reason.into()),
        }
    }
}

#[async_trait::async_trait]
impl NowPlayingPort for FakeNowPlaying {
    fn provider(&self) -> &str {
        &self.provider
    }

    async fn now_playing(&self, _credential: &[u8]) -> Result<Option<Song>, DomainError> {
        if let Some(reason) = &self.failing_with {
            return Err(DomainError::InvalidInput(reason.clone()));
        }

        Ok(self.playing.clone())
    }
}

pub struct FakeRecordingLookup {
    found: Option<RecordingId>,
    failing_with: Option<String>,
}

impl FakeRecordingLookup {
    pub fn finding(found: Option<RecordingId>) -> Self {
        Self {
            found,
            failing_with: None,
        }
    }

    pub fn finding_nothing() -> Self {
        Self::finding(None)
    }

    pub fn failing(reason: impl Into<String>) -> Self {
        Self {
            found: None,
            failing_with: Some(reason.into()),
        }
    }
}

#[async_trait::async_trait]
impl RecordingLookupPort for FakeRecordingLookup {
    async fn find_recording(
        &self,
        _title: &str,
        _artist: &str,
    ) -> Result<Option<RecordingId>, DomainError> {
        if let Some(reason) = &self.failing_with {
            return Err(DomainError::InvalidInput(reason.clone()));
        }

        Ok(self.found.clone())
    }
}
