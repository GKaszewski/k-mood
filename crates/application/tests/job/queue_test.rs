use std::sync::Arc;

use domain::entry::{Mood, MoodEntry, MoodEntryId};
use domain::job::{JobKind, JobStatus};
use domain::ports::{JobQueueCommandPort, JobQueueQueryPort};
use domain::song::RecordingId;
use domain::testing::{FakeRecordingLookup, InMemoryStore};
use domain::user::UserId;

use application::job::use_cases::{run_due_jobs, sweep_recording_backlog, sweep_weather_backlog};

const MOST_ATTEMPTS: u32 = 3;

struct Bench {
    store: Arc<InMemoryStore>,
    lookups: Arc<FakeRecordingLookup>,
}

fn a_bench() -> Bench {
    Bench {
        store: Arc::new(InMemoryStore::new()),
        lookups: Arc::new(FakeRecordingLookup::finding(None)),
    }
}

impl Bench {
    async fn an_unidentified_song(&self, title: &str) -> MoodEntryId {
        let entry = MoodEntry::new(UserId::generate(), Mood::Good, test_instant());
        domain::ports::MoodEntryCommandPort::save(self.store.as_ref(), &entry)
            .await
            .unwrap();

        self.store
            .put_unidentified_song(entry.id(), title, "Massive Attack");

        entry.id().clone()
    }

    async fn sweep(&self) -> usize {
        let deps = sweep_recording_backlog::Deps {
            backlog: self.store.clone(),
            queue: self.store.clone(),
        };

        sweep_recording_backlog::execute(200, &deps).await.unwrap()
    }

    async fn work(&self) -> run_due_jobs::Worked {
        let deps = run_due_jobs::Deps {
            queue: self.store.clone(),
            backfill: self.store.clone(),
            recordings: self.lookups.clone(),
            places: self.store.clone(),
            weather: None,
            weather_store: Arc::new(domain::testing::InMemoryDimensionStore::new(
                domain::dimension::DimensionKind::Weather,
            )),
        };

        run_due_jobs::execute(10, MOST_ATTEMPTS, &deps)
            .await
            .unwrap()
    }

    async fn queued(&self) -> Vec<domain::job::Job> {
        self.store.every_job()
    }

    async fn exhausted(&self) -> Vec<domain::job::Job> {
        JobQueueQueryPort::find_exhausted(self.store.as_ref(), 50)
            .await
            .unwrap()
    }
}

fn test_instant() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::DateTime::parse_from_rfc3339("2026-08-20T21:30:00+02:00").unwrap()
}

#[tokio::test]
async fn a_song_with_no_recording_identity_is_swept_onto_the_queue() {
    let bench = a_bench();
    bench.an_unidentified_song("Teardrop").await;

    let enqueued = bench.sweep().await;

    assert_eq!(enqueued, 1);
    let queued = bench.queued().await;
    assert_eq!(queued.len(), 1);
    assert_eq!(queued[0].kind(), JobKind::BackfillRecordingIdentity);
    assert_eq!(queued[0].status(), JobStatus::Pending);
}

#[tokio::test]
async fn sweeping_twice_does_not_queue_the_same_work_twice() {
    let bench = a_bench();
    bench.an_unidentified_song("Teardrop").await;

    bench.sweep().await;
    let again = bench.sweep().await;

    assert_eq!(again, 0, "the work was already queued");
    assert_eq!(bench.queued().await.len(), 1);
}

#[tokio::test]
async fn the_sweep_is_bounded_by_what_it_is_asked_for() {
    let bench = a_bench();
    for number in 0..10 {
        bench.an_unidentified_song(&format!("song {number}")).await;
    }

    let deps = sweep_recording_backlog::Deps {
        backlog: bench.store.clone(),
        queue: bench.store.clone(),
    };
    let enqueued = sweep_recording_backlog::execute(4, &deps).await.unwrap();

    assert_eq!(enqueued, 4);
}

#[tokio::test]
async fn a_job_that_succeeds_leaves_the_queue_and_records_the_identity() {
    let bench = a_bench();
    let found = RecordingId::new("8f3471b5-7e6a-4dbe-9c6b-1e56a5ed2f6d").unwrap();
    let bench = Bench {
        lookups: Arc::new(FakeRecordingLookup::finding(Some(found.clone()))),
        ..bench
    };
    let entry_id = bench.an_unidentified_song("Teardrop").await;
    bench.sweep().await;

    let worked = bench.work().await;

    assert_eq!(worked.finished, 1);
    assert_eq!(worked.failed, 0);
    assert!(bench.queued().await.is_empty(), "a finished job is gone");
    assert_eq!(bench.store.recording_of(&entry_id), Some(found));
}

