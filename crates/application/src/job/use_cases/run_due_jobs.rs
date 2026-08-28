use std::sync::Arc;

use domain::dimension::DimensionValue;
use domain::job::{Job, JobKind, JobSubject};
use domain::ports::{
    EntryDimensionPort, JobQueueCommandPort, RecordingBackfillQueryPort, RecordingLookupPort,
    UnidentifiedSong, UnwatchedPlace, WeatherBacklogQueryPort, WeatherLookupPort,
};

use crate::errors::ApplicationError;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Worked {
    pub finished: usize,
    pub failed: usize,
}

pub struct Deps {
    pub queue: Arc<dyn JobQueueCommandPort>,
    pub backfill: Arc<dyn RecordingBackfillQueryPort>,
    pub recordings: Arc<dyn RecordingLookupPort>,
    pub places: Arc<dyn WeatherBacklogQueryPort>,
    pub weather: Option<Arc<dyn WeatherLookupPort>>,
    pub weather_store: Arc<dyn EntryDimensionPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    most: usize,
    most_attempts: u32,
    deps: &Deps,
) -> Result<Worked, ApplicationError> {
    let mut claimed = Vec::new();

    for kind in JobKind::ALL {
        claimed.extend(deps.queue.claim(kind, most).await?);
    }

    let mut worked = Worked::default();

    for job in claimed {
        match attempt(&job, deps).await {
            Ok(()) => {
                deps.queue.finish(job.id()).await?;
                worked.finished += 1;
            }
            Err(reason) => {
                give_up_or_retry(&job, &reason, most_attempts, deps).await?;
                worked.failed += 1;
            }
        }
    }

    Ok(worked)
}

async fn give_up_or_retry(
    job: &Job,
    reason: &str,
    most_attempts: u32,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let attempted = job.attempts() + 1;

    if attempted < most_attempts {
        deps.queue.release(job.id(), reason).await?;

        return Ok(());
    }

    tracing::warn!(
        job_id = %job.id(),
        kind = job.kind().name(),
        attempts = attempted,
        reason,
        "giving up on a job; it stays visible so it can be retried deliberately"
    );

    deps.queue.exhaust(job.id(), reason).await?;

    Ok(())
}

async fn attempt(job: &Job, deps: &Deps) -> Result<(), String> {
    match job.kind() {
        JobKind::BackfillRecordingIdentity => backfill_recording_identity(job, deps).await,
        JobKind::ObserveWeather => observe_weather(job, deps).await,
    }
}

async fn observe_weather(job: &Job, deps: &Deps) -> Result<(), String> {
    let JobSubject::Entry(entry_id) = job.subject();

    let Some(lookup) = &deps.weather else {
        return Err("weather lookups are switched off".into());
    };

    let found = deps
        .places
        .find_place_without_weather(entry_id)
        .await
        .map_err(|error| error.to_string())?;

    let Some(place) = found else {
        return Ok(());
    };

    let observed = lookup
        .observed_at(&place.coordinates, &place.logged_at)
        .await
        .map_err(|error| error.to_string())?;

    let Some(weather) = observed else {
        tracing::debug!(entry_id = %entry_id, "the provider had no weather for this place and time");

        return Ok(());
    };

    remember_weather(&place, weather, deps).await
}

async fn remember_weather(
    place: &UnwatchedPlace,
    weather: domain::weather::Weather,
    deps: &Deps,
) -> Result<(), String> {
    deps.weather_store
        .save(&place.entry_id, &[DimensionValue::Weather(weather)])
        .await
        .map_err(|error| error.to_string())
}

async fn backfill_recording_identity(job: &Job, deps: &Deps) -> Result<(), String> {
    let JobSubject::Entry(entry_id) = job.subject();

    let found = deps
        .backfill
        .find_song_without_a_recording(entry_id)
        .await
        .map_err(|error| error.to_string())?;

    let Some(song) = found else {
        return Ok(());
    };

    let found = deps
        .recordings
        .find_recording(&song.title, &song.artist)
        .await
        .map_err(|error| error.to_string())?;

    let Some(recording_id) = found else {
        tracing::debug!(title = %song.title, "no recording matched this song");

        return Ok(());
    };

    remember(&song, &recording_id, deps).await
}

async fn remember(
    song: &UnidentifiedSong,
    recording_id: &domain::song::RecordingId,
    deps: &Deps,
) -> Result<(), String> {
    deps.backfill
        .record_identity(&song.entry_id, recording_id)
        .await
        .map_err(|error| error.to_string())
}
