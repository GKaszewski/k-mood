use crate::errors::DomainError;
use crate::song::{RecordingId, Song};

#[async_trait::async_trait]
pub trait NowPlayingPort: Send + Sync {
    fn provider(&self) -> &str;

    async fn now_playing(&self, credential: &[u8]) -> Result<Option<Song>, DomainError>;
}

#[async_trait::async_trait]
pub trait RecordingLookupPort: Send + Sync {
    async fn find_recording(
        &self,
        title: &str,
        artist: &str,
    ) -> Result<Option<RecordingId>, DomainError>;
}
