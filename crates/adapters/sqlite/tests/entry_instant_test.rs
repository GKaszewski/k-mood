use chrono::{DateTime, FixedOffset};
use sqlx::SqlitePool;

use domain::entry::{DateRange, Mood, MoodEntry};
use domain::ports::{MoodEntryCommandPort, MoodEntryQueryPort, UserCommandPort};
use domain::testing::test_user;
use domain::user::User;

use sqlite::repositories::{
    SqliteEntryCommandRepository, SqliteEntryQueryRepository, SqliteUserCommandRepository,
};

fn a_file() -> String {
    let name = format!("k-mood-instants-{}.sqlite", uuid::Uuid::new_v4());

    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .to_string()
}

async fn a_pool_with_a_user() -> (SqlitePool, User) {
    let path = a_file();
    let pool = sqlite::create_pool(&format!("sqlite://{path}"))
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

fn instant(text: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(text).unwrap()
}

async fn save(pool: &SqlitePool, user: &User, logged_at: &str) -> MoodEntry {
    let entry = MoodEntry::new(user.id().clone(), Mood::Good, instant(logged_at));
    SqliteEntryCommandRepository::new(pool.clone())
        .save(&entry)
        .await
        .unwrap();

    entry
}

async fn stored_text(pool: &SqlitePool, entry: &MoodEntry) -> String {
    let (held,): (String,) = sqlx::query_as("SELECT logged_at FROM mood_entries WHERE id = ?")
        .bind(entry.id().value().to_string())
        .fetch_one(pool)
        .await
        .unwrap();

    held
}

#[tokio::test]
async fn an_instant_is_held_in_utc_whatever_offset_it_arrived_in() {
    let (pool, user) = a_pool_with_a_user().await;

    let entry = save(&pool, &user, "2026-08-25T20:00:00+02:00").await;

    assert_eq!(
        stored_text(&pool, &entry).await,
        "2026-08-25T18:00:00+00:00",
        "one canonical form makes the column sortable"
    );
}

#[tokio::test]
async fn an_instant_survives_the_round_trip_unchanged() {
    let (pool, user) = a_pool_with_a_user().await;

    let entry = save(&pool, &user, "2026-08-25T20:00:00+02:00").await;

    let read = SqliteEntryQueryRepository::new(pool.clone())
        .find_by_id(entry.id())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        read.logged_at(),
        entry.logged_at(),
        "the instant is the fact; the offset it was written in is not"
    );
}

#[tokio::test]
async fn a_range_finds_entries_written_in_other_offsets() {
    let (pool, user) = a_pool_with_a_user().await;

    let eastern = save(&pool, &user, "2026-08-25T01:30:00+03:00").await;
    let western = save(&pool, &user, "2026-08-24T21:30:00-04:00").await;

    let whole_of_the_24th_utc = DateRange::new(
        instant("2026-08-24T00:00:00+00:00"),
        instant("2026-08-24T23:59:59+00:00"),
    )
    .unwrap();

    let found = SqliteEntryQueryRepository::new(pool.clone())
        .find_by_date_range(user.id(), &whole_of_the_24th_utc)
        .await
        .unwrap();

    let ids: Vec<_> = found.iter().map(|entry| entry.id().clone()).collect();

    assert!(
        ids.contains(eastern.id()),
        "01:30+03:00 is 22:30Z on the 24th and belongs in the range"
    );
    assert!(
        !ids.contains(western.id()),
        "21:30-04:00 is 01:30Z on the 25th, so it falls outside the range"
    );
}

#[tokio::test]
async fn entries_come_back_newest_first_across_mixed_offsets() {
    let (pool, user) = a_pool_with_a_user().await;

    let earlier = save(&pool, &user, "2026-08-25T00:30:00+02:00").await;
    let later = save(&pool, &user, "2026-08-25T00:00:00+00:00").await;

    let found = SqliteEntryQueryRepository::new(pool.clone())
        .find_all_by_user(user.id())
        .await
        .unwrap();

    assert_eq!(
        found[0].id(),
        later.id(),
        "22:30Z precedes 00:00Z, however the offsets sort as text"
    );
    assert_eq!(found[1].id(), earlier.id());
}
