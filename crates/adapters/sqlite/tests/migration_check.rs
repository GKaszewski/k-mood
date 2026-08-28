use sqlx::sqlite::SqlitePoolOptions;

async fn fresh_pool() -> sqlx::SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

async fn column_names(pool: &sqlx::SqlitePool, table: &str) -> Vec<String> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT name FROM pragma_table_info(?) ORDER BY cid")
        .bind(table)
        .fetch_all(pool)
        .await
        .unwrap();
    rows.into_iter().map(|row| row.0).collect()
}

async fn applied_count(pool: &sqlx::SqlitePool) -> i64 {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM schema_migrations")
        .fetch_one(pool)
        .await
        .unwrap();
    row.0
}

#[tokio::test]
async fn every_migration_applies_exactly_once_however_often_the_server_restarts() {
    let pool = fresh_pool().await;

    sqlite::run_migrations(&pool).await.unwrap();
    let after_first_boot = applied_count(&pool).await;

    sqlite::run_migrations(&pool).await.unwrap();
    sqlite::run_migrations(&pool).await.unwrap();
    let after_further_boots = applied_count(&pool).await;

    assert!(after_first_boot > 0, "no migrations were applied at all");
    assert_eq!(after_first_boot, after_further_boots);
}

#[tokio::test]
async fn mood_entries_no_longer_carries_content() {
    let pool = fresh_pool().await;
    sqlite::run_migrations(&pool).await.unwrap();

    assert_eq!(
        column_names(&pool, "mood_entries").await,
        [
            "id",
            "user_id",
            "mood",
            "logged_at",
            "created_at",
            "updated_at"
        ]
    );
    assert_eq!(
        column_names(&pool, "entry_content").await,
        ["entry_id", "content"]
    );
}

fn a_shared_file() -> String {
    let name = format!("k-mood-migrating-{}.sqlite", uuid::Uuid::new_v4());

    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .to_string()
}

fn forget(path: &str) {
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(format!("{path}-wal"));
    let _ = std::fs::remove_file(format!("{path}-shm"));
}

#[tokio::test]
async fn two_processes_starting_together_both_migrate_successfully() {
    let path = a_shared_file();
    let url = format!("sqlite://{path}");

    let server = sqlite::create_pool(&url).await.unwrap();
    let worker = sqlite::create_pool(&url).await.unwrap();

    let (migrating_server, migrating_worker) = tokio::join!(
        tokio::spawn({
            let pool = server.clone();
            async move { sqlite::run_migrations(&pool).await }
        }),
        tokio::spawn({
            let pool = worker.clone();
            async move { sqlite::run_migrations(&pool).await }
        }),
    );

    let by_the_server = migrating_server.unwrap();
    let by_the_worker = migrating_worker.unwrap();

    assert!(
        by_the_server.is_ok(),
        "the server could not start alongside the worker: {by_the_server:?}"
    );
    assert!(
        by_the_worker.is_ok(),
        "the worker could not start alongside the server: {by_the_worker:?}"
    );

    let applied = applied_count(&server).await;
    let names: Vec<(String,)> = sqlx::query_as("SELECT name FROM schema_migrations")
        .fetch_all(&server)
        .await
        .unwrap();

    assert_eq!(
        applied as usize,
        names.len(),
        "no migration is recorded twice"
    );
    assert!(applied > 0);

    forget(&path);
}

#[tokio::test]
async fn a_process_joining_a_migrated_database_applies_nothing() {
    let path = a_shared_file();
    let url = format!("sqlite://{path}");

    let first = sqlite::create_pool(&url).await.unwrap();
    sqlite::run_migrations(&first).await.unwrap();
    let already_applied = applied_count(&first).await;

    let second = sqlite::create_pool(&url).await.unwrap();
    sqlite::run_migrations(&second).await.unwrap();

    assert_eq!(applied_count(&second).await, already_applied);

    forget(&path);
}

#[tokio::test]
async fn legacy_instants_are_rewritten_into_one_sortable_form() {
    let pool = fresh_pool().await;
    sqlite::run_migrations(&pool).await.unwrap();

    let user = domain::testing::test_user("alice");
    domain::ports::UserCommandPort::save(
        &sqlite::repositories::SqliteUserCommandRepository::new(pool.clone()),
        &user,
    )
    .await
    .unwrap();

    let legacy = [
        ("2026-08-25T20:00:00+02:00", "2026-08-25T18:00:00+00:00"),
        ("2026-08-24T21:30:00-04:00", "2026-08-25T01:30:00+00:00"),
        ("2026-08-23T09:00:00+00:00", "2026-08-23T09:00:00+00:00"),
    ];

    for (index, (written, _)) in legacy.iter().enumerate() {
        sqlx::query(
            "INSERT INTO mood_entries (id, user_id, mood, logged_at, created_at, updated_at)
             VALUES (?, ?, 3, ?, ?, ?)",
        )
        .bind(format!("00000000-0000-0000-0000-00000000000{index}"))
        .bind(user.id().value().to_string())
        .bind(written)
        .bind(written)
        .bind(written)
        .execute(&pool)
        .await
        .unwrap();
    }

    sqlx::raw_sql(include_str!("../src/migrations/015_logged_at_in_utc.sql"))
        .execute(&pool)
        .await
        .unwrap();

    for (index, (written, expected)) in legacy.iter().enumerate() {
        let (held,): (String,) = sqlx::query_as("SELECT logged_at FROM mood_entries WHERE id = ?")
            .bind(format!("00000000-0000-0000-0000-00000000000{index}"))
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(held, *expected, "{written} should normalise to {expected}");
    }
}

#[tokio::test]
async fn a_token_minted_before_scopes_existed_still_writes_metrics() {
    let pool = fresh_pool().await;

    for (name, sql) in sqlite::migrations_before("016_api_token_scopes") {
        sqlx::raw_sql(sql)
            .execute(&pool)
            .await
            .unwrap_or_else(|error| panic!("{name} failed: {error}"));
    }

    let user = domain::testing::test_user("alice");
    domain::ports::UserCommandPort::save(
        &sqlite::repositories::SqliteUserCommandRepository::new(pool.clone()),
        &user,
    )
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO api_tokens (id, user_id, name, digest, scope, created_at)
         VALUES (?, ?, 'tasker', 'abc123', 'writeMetrics', ?)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(user.id().value().to_string())
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&pool)
    .await
    .unwrap();

    sqlx::raw_sql(include_str!("../src/migrations/016_api_token_scopes.sql"))
        .execute(&pool)
        .await
        .unwrap();

    let found = domain::ports::ApiTokenQueryPort::find_by_digest(
        &sqlite::repositories::SqliteApiTokenQueryRepository::new(pool.clone()),
        &domain::api_token::TokenDigest::from_persistence("abc123".into()),
    )
    .await
    .unwrap()
    .expect("the token should have survived the migration");

    assert!(
        found.allows(domain::api_token::TokenScope::WriteMetrics),
        "a token minted before scopes existed keeps doing what it always did"
    );
    assert!(
        !found.allows(domain::api_token::TokenScope::ReadJournal),
        "and gains nothing it was never granted"
    );
}
