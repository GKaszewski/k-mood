use sqlx::SqlitePool;

use domain::dimension::DimensionValue;
use domain::entry::{Content, Mood, MoodEntry, MoodEntryId};
use domain::location::Coordinates;
use domain::ports::{EntryDimensionPort, MoodEntryCommandPort, UserCommandPort};
use domain::song::Song;
use domain::testing::test_user;
use domain::user::{User, UserId};

use sqlite::repositories::{
    SqliteContentDimensionRepository, SqliteEntryCommandRepository,
    SqliteLocationDimensionRepository, SqliteSongDimensionRepository, SqliteUserCommandRepository,
};

const EVERY_TABLE_THAT_HANGS_OFF_AN_ENTRY: [&str; 4] = [
    "entry_content",
    "entry_location",
    "entry_song",
    "entry_activities",
];

async fn a_file() -> String {
    let name = format!("k-mood-cascade-{}.sqlite", uuid::Uuid::new_v4());

    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .to_string()
}

async fn an_entry_with_every_dimension(pool: &SqlitePool) -> (User, MoodEntryId) {
    let user = test_user("alice");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&user)
        .await
        .unwrap();

    let entry = MoodEntry::new(
        user.id().clone(),
        Mood::Good,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T12:00:00+02:00").unwrap(),
    );
    SqliteEntryCommandRepository::new(pool.clone())
        .save(&entry)
        .await
        .unwrap();

    SqliteContentDimensionRepository::new(pool.clone())
        .save(
            entry.id(),
            &[DimensionValue::Content(Content::new("a note").unwrap())],
        )
        .await
        .unwrap();
    SqliteLocationDimensionRepository::new(pool.clone())
        .save(
            entry.id(),
            &[DimensionValue::Location(
                Coordinates::new(52.2297, 21.0122).unwrap(),
            )],
        )
        .await
        .unwrap();
    SqliteSongDimensionRepository::new(pool.clone())
        .save(
            entry.id(),
            &[DimensionValue::Song(
                Song::new("Teardrop", "Massive Attack", None, None).unwrap(),
            )],
        )
        .await
        .unwrap();

    (user, entry.id().clone())
}

async fn rows_in(pool: &SqlitePool, table: &str) -> i64 {
    let counted: (i64,) =
        sqlx::query_as(sqlx::AssertSqlSafe(format!("SELECT COUNT(*) FROM {table}")))
            .fetch_one(pool)
            .await
            .unwrap();

    counted.0
}

async fn dimension_rows(pool: &SqlitePool) -> i64 {
    let mut total = 0;
    for table in EVERY_TABLE_THAT_HANGS_OFF_AN_ENTRY {
        total += rows_in(pool, table).await;
    }

    total
}

#[tokio::test]
async fn foreign_keys_are_switched_on_for_every_connection_the_pool_hands_out() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();

    for _ in 0..5 {
        let on: (i64,) = sqlx::query_as("PRAGMA foreign_keys")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(
            on.0, 1,
            "sqlite ignores ON DELETE CASCADE silently when foreign keys are off"
        );
    }

    remove(&path);
}

#[tokio::test]
async fn deleting_an_entry_really_does_remove_its_dimension_rows() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let (_, entry_id) = an_entry_with_every_dimension(&pool).await;
    assert!(
        dimension_rows(&pool).await >= 3,
        "the dimensions were not stored"
    );

    SqliteEntryCommandRepository::new(pool.clone())
        .delete(&entry_id)
        .await
        .unwrap();

    assert_eq!(
        dimension_rows(&pool).await,
        0,
        "ON DELETE CASCADE did not fire"
    );

    remove(&path);
}

#[tokio::test]
async fn a_dimension_row_cannot_be_written_for_an_entry_that_does_not_exist() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let refused = SqliteContentDimensionRepository::new(pool.clone())
        .save(
            &MoodEntryId::generate(),
            &[DimensionValue::Content(Content::new("orphan").unwrap())],
        )
        .await;

    assert!(
        refused.is_err(),
        "a foreign key that is not enforced is not a foreign key"
    );

    remove(&path);
}

#[tokio::test]
async fn deleting_an_account_removes_everything_that_hangs_off_it() {
    use domain::ports::CascadeDeletePort;
    use sqlite::repositories::SqliteCascadeDeleteRepository;

    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let (user, _) = an_entry_with_every_dimension(&pool).await;

    SqliteCascadeDeleteRepository::new(pool.clone())
        .delete_user_account(user.id())
        .await
        .unwrap();

    assert_eq!(rows_in(&pool, "users").await, 0);
    assert_eq!(rows_in(&pool, "mood_entries").await, 0);
    assert_eq!(
        dimension_rows(&pool).await,
        0,
        "the cascade must reach through the entry to its dimensions"
    );

    remove(&path);
}

#[tokio::test]
async fn the_oldest_tables_do_not_cascade_from_users_which_is_why_they_are_deleted_by_hand() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let (user, _) = an_entry_with_every_dimension(&pool).await;

    let refused = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user.id().value().to_string())
        .execute(&pool)
        .await;

    assert!(
        refused.is_err(),
        "mood_entries references users without ON DELETE CASCADE, so the repository must \
         delete the older tables itself. If this now succeeds, the schema gained a cascade \
         and those manual deletes are redundant."
    );

    remove(&path);
}

