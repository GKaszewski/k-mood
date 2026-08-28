use std::sync::Arc;

use domain::ports::{ImportSourcePort, ImportedRow, MoodEntryQueryPort, UserCommandPort};
use domain::testing::{InMemoryStore, test_user};
use domain::user::{Timezone, User};

use application::import::commands::ImportCommand;
use application::import::use_cases::import_entries;

struct StubDaylio(Vec<(String, String, u8)>);

#[async_trait::async_trait]
impl ImportSourcePort for StubDaylio {
    async fn read_entries(
        &self,
        _data: &[u8],
    ) -> Result<Vec<ImportedRow>, domain::errors::DomainError> {
        Ok(self
            .0
            .iter()
            .map(|(date, time, mood)| ImportedRow {
                mood: *mood,
                date: date.clone(),
                time: time.clone(),
                activities: Vec::new(),
                note: None,
            })
            .collect())
    }
}

async fn a_user_in(store: &Arc<InMemoryStore>, zone: Option<&str>) -> User {
    let mut user = test_user("alice");
    user.update_timezone(zone.map(|zone| Timezone::new(zone).unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();

    user
}

async fn import(
    store: &Arc<InMemoryStore>,
    user: &User,
    rows: Vec<(String, String, u8)>,
) -> Result<
    application::import::use_cases::import_entries::ImportResult,
    application::errors::ApplicationError,
> {
    let deps = import_entries::Deps {
        source: Arc::new(StubDaylio(rows)),
        entry_command: store.clone(),
        entry_query: store.clone(),
        activity_command: store.clone(),
        activity_query: store.clone(),
        dimensions: Vec::new(),
        users: store.clone(),
        preset: config::PresetConfig::default(),
    };

    import_entries::execute(
        ImportCommand {
            user_id: user.id().clone(),
            data: Vec::new(),
        },
        &deps,
    )
    .await
}

async fn instants_of(store: &Arc<InMemoryStore>, user: &User) -> Vec<String> {
    let mut held: Vec<String> = MoodEntryQueryPort::find_all_by_user(store.as_ref(), user.id())
        .await
        .unwrap()
        .iter()
        .map(|entry| entry.logged_at().to_rfc3339())
        .collect();
    held.sort();

    held
}

#[tokio::test]
async fn an_eight_pm_entry_is_eight_pm_where_the_user_lives() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    import(
        &store,
        &user,
        vec![("2026-08-25".into(), "8:00 PM".into(), 3)],
    )
    .await
    .unwrap();

    assert_eq!(
        instants_of(&store, &user).await,
        ["2026-08-25T20:00:00+02:00"],
        "a wall clock time with no zone is the user's own wall clock"
    );
}

#[tokio::test]
async fn the_offset_follows_daylight_saving_rather_than_being_fixed() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    import(
        &store,
        &user,
        vec![
            ("2026-01-15".into(), "8:00 PM".into(), 3),
            ("2026-07-15".into(), "8:00 PM".into(), 4),
        ],
    )
    .await
    .unwrap();

    assert_eq!(
        instants_of(&store, &user).await,
        ["2026-01-15T20:00:00+01:00", "2026-07-15T20:00:00+02:00"],
        "winter is one hour ahead of UTC in Warsaw, summer is two"
    );
}

#[tokio::test]
async fn a_twenty_four_hour_clock_is_read_the_same_way() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    import(
        &store,
        &user,
        vec![("2026-08-25".into(), "20:00".into(), 3)],
    )
    .await
    .unwrap();

    assert_eq!(
        instants_of(&store, &user).await,
        ["2026-08-25T20:00:00+02:00"]
    );
}

#[tokio::test]
async fn an_account_with_no_timezone_cannot_place_a_wall_clock_time() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, None).await;

    let refused = import(
        &store,
        &user,
        vec![("2026-08-25".into(), "8:00 PM".into(), 3)],
    )
    .await
    .unwrap_err();

    assert!(
        refused.to_string().contains("timezone"),
        "importing silently into the wrong hour is worse than asking: {refused}"
    );
    assert!(instants_of(&store, &user).await.is_empty());
}

#[tokio::test]
async fn an_hour_that_daylight_saving_skips_is_still_imported() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    let outcome = import(
        &store,
        &user,
        vec![("2026-03-29".into(), "2:30 AM".into(), 3)],
    )
    .await
    .unwrap();

    assert_eq!(
        outcome.imported, 1,
        "the clocks skip 2:30 that night, and the entry must still land somewhere sensible"
    );
    assert_eq!(instants_of(&store, &user).await.len(), 1);
}

#[tokio::test]
async fn importing_the_same_export_twice_does_not_duplicate_anything() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;
    let rows = vec![("2026-08-25".into(), "8:00 PM".into(), 3)];

    import(&store, &user, rows.clone()).await.unwrap();
    let again = import(&store, &user, rows).await.unwrap();

    assert_eq!(again.imported, 0);
    assert_eq!(again.skipped, 1);
    assert_eq!(instants_of(&store, &user).await.len(), 1);
}

#[tokio::test]
async fn re_importing_the_same_file_adds_nothing() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    let rows = vec![
        ("2026-08-25".to_string(), "20:00".to_string(), 4),
        ("2026-08-26".to_string(), "09:30".to_string(), 2),
    ];

    let first = import(&store, &user, rows.clone()).await.unwrap();
    assert_eq!(first.imported, 2);

    let again = import(&store, &user, rows).await.unwrap();

    assert_eq!(again.imported, 0, "every row was already here");
    assert_eq!(again.skipped, 2);
    assert_eq!(
        store.entry_count(),
        2,
        "re-importing must not double the history"
    );
}

#[tokio::test]
async fn an_entry_held_in_utc_still_matches_a_row_written_in_local_time() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    let eight_in_the_evening_in_warsaw =
        chrono::DateTime::parse_from_rfc3339("2026-08-25T18:00:00+00:00").unwrap();
    let held = domain::entry::MoodEntry::new(
        user.id().clone(),
        domain::entry::Mood::Good,
        eight_in_the_evening_in_warsaw,
    );
    domain::ports::MoodEntryCommandPort::save(store.as_ref(), &held)
        .await
        .unwrap();

    let outcome = import(
        &store,
        &user,
        vec![("2026-08-25".to_string(), "20:00".to_string(), 4)],
    )
    .await
    .unwrap();

    assert_eq!(
        outcome.imported, 0,
        "the same instant is the same entry whether it is held as +00:00 or +02:00"
    );
    assert_eq!(store.entry_count(), 1);
}

#[tokio::test]
async fn a_row_repeated_inside_one_file_lands_once() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user_in(&store, Some("Europe/Warsaw")).await;

    let repeated = ("2026-08-25".to_string(), "20:00".to_string(), 4);
    let outcome = import(&store, &user, vec![repeated.clone(), repeated])
        .await
        .unwrap();

    assert_eq!(outcome.imported, 1);
    assert_eq!(outcome.skipped, 1);
    assert_eq!(store.entry_count(), 1);
}
