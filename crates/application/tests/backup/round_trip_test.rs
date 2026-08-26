use std::sync::Arc;

use domain::activity::{Activity, ActivityName, CategoryName};
use domain::dimension::DimensionValue;
use domain::entry::{Content, Date, DateSpan, Mood, MoodEntry};
use domain::location::Coordinates;
use domain::metric::{DailyMetric, MetricValue, Source, Steps};
use domain::ports::{
    ActivityCommandPort, ActivityQueryPort, CycleStartCommandPort, CycleStartQueryPort,
    DailyMetricCommandPort, DailyMetricQueryPort, EntryDimensionPort, MoodEntryCommandPort,
    MoodEntryQueryPort, ReminderCommandPort, ReminderQueryPort, UserCommandPort,
    UserPreferencesQueryPort,
};
use domain::reminder::{DaySchedule, Reminder};
use domain::song::Song;
use domain::testing::{InMemoryDimensionStore, InMemoryStore, test_user};
use domain::user::{Timezone, User};

use application::export::use_cases::write_backup;
use application::restore::commands::RestoreBackupCommand;
use application::restore::use_cases::restore_backup::{self, RestoreOutcome};

fn dimension_stores() -> Vec<Arc<dyn EntryDimensionPort>> {
    use domain::dimension::DimensionKind;

    [
        DimensionKind::Content,
        DimensionKind::Activities,
        DimensionKind::Location,
        DimensionKind::Song,
        DimensionKind::Photos,
        DimensionKind::VoiceMemos,
    ]
    .into_iter()
    .map(|kind| Arc::new(InMemoryDimensionStore::new(kind)) as Arc<dyn EntryDimensionPort>)
    .collect()
}

struct World {
    store: Arc<InMemoryStore>,
    dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    user: User,
}

async fn a_populated_account() -> World {
    let store = Arc::new(InMemoryStore::new());
    let dimensions = dimension_stores();

    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    let walking = Activity::new(
        user.id().clone(),
        ActivityName::new("long walk").unwrap(),
        Some(CategoryName::new("health").unwrap()),
    );
    ActivityCommandPort::save(store.as_ref(), &walking)
        .await
        .unwrap();

    let entry = MoodEntry::new(
        user.id().clone(),
        Mood::Rad,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T21:30:00+02:00").unwrap(),
    );
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();

    let attached = vec![
        DimensionValue::Content(Content::new("Long walk by the river").unwrap()),
        DimensionValue::activities(vec![walking.id().clone()]),
        DimensionValue::Location(Coordinates::new(52.2297, 21.0122).unwrap()),
        DimensionValue::Song(
            Song::new("Teardrop", "Massive Attack", Some("Mezzanine".into()), None).unwrap(),
        ),
    ];
    for port in &dimensions {
        port.save(entry.id(), &attached).await.unwrap();
    }

    DailyMetricCommandPort::save(
        store.as_ref(),
        &[DailyMetric::new(
            user.id().clone(),
            on("2026-08-20"),
            MetricValue::Steps(Steps::new(8_412).unwrap()),
            Source::Manual,
        )],
    )
    .await
    .unwrap();

    CycleStartCommandPort::record(store.as_ref(), user.id(), &on("2026-08-01"))
        .await
        .unwrap();

    let reminder = Reminder::new(
        user.id().clone(),
        DaySchedule::every_day_at(chrono::NaiveTime::from_hms_opt(20, 0, 0).unwrap()),
    );
    ReminderCommandPort::save(store.as_ref(), &reminder)
        .await
        .unwrap();

    let preference_deps = application::user::use_cases::set_preferences::Deps {
        command: store.clone(),
        query: store.clone(),
    };
    application::user::use_cases::set_preferences::execute(
        user.id().clone(),
        true,
        &preference_deps,
    )
    .await
    .unwrap();

    World {
        store,
        dimensions,
        user,
    }
}

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

async fn backup_of(world: &World) -> Vec<u8> {
    let deps = write_backup::Deps {
        entries: world.store.clone(),
        dimensions: world.dimensions.clone(),
        activities: world.store.clone(),
        reminders: world.store.clone(),
        metrics: world.store.clone(),
        cycles: world.store.clone(),
        preferences: world.store.clone(),
        media_storage: world.store.clone(),
        writer: Arc::new(exporter::ZipBackupWriter),
    };

    write_backup::execute(world.user.id().clone(), &deps)
        .await
        .unwrap()
}

async fn restore_into(world: &World, archive: Vec<u8>) -> RestoreOutcome {
    let deps = restore_backup::Deps {
        reader: Arc::new(importer::KmoodBackupAdapter),
        entry_command: world.store.clone(),
        dimensions: world.dimensions.clone(),
        activity_command: world.store.clone(),
        activity_query: world.store.clone(),
        reminder_command: world.store.clone(),
        metrics: world.store.clone(),
        cycles: world.store.clone(),
        preferences_command: world.store.clone(),
        preferences_query: world.store.clone(),
        media_storage: world.store.clone(),
    };

    restore_backup::execute(
        RestoreBackupCommand {
            user_id: world.user.id().clone(),
            data: archive,
        },
        &deps,
    )
    .await
    .unwrap()
}

