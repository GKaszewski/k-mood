use std::time::Duration;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

const BUSY_TIMEOUT: Duration = Duration::from_secs(10);

const MIGRATIONS: &[(&str, &str)] = &[
    ("001_initial", include_str!("migrations/001_initial.sql")),
    (
        "002_push_subscriptions",
        include_str!("migrations/002_push_subscriptions.sql"),
    ),
    (
        "003_entry_content",
        include_str!("migrations/003_entry_content.sql"),
    ),
    (
        "004_drop_entry_content_column",
        include_str!("migrations/004_drop_entry_content_column.sql"),
    ),
    (
        "005_location_and_song",
        include_str!("migrations/005_location_and_song.sql"),
    ),
    (
        "006_provider_connections",
        include_str!("migrations/006_provider_connections.sql"),
    ),
    (
        "007_daily_metrics",
        include_str!("migrations/007_daily_metrics.sql"),
    ),
    (
        "008_api_tokens",
        include_str!("migrations/008_api_tokens.sql"),
    ),
    (
        "009_metric_rejections",
        include_str!("migrations/009_metric_rejections.sql"),
    ),
    (
        "010_cycle_and_preferences",
        include_str!("migrations/010_cycle_and_preferences.sql"),
    ),
    ("011_jobs", include_str!("migrations/011_jobs.sql")),
    (
        "012_entry_weather",
        include_str!("migrations/012_entry_weather.sql"),
    ),
    (
        "013_reminder_last_sent",
        include_str!("migrations/013_reminder_last_sent.sql"),
    ),
    (
        "014_media_ownership",
        include_str!("migrations/014_media_ownership.sql"),
    ),
    (
        "015_logged_at_in_utc",
        include_str!("migrations/015_logged_at_in_utc.sql"),
    ),
    (
        "016_api_token_scopes",
        include_str!("migrations/016_api_token_scopes.sql"),
    ),
];

const TAKE_THE_WRITE_LOCK_UP_FRONT: &str = "BEGIN IMMEDIATE";

const SCHEMA_MIGRATIONS_TABLE: &str = "CREATE TABLE IF NOT EXISTS schema_migrations (
    name TEXT PRIMARY KEY NOT NULL,
    applied_at TEXT NOT NULL
)";

pub fn migrations_before(name: &str) -> impl Iterator<Item = (&'static str, &'static str)> {
    MIGRATIONS
        .iter()
        .take_while(move |(applied, _)| *applied != name)
        .copied()
}

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options: SqliteConnectOptions = database_url
        .parse::<SqliteConnectOptions>()?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(BUSY_TIMEOUT)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut connection = pool.acquire().await?;

    sqlx::raw_sql(SCHEMA_MIGRATIONS_TABLE)
        .execute(&mut *connection)
        .await?;

    sqlx::raw_sql(TAKE_THE_WRITE_LOCK_UP_FRONT)
        .execute(&mut *connection)
        .await?;

    match apply_pending(&mut connection).await {
        Ok(()) => {
            sqlx::raw_sql("COMMIT").execute(&mut *connection).await?;

            Ok(())
        }
        Err(error) => {
            let _ = sqlx::raw_sql("ROLLBACK").execute(&mut *connection).await;

            Err(error)
        }
    }
}

async fn apply_pending(connection: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    for (name, sql) in MIGRATIONS {
        if is_applied(connection, name).await? {
            continue;
        }

        sqlx::raw_sql(*sql).execute(&mut *connection).await?;

        sqlx::query("INSERT INTO schema_migrations (name, applied_at) VALUES (?, ?)")
            .bind(*name)
            .bind(chrono::Utc::now().to_rfc3339())
            .execute(&mut *connection)
            .await?;

        tracing::info!(migration = *name, "applied migration");
    }

    Ok(())
}

async fn is_applied(
    connection: &mut sqlx::SqliteConnection,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let existing: Option<(String,)> =
        sqlx::query_as("SELECT name FROM schema_migrations WHERE name = ?")
            .bind(name)
            .fetch_optional(&mut *connection)
            .await?;

    Ok(existing.is_some())
}
