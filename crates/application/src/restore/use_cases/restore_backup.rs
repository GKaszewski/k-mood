use std::collections::HashMap;

use std::sync::Arc;

use chrono::{NaiveTime, Weekday};

use domain::activity::{Activity, ActivityId, ActivityName, CategoryName};
use domain::attachment::{ContentType, MediaUpload, PhotoId, VoiceMemoId};
use domain::dimension::DimensionValue;
use domain::entry::{Date, Mood, MoodEntry};
use domain::metric::{DailyMetric, MetricKind, MetricValue, Source};
use domain::ports::{
    ActivityCommandPort, ActivityQueryPort, BackupReaderPort, CycleStartCommandPort,
    DailyMetricCommandPort, EntryDimensionPort, MediaOwnershipPort, MediaStoragePort,
    MoodEntryCommandPort, MoodEntryQueryPort, ReminderCommandPort, ReminderQueryPort,
    RestorableActivity, RestorableContents, RestorableEntry, RestorableMedia, RestorableMetric,
    RestorableReminder, UserPreferencesCommandPort, UserPreferencesQueryPort,
};
use domain::provider::ProviderName;
use domain::reminder::{DaySchedule, Reminder};
use domain::user::UserId;

use crate::errors::ApplicationError;
use crate::import::already_here::AlreadyHere;
use crate::restore::commands::RestoreBackupCommand;
use crate::user::preferences::preferences_of;

const WEEK: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

const PHOTO_FALLBACK_TYPE: &str = "image/jpeg";
const VOICE_MEMO_FALLBACK_TYPE: &str = "audio/webm";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RestoreOutcome {
    pub entries: usize,
    pub metrics: usize,
    pub cycle_starts: usize,
    pub activities: usize,
    pub reminders: usize,
    pub media: usize,
    pub skipped: usize,
    pub unreadable: Vec<String>,
}

pub struct Deps {
    pub reader: Arc<dyn BackupReaderPort>,
    pub entry_command: Arc<dyn MoodEntryCommandPort>,
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub activity_command: Arc<dyn ActivityCommandPort>,
    pub activity_query: Arc<dyn ActivityQueryPort>,
    pub reminder_command: Arc<dyn ReminderCommandPort>,
    pub reminder_query: Arc<dyn ReminderQueryPort>,
    pub metrics: Arc<dyn DailyMetricCommandPort>,
    pub cycles: Arc<dyn CycleStartCommandPort>,
    pub preferences_command: Arc<dyn UserPreferencesCommandPort>,
    pub preferences_query: Arc<dyn UserPreferencesQueryPort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub media_ownership: Arc<dyn MediaOwnershipPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(
    command: RestoreBackupCommand,
    deps: &Deps,
) -> Result<RestoreOutcome, ApplicationError> {
    let contents = deps.reader.read(&command.data).await?;
    let user_id = command.user_id;
    let mut outcome = RestoreOutcome::default();

    let activities = restore_activities(&user_id, &contents.activities, deps, &mut outcome).await?;
    let media = restore_media(&user_id, &contents, deps, &mut outcome).await;
    let renamed = Renamed { activities, media };
    restore_entries(&user_id, &contents.entries, &renamed, deps, &mut outcome).await?;
    restore_metrics(&user_id, &contents.metrics, deps, &mut outcome).await?;
    restore_cycle_starts(&user_id, &contents.cycle_starts, deps, &mut outcome).await?;
    restore_reminders(&user_id, &contents.reminders, deps, &mut outcome).await?;
    restore_preferences(&user_id, contents.tracks_cycle, deps).await?;

    tracing::info!(
        entries = outcome.entries,
        metrics = outcome.metrics,
        cycle_starts = outcome.cycle_starts,
        activities = outcome.activities,
        reminders = outcome.reminders,
        media = outcome.media,
        skipped = outcome.skipped,
        unreadable = outcome.unreadable.len(),
        "restored a backup"
    );

    Ok(outcome)
}

async fn restore_activities(
    user_id: &UserId,
    backed_up: &[RestorableActivity],
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) -> Result<HashMap<String, ActivityId>, ApplicationError> {
    let existing: HashMap<String, ActivityId> = deps
        .activity_query
        .find_by_user(user_id)
        .await?
        .iter()
        .map(|activity| (activity.name().value().to_string(), activity.id().clone()))
        .collect();

    let mut renamed = HashMap::new();

    for held in backed_up {
        if let Some(already_here) = existing.get(&held.name) {
            renamed.insert(held.id.clone(), already_here.clone());
            continue;
        }

        let Ok(name) = ActivityName::new(&held.name) else {
            outcome
                .unreadable
                .push(format!("activity {}: unusable name", held.name));
            continue;
        };

        let category = held
            .category
            .as_ref()
            .and_then(|category| CategoryName::new(category).ok());

        let mut activity = Activity::new(user_id.clone(), name, category);
        if held.archived
            && let Err(error) = activity.archive()
        {
            outcome
                .unreadable
                .push(format!("activity {}: {error}", held.name));
        }

        deps.activity_command.save(&activity).await?;
        renamed.insert(held.id.clone(), activity.id().clone());
        outcome.activities += 1;
    }

    Ok(renamed)
}

async fn restore_entries(
    user_id: &UserId,
    backed_up: &[RestorableEntry],
    renamed: &Renamed,
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) -> Result<(), ApplicationError> {
    let existing = deps.entry_query.find_all_by_user(user_id).await?;
    let mut already_here = AlreadyHere::holding(&existing);

    for held in backed_up {
        let Ok(logged_at) = chrono::DateTime::parse_from_rfc3339(&held.logged_at) else {
            outcome
                .unreadable
                .push(format!("entry at {}: unreadable instant", held.logged_at));
            continue;
        };

        let Ok(mood) = Mood::try_from(held.mood) else {
            outcome
                .unreadable
                .push(format!("entry at {}: unreadable mood", held.logged_at));
            continue;
        };

        if already_here.holds(&logged_at, mood) {
            outcome.skipped += 1;
            continue;
        }
        already_here.remember(&logged_at, mood);

        let entry = MoodEntry::new(user_id.clone(), mood, logged_at);
        deps.entry_command.save(&entry).await?;

        let dimensions = rebuilt_dimensions(&held.dimensions, renamed);
        for port in &deps.dimensions {
            port.save(entry.id(), &dimensions).await?;
        }

        outcome.entries += 1;
    }

    Ok(())
}

pub struct Renamed {
    activities: HashMap<String, ActivityId>,
    media: HashMap<String, String>,
}

impl Renamed {
    fn activity(&self, was: &ActivityId) -> Option<ActivityId> {
        self.activities.get(&was.value().to_string()).cloned()
    }

