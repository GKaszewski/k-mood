use std::io::Read;

use domain::activity::{Activity, ActivityName};
use domain::dimension::{ComposedEntry, DimensionValue};
use domain::entry::{Content, Mood, MoodEntry};
use domain::location::Coordinates;
use domain::metric::{DailyMetric, MetricValue, Source, Steps};
use domain::ports::{BackupMedia, BackupWriterPort, ExtractWriterPort, SharedExtract, UserBackup};
use domain::song::Song;
use domain::user::{UserId, UserPreferences};

use exporter::{MarkdownExtractWriter, ZipBackupWriter};

fn at(instant: &str) -> chrono::DateTime<chrono::FixedOffset> {
    chrono::DateTime::parse_from_rfc3339(instant).unwrap()
}

fn a_revealing_entry(owner: &UserId, exercise: &Activity) -> ComposedEntry {
    ComposedEntry {
        entry: MoodEntry::new(owner.clone(), Mood::Rad, at("2026-08-20T21:30:00+02:00")),
        dimensions: vec![
            DimensionValue::Content(Content::new("Long walk by the river").unwrap()),
            DimensionValue::activities(vec![exercise.id().clone()]),
            DimensionValue::Location(Coordinates::new(52.2297, 21.0122).unwrap()),
            DimensionValue::Song(
                Song::new("Teardrop", "Massive Attack", Some("Mezzanine".into()), None).unwrap(),
            ),
        ],
    }
}

async fn extract_of(entries: Vec<ComposedEntry>, activities: Vec<Activity>) -> String {
    let bytes = MarkdownExtractWriter
        .write(&SharedExtract {
            entries,
            activities,
        })
        .await
        .unwrap();

    String::from_utf8(bytes).unwrap()
}

#[tokio::test]
async fn the_extract_carries_the_journal_a_person_would_want_to_read() {
    let owner = UserId::generate();
    let exercise = Activity::new(owner.clone(), ActivityName::new("long walk").unwrap(), None);

    let document = extract_of(vec![a_revealing_entry(&owner, &exercise)], vec![exercise]).await;

    assert!(document.contains("Long walk by the river"), "{document}");
    assert!(document.contains("Rad"), "{document}");
    assert!(
        document.contains("long walk"),
        "the activity name is missing"
    );
    assert!(document.contains("Thursday 20 August 2026"), "{document}");
}

#[tokio::test]
async fn the_extract_discloses_no_place_and_no_song() {
    let owner = UserId::generate();
    let exercise = Activity::new(owner.clone(), ActivityName::new("long walk").unwrap(), None);

    let document = extract_of(vec![a_revealing_entry(&owner, &exercise)], vec![exercise]).await;

    for secret in ["52.2", "21.0", "Massive Attack", "Teardrop", "Mezzanine"] {
        assert!(
            !document.contains(secret),
            "the extract leaked {secret}:\n{document}"
        );
    }
}

#[tokio::test]
async fn the_extract_says_what_it_is_not() {
    let document = extract_of(Vec::new(), Vec::new()).await;

    assert!(document.contains("not a backup"), "{document}");
}

#[tokio::test]
async fn a_backup_carries_everything_the_extract_leaves_out() {
    let owner = UserId::generate();
    let exercise = Activity::new(owner.clone(), ActivityName::new("long walk").unwrap(), None);

    let backup = UserBackup {
        entries: vec![a_revealing_entry(&owner, &exercise)],
        metrics: vec![DailyMetric::new(
            owner.clone(),
            domain::entry::Date::from_persistence("2026-08-20".parse().unwrap()),
            MetricValue::Steps(Steps::new(8_412).unwrap()),
            Source::Manual,
        )],
        cycle_starts: vec![domain::cycle::CycleStartRestore::on(
            domain::entry::Date::from_persistence("2026-08-01".parse().unwrap()),
        )],
        activities: vec![exercise],
        reminders: Vec::new(),
        preferences: UserPreferences::off_by_default(owner),
        media: BackupMedia {
            photos: Vec::new(),
            voice_memos: Vec::new(),
        },
    };

    let archive = ZipBackupWriter.write(&backup).await.unwrap();
    let manifest = manifest_of(&archive);

    for expected in [
        "Long walk by the river",
        "52.2297",
        "Massive Attack",
        "8412",
        "2026-08-01",
        "long walk",
    ] {
        assert!(
            manifest.contains(expected),
            "the backup is missing {expected}"
        );
    }
}

fn manifest_of(archive: &[u8]) -> String {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive)).unwrap();
    let mut file = zip.by_name("backup.json").expect("a backup has a manifest");
    let mut manifest = String::new();
    file.read_to_string(&mut manifest).unwrap();

    manifest
}
