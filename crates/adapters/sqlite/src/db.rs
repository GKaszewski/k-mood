use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

const MIGRATIONS: &[&str] = &[
    include_str!("migrations/001_initial.sql"),
    include_str!("migrations/002_push_subscriptions.sql"),
];

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options: SqliteConnectOptions = database_url
        .parse::<SqliteConnectOptions>()?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for migration in MIGRATIONS {
        sqlx::raw_sql(*migration).execute(pool).await?;
    }
    tracing::info!("database migrations completed");
    Ok(())
}