#[tokio::test]
async fn every_table_added_since_does_cascade_from_users() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let cascading = [
        "provider_connections",
        "daily_metrics",
        "api_tokens",
        "metric_rejections",
        "cycle_starts",
        "user_preferences",
    ];

    for table in cascading {
        let sql: (String,) =
            sqlx::query_as("SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?")
                .bind(table)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert!(
            sql.0.contains("REFERENCES users(id) ON DELETE CASCADE"),
            "{table} should be removed by the database when its account goes"
        );
    }

    remove(&path);
}

#[tokio::test]
async fn an_entry_cannot_belong_to_an_account_that_does_not_exist() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let orphan = MoodEntry::new(
        UserId::generate(),
        Mood::Good,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T12:00:00+02:00").unwrap(),
    );

    let refused = SqliteEntryCommandRepository::new(pool.clone())
        .save(&orphan)
        .await;

    assert!(refused.is_err(), "an entry with no owner should be refused");

    remove(&path);
}

fn remove(path: &str) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{path}-wal"));
    let _ = std::fs::remove_file(format!("{path}-shm"));
}

const EVERY_TABLE_A_USER_FILLS: [&str; 7] = [
    "mood_entries",
    "activities",
    "reminders",
    "daily_metrics",
    "cycle_starts",
    "metric_rejections",
    "media_owners",
];

async fn fill_every_table_for(pool: &SqlitePool, user: &User) {
    let entry = MoodEntry::new(
        user.id().clone(),
        Mood::Good,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T12:00:00+02:00").unwrap(),
    );
    SqliteEntryCommandRepository::new(pool.clone())
        .save(&entry)
        .await
        .unwrap();

    domain::ports::ActivityCommandPort::save(
        &sqlite::repositories::SqliteActivityCommandRepository::new(pool.clone()),
        &domain::testing::test_activity(user.id().clone(), "gaming"),
    )
    .await
    .unwrap();

    domain::ports::ReminderCommandPort::save(
        &sqlite::repositories::SqliteReminderCommandRepository::new(pool.clone()),
        &domain::testing::test_reminder(user.id().clone()),
    )
    .await
    .unwrap();

    domain::ports::DailyMetricCommandPort::save(
        &sqlite::repositories::SqliteDailyMetricCommandRepository::new(pool.clone()),
        &[domain::metric::DailyMetric::new(
            user.id().clone(),
            a_date("2026-08-20"),
            domain::metric::MetricValue::Steps(domain::metric::Steps::new(8_412).unwrap()),
            domain::metric::Source::Manual,
        )],
    )
    .await
    .unwrap();

    domain::ports::CycleStartCommandPort::record(
        &sqlite::repositories::SqliteCycleStartRepository::new(pool.clone()),
        user.id(),
        &a_date("2026-08-01"),
    )
    .await
    .unwrap();

    domain::ports::RejectionCommandPort::record(
        &sqlite::repositories::SqliteRejectionRepository::new(pool.clone(), 100),
        &[domain::rejection::RejectedMetric::new(
            user.id().clone(),
            domain::rejection::RejectionOrigin::Import,
            domain::rejection::RejectionDetail::new(
                None,
                Some(a_date("2026-08-19")),
                "steps",
                Some(-1),
            ),
            String::from("steps cannot be negative"),
        )],
    )
    .await
    .unwrap();

    domain::ports::MediaOwnershipPort::remember(
        &sqlite::repositories::SqliteMediaOwnershipRepository::new(pool.clone()),
        user.id(),
        (&domain::attachment::PhotoId::generate()).into(),
    )
    .await
    .unwrap();
}

fn a_date(day: &str) -> domain::entry::Date {
    domain::entry::Date::from_persistence(day.parse().unwrap())
}

#[tokio::test]
async fn clearing_a_users_data_empties_every_table_they_fill() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let user = test_user("alice");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&user)
        .await
        .unwrap();
    fill_every_table_for(&pool, &user).await;

    for table in EVERY_TABLE_A_USER_FILLS {
        assert_eq!(rows_in(&pool, table).await, 1, "{table} should be seeded");
    }

    domain::ports::CascadeDeletePort::delete_all_user_data(
        &sqlite::repositories::SqliteCascadeDeleteRepository::new(pool.clone()),
        user.id(),
    )
    .await
    .unwrap();

    for table in EVERY_TABLE_A_USER_FILLS {
        assert_eq!(
            rows_in(&pool, table).await,
            0,
            "{table} still holds data the user asked to be cleared"
        );
    }

    let users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(users.0, 1, "clearing data is not deleting the account");

    remove(&path);
}

#[tokio::test]
async fn deleting_an_account_empties_every_table_they_fill() {
    let path = a_file().await;
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let user = test_user("alice");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&user)
        .await
        .unwrap();
    fill_every_table_for(&pool, &user).await;

    domain::ports::CascadeDeletePort::delete_user_account(
        &sqlite::repositories::SqliteCascadeDeleteRepository::new(pool.clone()),
        user.id(),
    )
    .await
    .unwrap();

    for table in EVERY_TABLE_A_USER_FILLS {
        assert_eq!(
            rows_in(&pool, table).await,
            0,
            "{table} outlived the account it belonged to"
        );
    }

    let users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(users.0, 0);

    remove(&path);
}