    fn photo(&self, was: &PhotoId) -> Option<PhotoId> {
        self.media
            .get(&was.value().to_string())
            .and_then(|now| now.parse().ok())
            .map(PhotoId::from_uuid)
    }

    fn voice_memo(&self, was: &VoiceMemoId) -> Option<VoiceMemoId> {
        self.media
            .get(&was.value().to_string())
            .and_then(|now| now.parse().ok())
            .map(VoiceMemoId::from_uuid)
    }
}

fn rebuilt_dimensions(dimensions: &[DimensionValue], renamed: &Renamed) -> Vec<DimensionValue> {
    dimensions
        .iter()
        .map(|dimension| match dimension {
            DimensionValue::Activities(ids) => DimensionValue::activities(
                ids.iter().filter_map(|id| renamed.activity(id)).collect(),
            ),
            DimensionValue::Photos(ids) => {
                DimensionValue::Photos(ids.iter().filter_map(|id| renamed.photo(id)).collect())
            }
            DimensionValue::VoiceMemos(ids) => DimensionValue::VoiceMemos(
                ids.iter().filter_map(|id| renamed.voice_memo(id)).collect(),
            ),
            kept => kept.clone(),
        })
        .collect()
}

async fn restore_metrics(
    user_id: &UserId,
    backed_up: &[RestorableMetric],
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) -> Result<(), ApplicationError> {
    let mut writable = Vec::new();

    for held in backed_up {
        let readable = MetricKind::from_name(&held.kind)
            .and_then(|kind| MetricValue::of_kind(kind, held.value).ok())
            .zip(held.date.parse().ok().map(Date::from_persistence));

        let Some((value, date)) = readable else {
            outcome
                .unreadable
                .push(format!("metric {} on {}", held.kind, held.date));
            continue;
        };

        let source = match &held.provider {
            None => Source::Manual,
            Some(name) => match ProviderName::new(name) {
                Ok(provider) => Source::Provider(provider),
                Err(_) => {
                    outcome.unreadable.push(format!(
                        "metric {} on {}: {name} is not a usable provider name, so it reads as manual",
                        held.kind, held.date
                    ));

                    Source::Manual
                }
            },
        };

        writable.push(DailyMetric::new(user_id.clone(), date, value, source));
    }

    outcome.metrics = deps.metrics.save(&writable).await?;

    Ok(())
}

async fn restore_cycle_starts(
    user_id: &UserId,
    backed_up: &[String],
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) -> Result<(), ApplicationError> {
    for held in backed_up {
        let Some(date) = held.parse().ok().map(Date::from_persistence) else {
            outcome.unreadable.push(format!("cycle start {held}"));
            continue;
        };

        deps.cycles.record(user_id, &date).await?;
        outcome.cycle_starts += 1;
    }

    Ok(())
}

async fn restore_reminders(
    user_id: &UserId,
    backed_up: &[RestorableReminder],
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) -> Result<(), ApplicationError> {
    let existing: Vec<DaySchedule> = deps
        .reminder_query
        .find_by_user(user_id)
        .await?
        .iter()
        .map(|reminder| reminder.schedule().clone())
        .collect();

    for held in backed_up {
        let schedule = rebuilt_schedule(held);

        if existing.contains(&schedule) {
            outcome.skipped += 1;
            continue;
        }

        let mut reminder = Reminder::new(user_id.clone(), schedule);
        if held.enabled {
            reminder.enable();
        } else {
            reminder.disable();
        }

        deps.reminder_command.save(&reminder).await?;
        outcome.reminders += 1;
    }

    Ok(())
}

fn rebuilt_schedule(held: &RestorableReminder) -> DaySchedule {
    let mut schedule = DaySchedule::new();

    for (day, time) in WEEK.iter().zip(&held.times) {
        let at = time
            .as_ref()
            .and_then(|time| NaiveTime::parse_from_str(time, "%H:%M").ok());
        schedule.set_time(*day, at);
    }

    schedule
}

async fn restore_preferences(
    user_id: &UserId,
    tracks_cycle: bool,
    deps: &Deps,
) -> Result<(), ApplicationError> {
    let mut preferences = preferences_of(user_id, &deps.preferences_query).await?;
    preferences.track_cycle(tracks_cycle);

    deps.preferences_command.save(&preferences).await?;

    Ok(())
}

async fn restore_media(
    owner: &UserId,
    contents: &RestorableContents,
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) -> HashMap<String, String> {
    let mut renamed = HashMap::new();

    for held in &contents.photos {
        let upload = match upload_of(held, PHOTO_FALLBACK_TYPE) {
            Ok(upload) => upload,
            Err(reason) => {
                outcome
                    .unreadable
                    .push(format!("photo {}: {reason}", held.id));
                continue;
            }
        };

        match deps.media_storage.store_photo(upload).await {
            Ok(now) => {
                remember(owner, (&now).into(), deps, outcome).await;
                renamed.insert(held.id.clone(), now.value().to_string());
                outcome.media += 1;
            }
            Err(error) => outcome
                .unreadable
                .push(format!("photo {}: could not be stored: {error}", held.id)),
        }
    }

    for held in &contents.voice_memos {
        let upload = match upload_of(held, VOICE_MEMO_FALLBACK_TYPE) {
            Ok(upload) => upload,
            Err(reason) => {
                outcome
                    .unreadable
                    .push(format!("voice memo {}: {reason}", held.id));
                continue;
            }
        };

        match deps.media_storage.store_voice_memo(upload).await {
            Ok(now) => {
                remember(owner, (&now).into(), deps, outcome).await;
                renamed.insert(held.id.clone(), now.value().to_string());
                outcome.media += 1;
            }
            Err(error) => outcome.unreadable.push(format!(
                "voice memo {}: could not be stored: {error}",
                held.id
            )),
        }
    }

    renamed
}

async fn remember(
    owner: &UserId,
    media: domain::attachment::MediaRef,
    deps: &Deps,
    outcome: &mut RestoreOutcome,
) {
    if let Err(error) = deps.media_ownership.remember(owner, media).await {
        outcome
            .unreadable
            .push(format!("{media}: restored but not claimed: {error}"));
    }
}

fn upload_of(held: &RestorableMedia, fallback: &str) -> Result<MediaUpload, String> {
    let content_type = match held.content_type.clone() {
        Some(content_type) => content_type,
        None => ContentType::new(fallback).map_err(|error| error.to_string())?,
    };

    MediaUpload::new(held.data.clone(), content_type).map_err(|error| error.to_string())
}