async fn an_empty_account() -> World {
    let store = Arc::new(InMemoryStore::new());
    let mut user = test_user("bob");
    user.update_timezone(Some(Timezone::new("Europe/Warsaw").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    World {
        store,
        dimensions: dimension_stores(),
        user,
    }
}

#[tokio::test]
async fn a_backup_restores_into_an_empty_account_without_losing_anything() {
    let source = a_populated_account().await;
    let archive = backup_of(&source).await;

    let restored = an_empty_account().await;
    let outcome = restore_into(&restored, archive).await;

    assert!(
        outcome.unreadable.is_empty(),
        "the restore could not read: {:?}",
        outcome.unreadable
    );
    assert_eq!(outcome.entries, 1);
    assert_eq!(outcome.metrics, 1);
    assert_eq!(outcome.cycle_starts, 1);
    assert_eq!(outcome.activities, 1);
    assert_eq!(outcome.reminders, 1);
}

#[tokio::test]
async fn every_dimension_survives_the_round_trip() {
    let source = a_populated_account().await;
    let archive = backup_of(&source).await;

    let restored = an_empty_account().await;
    restore_into(&restored, archive).await;

    let entries =
        MoodEntryQueryPort::find_by_user(restored.store.as_ref(), restored.user.id(), None, None)
            .await
            .unwrap();
    let composed = application::entry::composition::EntryComposer::new(restored.dimensions.clone())
        .compose(entries)
        .await
        .unwrap();

    assert_eq!(composed.len(), 1);
    let entry = &composed[0];

    assert_eq!(entry.entry.mood(), Mood::Rad);
    assert_eq!(
        entry.content().map(|content| content.value().to_string()),
        Some("Long walk by the river".to_string())
    );
    assert!(entry.location().is_some(), "the location was lost");
    assert_eq!(
        entry.song().map(|song| song.artist().value().to_string()),
        Some("Massive Attack".to_string())
    );
    assert_eq!(entry.activities().len(), 1, "the activity tag was lost");
}

#[tokio::test]
async fn a_restored_activity_tag_points_at_the_restored_activity() {
    let source = a_populated_account().await;
    let archive = backup_of(&source).await;

    let restored = an_empty_account().await;
    restore_into(&restored, archive).await;

    let catalog = ActivityQueryPort::find_by_user(restored.store.as_ref(), restored.user.id())
        .await
        .unwrap();
    let entries =
        MoodEntryQueryPort::find_by_user(restored.store.as_ref(), restored.user.id(), None, None)
            .await
            .unwrap();
    let composed = application::entry::composition::EntryComposer::new(restored.dimensions.clone())
        .compose(entries)
        .await
        .unwrap();

    assert_eq!(catalog.len(), 1);
    assert_eq!(
        composed[0].activities(),
        [catalog[0].id().clone()],
        "the tag must reference the activity as it now exists, not as it was"
    );
    assert_eq!(catalog[0].name().value(), "long walk");
}

#[tokio::test]
async fn metrics_cycle_starts_and_preferences_all_come_back() {
    let source = a_populated_account().await;
    let archive = backup_of(&source).await;

    let restored = an_empty_account().await;
    restore_into(&restored, archive).await;

    let span = DateSpan::new(on("2026-01-01"), on("2026-12-31")).unwrap();
    let metrics =
        DailyMetricQueryPort::find_by_span(restored.store.as_ref(), restored.user.id(), &span)
            .await
            .unwrap();
    let starts = CycleStartQueryPort::find_by_user(restored.store.as_ref(), restored.user.id())
        .await
        .unwrap();
    let preferences =
        UserPreferencesQueryPort::find_by_user(restored.store.as_ref(), restored.user.id())
            .await
            .unwrap()
            .unwrap();
    let reminders = ReminderQueryPort::find_by_user(restored.store.as_ref(), restored.user.id())
        .await
        .unwrap();

    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].value().count(), 8_412);
    assert_eq!(starts.len(), 1);
    assert_eq!(starts[0].to_string(), "2026-08-01");
    assert!(preferences.tracks_cycle(), "the preference was lost");
    assert_eq!(reminders.len(), 1);
    assert!(reminders[0].is_enabled());
}

#[tokio::test]
async fn an_archive_that_is_not_a_backup_is_refused() {
    let restored = an_empty_account().await;

    let deps = restore_backup::Deps {
        reader: Arc::new(importer::KmoodBackupAdapter),
        entry_command: restored.store.clone(),
        dimensions: restored.dimensions.clone(),
        activity_command: restored.store.clone(),
        activity_query: restored.store.clone(),
        reminder_command: restored.store.clone(),
        metrics: restored.store.clone(),
        cycles: restored.store.clone(),
        preferences_command: restored.store.clone(),
        preferences_query: restored.store.clone(),
        media_storage: restored.store.clone(),
    };

    let refused = restore_backup::execute(
        RestoreBackupCommand {
            user_id: restored.user.id().clone(),
            data: b"this is not a zip file at all".to_vec(),
        },
        &deps,
    )
    .await
    .unwrap_err();

    assert!(refused.to_string().contains("zip"), "got {refused}");
}