#[tokio::test]
async fn a_lookup_that_finds_nothing_still_finishes_the_job() {
    let bench = a_bench();
    bench.an_unidentified_song("Teardrop").await;
    bench.sweep().await;

    let worked = bench.work().await;

    assert_eq!(worked.finished, 1, "nothing to find is not a failure");
    assert!(bench.queued().await.is_empty());
}

#[tokio::test]
async fn a_failing_job_goes_back_to_the_queue_with_the_reason_recorded() {
    let bench = Bench {
        lookups: Arc::new(FakeRecordingLookup::failing("musicbrainz is unreachable")),
        ..a_bench()
    };
    bench.an_unidentified_song("Teardrop").await;
    bench.sweep().await;

    let worked = bench.work().await;

    assert_eq!(worked.failed, 1);
    let queued = bench.queued().await;
    assert_eq!(queued[0].status(), JobStatus::Pending);
    assert_eq!(queued[0].attempts(), 1);
    assert!(
        queued[0].last_error().unwrap().contains("unreachable"),
        "got {:?}",
        queued[0].last_error()
    );
}

#[tokio::test]
async fn a_job_that_keeps_failing_stops_being_retried_but_stays_visible() {
    let bench = Bench {
        lookups: Arc::new(FakeRecordingLookup::failing("musicbrainz is unreachable")),
        ..a_bench()
    };
    bench.an_unidentified_song("Teardrop").await;
    bench.sweep().await;

    for _ in 0..MOST_ATTEMPTS {
        bench.work().await;
    }

    let after_giving_up = bench.work().await;

    assert_eq!(
        after_giving_up.finished + after_giving_up.failed,
        0,
        "an exhausted job is not claimed again"
    );

    let exhausted = bench.exhausted().await;
    assert_eq!(exhausted.len(), 1);
    assert_eq!(exhausted[0].attempts(), MOST_ATTEMPTS);
    assert!(exhausted[0].last_error().is_some());
}

#[tokio::test]
async fn work_lost_to_a_crash_is_found_again_by_the_sweep() {
    let bench = a_bench();
    bench.an_unidentified_song("Teardrop").await;
    bench.sweep().await;

    bench.store.lose_every_job();
    assert!(bench.queued().await.is_empty(), "the queue was wiped");

    let enqueued = bench.sweep().await;

    assert_eq!(
        enqueued, 1,
        "the sweep rediscovered it from the entry itself"
    );
}

#[tokio::test]
async fn a_job_left_running_by_a_dead_worker_is_reclaimed() {
    let bench = a_bench();
    bench.an_unidentified_song("Teardrop").await;
    bench.sweep().await;

    JobQueueCommandPort::claim(bench.store.as_ref(), JobKind::BackfillRecordingIdentity, 10)
        .await
        .unwrap();
    assert_eq!(bench.queued().await[0].status(), JobStatus::Running);

    let reclaimed = JobQueueCommandPort::reclaim_stalled(bench.store.as_ref(), 0)
        .await
        .unwrap();

    assert_eq!(reclaimed, 1);
    assert_eq!(bench.queued().await[0].status(), JobStatus::Pending);
}

use domain::dimension::DimensionKind;
use domain::location::Coordinates;
use domain::provider::ProviderName;
use domain::testing::{FakeWeatherLookup, InMemoryDimensionStore};
use domain::weather::{Celsius, Condition, Weather};

struct WeatherBench {
    store: Arc<InMemoryStore>,
    lookup: Arc<FakeWeatherLookup>,
    weather_store: Arc<InMemoryDimensionStore>,
    switched_on: bool,
}

fn a_downpour() -> Weather {
    Weather::new(
        Condition::Rain,
        Celsius::new(11.5).unwrap(),
        ProviderName::new("open-meteo").unwrap(),
    )
}

fn a_weather_bench(lookup: FakeWeatherLookup, switched_on: bool) -> WeatherBench {
    WeatherBench {
        store: Arc::new(InMemoryStore::new()),
        lookup: Arc::new(lookup),
        weather_store: Arc::new(InMemoryDimensionStore::new(DimensionKind::Weather)),
        switched_on,
    }
}

