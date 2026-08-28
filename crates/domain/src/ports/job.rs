use crate::errors::DomainError;
use crate::job::{Job, JobId, JobKind, JobSubject};
use crate::user::UserId;

#[async_trait::async_trait]
pub trait JobQueueCommandPort: Send + Sync {
    async fn enqueue(&self, kind: JobKind, subject: &JobSubject) -> Result<bool, DomainError>;

    async fn claim(&self, kind: JobKind, most: usize) -> Result<Vec<Job>, DomainError>;

    async fn finish(&self, id: &JobId) -> Result<(), DomainError>;

    async fn release(&self, id: &JobId, reason: &str) -> Result<(), DomainError>;

    async fn exhaust(&self, id: &JobId, reason: &str) -> Result<(), DomainError>;

    async fn reclaim_stalled(&self, stalled_after_seconds: i64) -> Result<u64, DomainError>;
}

#[async_trait::async_trait]
pub trait JobQueueQueryPort: Send + Sync {
    async fn find_exhausted(&self, most: usize) -> Result<Vec<Job>, DomainError>;
}

#[async_trait::async_trait]
pub trait WeatherLookupPort: Send + Sync {
    fn provider(&self) -> &crate::provider::ProviderName;

    async fn observed_at(
        &self,
        coordinates: &crate::location::Coordinates,
        instant: &chrono::DateTime<chrono::FixedOffset>,
    ) -> Result<Option<crate::weather::Weather>, DomainError>;
}

pub struct UnwatchedPlace {
    pub entry_id: crate::entry::MoodEntryId,
    pub coordinates: crate::location::Coordinates,
    pub logged_at: chrono::DateTime<chrono::FixedOffset>,
}

impl UnwatchedPlace {
    pub fn subject(&self) -> JobSubject {
        JobSubject::Entry(self.entry_id.clone())
    }
}

#[async_trait::async_trait]
pub trait WeatherBacklogQueryPort: Send + Sync {
    async fn find_places_without_weather(
        &self,
        most: usize,
    ) -> Result<Vec<UnwatchedPlace>, DomainError>;

    async fn find_place_without_weather(
        &self,
        entry_id: &crate::entry::MoodEntryId,
    ) -> Result<Option<UnwatchedPlace>, DomainError>;
}

#[async_trait::async_trait]
pub trait RecordingBackfillQueryPort: Send + Sync {
    async fn find_songs_without_a_recording(
        &self,
        most: usize,
    ) -> Result<Vec<UnidentifiedSong>, DomainError>;

    async fn find_song_without_a_recording(
        &self,
        entry_id: &crate::entry::MoodEntryId,
    ) -> Result<Option<UnidentifiedSong>, DomainError>;

    async fn record_identity(
        &self,
        entry_id: &crate::entry::MoodEntryId,
        recording_id: &crate::song::RecordingId,
    ) -> Result<(), DomainError>;
}

pub struct UnidentifiedSong {
    pub entry_id: crate::entry::MoodEntryId,
    pub user_id: UserId,
    pub title: String,
    pub artist: String,
}

impl UnidentifiedSong {
    pub fn subject(&self) -> JobSubject {
        JobSubject::Entry(self.entry_id.clone())
    }
}
