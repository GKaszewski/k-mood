use std::sync::Arc;
use std::time::Duration;

use application::job::use_cases::{run_due_jobs, sweep_recording_backlog, sweep_weather_backlog};
use application::reminder::use_cases::process_due_reminders;
use config::WorkerConfig;
use domain::ports::{
    EntryDimensionPort, JobQueueCommandPort, RecordingBackfillQueryPort, RecordingLookupPort,
    RefreshSessionCommandPort, ReminderQueryPort, ReminderSenderPort, UserQueryPort,
    WeatherBacklogQueryPort, WeatherLookupPort,
};

pub fn every(seconds: u64) -> tokio::time::Interval {
    let mut interval = tokio::time::interval(Duration::from_secs(seconds.max(1)));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    interval
}

pub struct QueueDependencies {
    pub queue: Arc<dyn JobQueueCommandPort>,
    pub backfill: Arc<dyn RecordingBackfillQueryPort>,
    pub recordings: Arc<dyn RecordingLookupPort>,
    pub places: Arc<dyn WeatherBacklogQueryPort>,
    pub weather: Option<Arc<dyn WeatherLookupPort>>,
    pub weather_store: Arc<dyn EntryDimensionPort>,
}

pub async fn work_the_queue(held: QueueDependencies, worker: WorkerConfig) {
    let queue = held.queue.clone();
    let deps = run_due_jobs::Deps {
        queue: held.queue,
        backfill: held.backfill,
        recordings: held.recordings,
        places: held.places,
        weather: held.weather,
        weather_store: held.weather_store,
    };

    let mut ticks = every(worker.poll_seconds);

    loop {
        ticks.tick().await;

        match queue.reclaim_stalled(worker.stalled_after_seconds).await {
            Ok(reclaimed) if reclaimed > 0 => {
                tracing::info!(reclaimed, "reclaimed jobs left running by a stopped worker")
            }
            Ok(_) => {}
            Err(error) => tracing::error!(%error, "could not reclaim stalled jobs"),
        }

        match run_due_jobs::execute(worker.jobs_per_poll, worker.most_attempts, &deps).await {
            Ok(worked) if worked.finished + worked.failed > 0 => {
                tracing::info!(
                    finished = worked.finished,
                    failed = worked.failed,
                    "worked the queue"
                )
            }
            Ok(_) => {}
            Err(error) => tracing::error!(%error, "could not work the queue"),
        }
    }
}

pub async fn sweep_for_work(
    recordings: Arc<dyn RecordingBackfillQueryPort>,
    places: Option<Arc<dyn WeatherBacklogQueryPort>>,
    queue: Arc<dyn JobQueueCommandPort>,
    worker: WorkerConfig,
) {
    let for_recordings = sweep_recording_backlog::Deps {
        backlog: recordings,
        queue: queue.clone(),
    };

    let for_weather = places.map(|backlog| sweep_weather_backlog::Deps { backlog, queue });

    if for_weather.is_none() {
        tracing::info!("not sweeping for weather, because weather cannot be looked up");
    }

    let mut ticks = every(worker.sweep_seconds);

    loop {
        ticks.tick().await;

        if let Err(error) =
            sweep_recording_backlog::execute(worker.enqueued_per_sweep, &for_recordings).await
        {
            tracing::error!(%error, "could not sweep for songs with no recording identity");
        }

        let Some(for_weather) = &for_weather else {
            continue;
        };

        if let Err(error) =
            sweep_weather_backlog::execute(worker.enqueued_per_sweep, for_weather).await
        {
            tracing::error!(%error, "could not sweep for places with no weather");
        }
    }
}

pub async fn send_due_reminders(
    reminder_query: Arc<dyn ReminderQueryPort>,
    user_query: Arc<dyn UserQueryPort>,
    sender: Arc<dyn ReminderSenderPort>,
    worker: WorkerConfig,
) {
    let deps = process_due_reminders::Deps {
        reminder_query,
        user_query,
        sender,
    };

    let mut ticks = every(worker.reminder_seconds);

    loop {
        ticks.tick().await;

        match process_due_reminders::execute(&deps).await {
            Ok(sent) if sent > 0 => tracing::info!(sent, "reminders sent"),
            Ok(_) => {}
            Err(error) => tracing::error!(%error, "could not process due reminders"),
        }
    }
}

pub async fn clear_expired_sessions(
    sessions: Arc<dyn RefreshSessionCommandPort>,
    worker: WorkerConfig,
) {
    let mut ticks = every(worker.session_cleanup_seconds);

    loop {
        ticks.tick().await;

        match sessions.delete_expired().await {
            Ok(deleted) if deleted > 0 => tracing::info!(deleted, "expired sessions cleared"),
            Ok(_) => {}
            Err(error) => tracing::error!(%error, "could not clear expired sessions"),
        }
    }
}