impl WeatherBench {
    async fn a_place_with_no_weather(&self) -> MoodEntryId {
        let entry = MoodEntry::new(UserId::generate(), Mood::Good, test_instant());
        domain::ports::MoodEntryCommandPort::save(self.store.as_ref(), &entry)
            .await
            .unwrap();

        self.store.put_place_without_weather(
            entry.id(),
            Coordinates::new(52.2297, 21.0122).unwrap(),
            test_instant(),
        );

        entry.id().clone()
    }

    async fn sweep(&self) -> usize {
        let deps = sweep_weather_backlog::Deps {
            backlog: self.store.clone(),
            queue: self.store.clone(),
        };

        sweep_weather_backlog::execute(200, &deps).await.unwrap()
    }

    async fn work(&self) -> run_due_jobs::Worked {
        let deps = run_due_jobs::Deps {
            queue: self.store.clone(),
            backfill: self.store.clone(),
            recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
            places: self.store.clone(),
            weather: self
                .switched_on
                .then(|| self.lookup.clone() as Arc<dyn domain::ports::WeatherLookupPort>),
            weather_store: self.weather_store.clone(),
        };

        run_due_jobs::execute(10, MOST_ATTEMPTS, &deps)
            .await
            .unwrap()
    }

    async fn stored_weather(&self, entry_id: &MoodEntryId) -> Option<Weather> {
        let held = domain::ports::EntryDimensionPort::load(
            self.weather_store.as_ref(),
            std::slice::from_ref(entry_id),
        )
        .await
        .unwrap();

        match held.get(entry_id) {
            Some(domain::dimension::DimensionValue::Weather(weather)) => Some(weather.clone()),
            _ => None,
        }
    }
}

#[tokio::test]
async fn a_place_with_no_weather_is_swept_onto_the_queue_and_observed() {
    let bench = a_weather_bench(FakeWeatherLookup::observing(Some(a_downpour())), true);
    let entry_id = bench.a_place_with_no_weather().await;

    assert_eq!(bench.sweep().await, 1);
    let worked = bench.work().await;

    assert_eq!(worked.finished, 1);
    let observed = bench.stored_weather(&entry_id).await.expect("weather");
    assert_eq!(observed.condition(), Condition::Rain);
    assert_eq!(observed.observed_by().value(), "open-meteo");
}

#[tokio::test]
async fn nothing_leaves_the_box_when_weather_lookups_are_switched_off() {
    let bench = a_weather_bench(FakeWeatherLookup::observing(Some(a_downpour())), false);
    let entry_id = bench.a_place_with_no_weather().await;
    bench.sweep().await;

    let worked = bench.work().await;

    assert_eq!(worked.failed, 1, "the job cannot be done, and says so");
    assert_eq!(
        bench.lookup.times_asked(),
        0,
        "the provider was never asked"
    );
    assert!(bench.stored_weather(&entry_id).await.is_none());
    assert!(
        bench.queued().await[0]
            .last_error()
            .unwrap()
            .contains("switched off"),
        "got {:?}",
        bench.queued().await[0].last_error()
    );
}

#[tokio::test]
async fn a_provider_that_has_no_reading_for_a_place_does_not_fail_the_job() {
    let bench = a_weather_bench(FakeWeatherLookup::observing(None), true);
    let entry_id = bench.a_place_with_no_weather().await;
    bench.sweep().await;

    let worked = bench.work().await;

    assert_eq!(worked.finished, 1);
    assert!(bench.stored_weather(&entry_id).await.is_none());
}

#[tokio::test]
async fn an_unreachable_provider_leaves_the_job_to_be_retried() {
    let bench = a_weather_bench(FakeWeatherLookup::failing("open-meteo timed out"), true);
    bench.a_place_with_no_weather().await;
    bench.sweep().await;

    let worked = bench.work().await;

    assert_eq!(worked.failed, 1);
    let queued = bench.queued().await;
    assert_eq!(queued[0].attempts(), 1);
    assert!(queued[0].last_error().unwrap().contains("timed out"));
}

impl WeatherBench {
    async fn queued(&self) -> Vec<domain::job::Job> {
        self.store.every_job()
    }
}
