use sqlx::sqlite::SqlitePoolOptions;

use domain::entry::Date;
use domain::ports::{CascadeDeletePort, RejectionCommandPort, RejectionQueryPort, UserCommandPort};
use domain::provider::ProviderName;
use domain::rejection::{RejectedMetric, RejectionDetail, RejectionOrigin};
use domain::testing::test_user;
use domain::user::{User, UserId};

use sqlite::repositories::{
    SqliteCascadeDeleteRepository, SqliteRejectionRepository, SqliteUserCommandRepository,
};

const KEPT: usize = 5;

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

fn rejected(owner: &UserId, kind: &str, reason: &str) -> RejectedMetric {
    RejectedMetric::new(
        owner.clone(),
        RejectionOrigin::Import,
        RejectionDetail::new(
            Some(ProviderName::new("healthkit").unwrap()),
            Some(Date::from_persistence("2026-08-20".parse().unwrap())),
            kind,
            Some(9_999),
        ),
        reason,
    )
}

#[tokio::test]
async fn a_recorded_rejection_comes_back_with_everything_needed_to_diagnose_it() {
    let (pool, user) = a_pool_with_a_user().await;
    let trace = SqliteRejectionRepository::new(pool.clone(), KEPT);

    trace
        .record(&[rejected(
            user.id(),
            "hrv",
            "heart rate variability must be between 1 and 300",
        )])
        .await
        .unwrap();

    let found = trace.find_recent_by_user(user.id()).await.unwrap();

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].origin(), RejectionOrigin::Import);
    assert_eq!(found[0].detail().kind(), "hrv");
    assert_eq!(found[0].detail().value(), Some(9_999));
    assert_eq!(found[0].detail().date().unwrap().to_string(), "2026-08-20");
    assert_eq!(found[0].detail().provider().unwrap().value(), "healthkit");
    assert!(found[0].reason().contains("between 1 and 300"));
}

#[tokio::test]
async fn only_the_most_recent_rejections_are_kept() {
    let (pool, user) = a_pool_with_a_user().await;
    let trace = SqliteRejectionRepository::new(pool.clone(), KEPT);

    for number in 0..12 {
        trace
            .record(&[rejected(user.id(), "hrv", &format!("failure {number}"))])
            .await
            .unwrap();
    }

    let found = trace.find_recent_by_user(user.id()).await.unwrap();

    assert_eq!(found.len(), KEPT);

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM metric_rejections")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(total.0, KEPT as i64, "older rows should have been pruned");
}

#[tokio::test]
async fn one_accounts_rejections_are_never_shown_to_another() {
    let (pool, mine) = a_pool_with_a_user().await;
    let theirs = test_user("bob");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&theirs)
        .await
        .unwrap();

    let trace = SqliteRejectionRepository::new(pool.clone(), KEPT);
    trace
        .record(&[rejected(mine.id(), "hrv", "mine")])
        .await
        .unwrap();
    trace
        .record(&[rejected(theirs.id(), "steps", "theirs")])
        .await
        .unwrap();

    let found = trace.find_recent_by_user(mine.id()).await.unwrap();

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].reason(), "mine");
}

#[tokio::test]
async fn pruning_one_account_leaves_anothers_alone() {
    let (pool, mine) = a_pool_with_a_user().await;
    let theirs = test_user("bob");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&theirs)
        .await
        .unwrap();

    let trace = SqliteRejectionRepository::new(pool.clone(), KEPT);
    trace
        .record(&[rejected(theirs.id(), "steps", "theirs")])
        .await
        .unwrap();

    for number in 0..12 {
        trace
            .record(&[rejected(mine.id(), "hrv", &format!("failure {number}"))])
            .await
            .unwrap();
    }

    assert_eq!(
        trace.find_recent_by_user(theirs.id()).await.unwrap().len(),
        1
    );
}

#[tokio::test]
async fn a_rejection_with_no_date_is_still_recorded() {
    let (pool, user) = a_pool_with_a_user().await;
    let trace = SqliteRejectionRepository::new(pool.clone(), KEPT);

    trace
        .record(&[RejectedMetric::new(
            user.id().clone(),
            RejectionOrigin::Import,
            RejectionDetail::new(None, None, "steps", None),
            "yesterday is not a date",
        )])
        .await
        .unwrap();

    let found = trace.find_recent_by_user(user.id()).await.unwrap();

    assert_eq!(found.len(), 1);
    assert!(found[0].detail().date().is_none());
    assert!(found[0].detail().provider().is_none());
    assert!(found[0].detail().value().is_none());
}

#[tokio::test]
async fn a_row_with_an_origin_this_build_does_not_know_is_skipped() {
    let (pool, user) = a_pool_with_a_user().await;
    let trace = SqliteRejectionRepository::new(pool.clone(), KEPT);
    trace
        .record(&[rejected(user.id(), "hrv", "readable")])
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO metric_rejections
         (id, user_id, origin, provider, date, kind, value, reason, recorded_at)
         VALUES (?, ?, 'telepathy', NULL, NULL, 'steps', NULL, 'from the future', ?)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(user.id().value().to_string())
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&pool)
    .await
    .unwrap();

    let found = trace.find_recent_by_user(user.id()).await.unwrap();

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].reason(), "readable");
}

#[tokio::test]
async fn deleting_an_account_takes_its_rejections_with_it() {
    let (pool, user) = a_pool_with_a_user().await;
    SqliteRejectionRepository::new(pool.clone(), KEPT)
        .record(&[rejected(user.id(), "hrv", "whatever")])
        .await
        .unwrap();

    SqliteCascadeDeleteRepository::new(pool.clone())
        .delete_user_account(user.id())
        .await
        .unwrap();

    let remaining: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM metric_rejections")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(remaining.0, 0);
}
