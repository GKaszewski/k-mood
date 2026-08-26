use chrono::Utc;

use crate::errors::DomainError;
use crate::job::{Job, JobId, JobKind, JobStatus, JobSubject};
use crate::ports::UnidentifiedSong;

use super::InMemoryStore;

impl InMemoryStore {
    pub fn put_place_without_weather(
        &self,
        entry_id: &crate::entry::MoodEntryId,
        coordinates: crate::location::Coordinates,
        logged_at: chrono::DateTime<chrono::FixedOffset>,
    ) {
        self.places_without_weather.write().unwrap().push((
            entry_id.clone(),
            coordinates,
            logged_at,
        ));
    }

    pub fn every_job(&self) -> Vec<Job> {
        self.jobs.read().unwrap().clone()
    }

    pub fn lose_every_job(&self) {
        self.jobs.write().unwrap().clear();
    }

    pub fn put_unidentified_song(
        &self,
        entry_id: &crate::entry::MoodEntryId,
        title: &str,
        artist: &str,
    ) {
        self.unidentified_songs.write().unwrap().push((
            entry_id.clone(),
            title.to_string(),
            artist.to_string(),
        ));
    }

    pub fn recording_of(
        &self,
        entry_id: &crate::entry::MoodEntryId,
    ) -> Option<crate::song::RecordingId> {
        self.recordings
            .read()
            .unwrap()
            .iter()
            .find(|held| &held.0 == entry_id)
            .map(|held| held.1.clone())
    }
}

#[async_trait::async_trait]
impl crate::ports::JobQueueCommandPort for InMemoryStore {
    async fn enqueue(&self, kind: JobKind, subject: &JobSubject) -> Result<bool, DomainError> {
        let mut queued = self.jobs.write().unwrap();

        let already_here = queued
            .iter()
            .any(|job| job.kind() == kind && job.subject() == subject);

        if already_here {
            return Ok(false);
        }

        queued.push(Job::pending(kind, subject.clone()));

        Ok(true)
    }

    async fn claim(&self, kind: JobKind, most: usize) -> Result<Vec<Job>, DomainError> {
        let mut queued = self.jobs.write().unwrap();
        let mut claimed = Vec::new();

        for job in queued.iter_mut() {
            if claimed.len() == most {
                break;
            }

            if job.kind() != kind || job.status() != JobStatus::Pending {
                continue;
            }

            *job = running(job);
            claimed.push(job.clone());
        }

        Ok(claimed)
    }

    async fn finish(&self, id: &JobId) -> Result<(), DomainError> {
        self.jobs.write().unwrap().retain(|job| job.id() != id);

        Ok(())
    }

    async fn release(&self, id: &JobId, reason: &str) -> Result<(), DomainError> {
        self.rewrite(id, JobStatus::Pending, Some(reason), true);

        Ok(())
    }

    async fn exhaust(&self, id: &JobId, reason: &str) -> Result<(), DomainError> {
        self.rewrite(id, JobStatus::Exhausted, Some(reason), true);

        Ok(())
    }

    async fn reclaim_stalled(&self, stalled_after_seconds: i64) -> Result<u64, DomainError> {
        let mut queued = self.jobs.write().unwrap();
        let mut reclaimed = 0;

        for job in queued.iter_mut() {
            let stalled = (Utc::now() - *job.updated_at()).num_seconds() >= stalled_after_seconds;

            if job.status() == JobStatus::Running && stalled {
                *job = rebuilt(job, JobStatus::Pending, job.last_error(), false);
                reclaimed += 1;
            }
        }

        Ok(reclaimed)
    }
}

#[async_trait::async_trait]
impl crate::ports::JobQueueQueryPort for InMemoryStore {
    async fn find_exhausted(&self, most: usize) -> Result<Vec<Job>, DomainError> {
        Ok(self
            .jobs
            .read()
            .unwrap()
            .iter()
            .filter(|job| job.status() == JobStatus::Exhausted)
            .take(most)
            .cloned()
            .collect())
    }
}

#[async_trait::async_trait]
impl crate::ports::RecordingBackfillQueryPort for InMemoryStore {
    async fn find_songs_without_a_recording(
        &self,
        most: usize,
    ) -> Result<Vec<UnidentifiedSong>, DomainError> {
        let identified: Vec<crate::entry::MoodEntryId> = self
            .recordings
            .read()
            .unwrap()
            .iter()
            .map(|held| held.0.clone())
            .collect();

        Ok(self
            .unidentified_songs
            .read()
            .unwrap()
            .iter()
            .filter(|held| !identified.contains(&held.0))
            .take(most)
            .map(|held| UnidentifiedSong {
                entry_id: held.0.clone(),
                user_id: crate::user::UserId::generate(),
                title: held.1.clone(),
                artist: held.2.clone(),
            })
            .collect())
    }

    async fn record_identity(
        &self,
        entry_id: &crate::entry::MoodEntryId,
        recording_id: &crate::song::RecordingId,
    ) -> Result<(), DomainError> {
        self.recordings
            .write()
            .unwrap()
            .push((entry_id.clone(), recording_id.clone()));

        Ok(())
    }
}

impl InMemoryStore {
    fn rewrite(&self, id: &JobId, status: JobStatus, reason: Option<&str>, counted: bool) {
        let mut queued = self.jobs.write().unwrap();

        if let Some(job) = queued.iter_mut().find(|job| job.id() == id) {
            *job = rebuilt(job, status, reason, counted);
        }
    }
}

fn running(job: &Job) -> Job {
    rebuilt(job, JobStatus::Running, job.last_error(), false)
}

fn rebuilt(job: &Job, status: JobStatus, reason: Option<&str>, counted: bool) -> Job {
    Job::from_persistence(crate::job::JobData {
        id: job.id().clone(),
        kind: job.kind(),
        subject: job.subject().clone(),
        status,
        attempts: job.attempts() + u32::from(counted),
        last_error: reason.map(|reason| reason.to_string()),
        enqueued_at: *job.enqueued_at(),
        updated_at: Utc::now(),
    })
}

#[async_trait::async_trait]
impl crate::ports::WeatherBacklogQueryPort for InMemoryStore {
    async fn find_places_without_weather(
        &self,
        most: usize,
    ) -> Result<Vec<crate::ports::UnwatchedPlace>, DomainError> {
        let watched: Vec<crate::entry::MoodEntryId> = self
            .observed_weather
            .read()
            .unwrap()
            .iter()
            .map(|held| held.0.clone())
            .collect();

        Ok(self
            .places_without_weather
            .read()
            .unwrap()
            .iter()
            .filter(|held| !watched.contains(&held.0))
            .take(most)
            .map(|held| crate::ports::UnwatchedPlace {
                entry_id: held.0.clone(),
                coordinates: held.1,
                logged_at: held.2,
            })
            .collect())
    }
}
