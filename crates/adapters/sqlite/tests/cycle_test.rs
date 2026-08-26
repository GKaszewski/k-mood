use sqlx::sqlite::SqlitePoolOptions;

use domain::entry::Date;
use domain::ports::{
    CascadeDeletePort, CycleStartCommandPort, CycleStartQueryPort, UserCommandPort,
    UserPreferencesCommandPort, UserPreferencesQueryPort,
};
use domain::testing::test_user;
use domain::user::{User, UserId, UserPreferences};

use sqlite::repositories::{
    SqliteCascadeDeleteRepository, SqliteCycleStartRepository, SqliteUserCommandRepository,
    SqliteUserPreferencesRepository,
};

async fn a_pool_with_a_user() -> (sqlx::SqlitePool, User) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let user = test_user("alice");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&user)
        .await
        .unwrap();

    (pool, user)
}

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

#[tokio::test]
async fn recording_the_same_start_twice_leaves_one_row() {
    let (pool, user) = a_pool_with_a_user().await;
    let starts = SqliteCycleStartRepository::new(pool.clone());

    starts.record(user.id(), &on("2026-01-01")).await.unwrap();
    starts.record(user.id(), &on("2026-01-01")).await.unwrap();

    let rows: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cycle_starts")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(rows.0, 1);
    assert_eq!(starts.find_by_user(user.id()).await.unwrap().len(), 1);
}

#[tokio::test]
async fn starts_come_back_oldest_first() {
    let (pool, user) = a_pool_with_a_user().await;
    let starts = SqliteCycleStartRepository::new(pool.clone());

    for day in ["2026-02-26", "2026-01-01", "2026-01-29"] {
        starts.record(user.id(), &on(day)).await.unwrap();
    }

    let found: Vec<String> = starts
        .find_by_user(user.id())
        .await
        .unwrap()
        .iter()
        .map(|date| date.to_string())
        .collect();

    assert_eq!(found, ["2026-01-01", "2026-01-29", "2026-02-26"]);
}

#[tokio::test]
async fn forgetting_a_start_removes_only_that_one() {
    let (pool, user) = a_pool_with_a_user().await;
    let starts = SqliteCycleStartRepository::new(pool.clone());
    starts.record(user.id(), &on("2026-01-01")).await.unwrap();
    starts.record(user.id(), &on("2026-01-29")).await.unwrap();

    starts.forget(user.id(), &on("2026-01-01")).await.unwrap();

    let found: Vec<String> = starts
        .find_by_user(user.id())
        .await
        .unwrap()
        .iter()
        .map(|date| date.to_string())
        .collect();

    assert_eq!(found, ["2026-01-29"]);
}

#[tokio::test]
async fn one_accounts_starts_are_not_anothers() {
    let (pool, mine) = a_pool_with_a_user().await;
    let starts = SqliteCycleStartRepository::new(pool.clone());

    starts.record(mine.id(), &on("2026-01-01")).await.unwrap();

    assert!(
        starts
            .find_by_user(&UserId::generate())
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn an_account_with_no_stored_preference_has_none_to_read() {
    let (pool, user) = a_pool_with_a_user().await;

    let found = SqliteUserPreferencesRepository::new(pool.clone())
        .find_by_user(user.id())
        .await
        .unwrap();

    assert!(
        found.is_none(),
        "the default belongs to the domain, not the row"
    );
}

#[tokio::test]
async fn a_preference_survives_being_written_twice() {
    let (pool, user) = a_pool_with_a_user().await;
    let preferences = SqliteUserPreferencesRepository::new(pool.clone());

    let mut held = UserPreferences::off_by_default(user.id().clone());
    held.track_cycle(true);
    preferences.save(&held).await.unwrap();

    held.track_cycle(false);
    preferences.save(&held).await.unwrap();

    let found = preferences.find_by_user(user.id()).await.unwrap().unwrap();

    assert!(!found.tracks_cycle());

    let rows: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_preferences")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(rows.0, 1);
}

#[tokio::test]
async fn deleting_an_account_takes_its_cycle_and_preferences_with_it() {
    let (pool, user) = a_pool_with_a_user().await;
    SqliteCycleStartRepository::new(pool.clone())
        .record(user.id(), &on("2026-01-01"))
        .await
        .unwrap();

    let mut held = UserPreferences::off_by_default(user.id().clone());
    held.track_cycle(true);
    SqliteUserPreferencesRepository::new(pool.clone())
        .save(&held)
        .await
        .unwrap();

    SqliteCascadeDeleteRepository::new(pool.clone())
        .delete_user_account(user.id())
        .await
        .unwrap();

    let starts: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cycle_starts")
        .fetch_one(&pool)
        .await
        .unwrap();
    let preferences: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_preferences")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(starts.0, 0);
    assert_eq!(preferences.0, 0);
}
