use sqlx::sqlite::SqlitePoolOptions;

use domain::entry::{Date, DateSpan};
use domain::metric::{DailyMetric, Hrv, MetricKind, MetricValue, Source, Steps};
use domain::ports::{
    CascadeDeletePort, DailyMetricCommandPort, DailyMetricQueryPort, UserCommandPort,
};
use domain::provider::ProviderName;
use domain::testing::test_user;
use domain::user::UserId;

use sqlite::repositories::{
    SqliteCascadeDeleteRepository, SqliteDailyMetricCommandRepository,
    SqliteDailyMetricQueryRepository, SqliteRejectionRepository, SqliteUserCommandRepository,
};

const REJECTIONS_KEPT: usize = 200;

fn a_trace(pool: sqlx::SqlitePool) -> std::sync::Arc<dyn domain::ports::RejectionCommandPort> {
    std::sync::Arc::new(SqliteRejectionRepository::new(pool, REJECTIONS_KEPT))
}

async fn a_pool_with_a_user() -> (sqlx::SqlitePool, UserId) {
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

    (pool, user.id().clone())
}

fn on(day: &str) -> Date {
    Date::from_persistence(day.parse().unwrap())
}

fn steps(count: u32) -> MetricValue {
    MetricValue::Steps(Steps::new(count).unwrap())
}

fn from_provider() -> Source {
    Source::Provider(ProviderName::new("healthkit").unwrap())
}

async fn stored_for(pool: &sqlx::SqlitePool, user_id: &UserId, day: &str) -> Vec<DailyMetric> {
    let span = DateSpan::new(on(day), on(day)).unwrap();
    SqliteDailyMetricQueryRepository::new(pool.clone(), a_trace(pool.clone()))
        .find_by_span(user_id, &span)
        .await
        .unwrap()
}

#[tokio::test]
async fn restating_a_date_replaces_the_row_rather_than_adding_one() {
    let (pool, user_id) = a_pool_with_a_user().await;
    let metrics = SqliteDailyMetricCommandRepository::new(pool.clone());

    for count in [8_000, 8_412] {
        metrics
            .save(&[DailyMetric::new(
                user_id.clone(),
                on("2026-08-20"),
                steps(count),
                Source::Manual,
            )])
            .await
            .unwrap();
    }

    let stored = stored_for(&pool, &user_id, "2026-08-20").await;

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].value(), &steps(8_412));
}

#[tokio::test]
async fn an_import_leaves_a_count_the_user_stated_alone() {
    let (pool, user_id) = a_pool_with_a_user().await;
    let metrics = SqliteDailyMetricCommandRepository::new(pool.clone());

    metrics
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();
    metrics
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(1_000),
            from_provider(),
        )])
        .await
        .unwrap();

    let stored = stored_for(&pool, &user_id, "2026-08-20").await;

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].value(), &steps(8_412));
    assert_eq!(stored[0].source(), &Source::Manual);
}

#[tokio::test]
async fn a_count_the_user_states_replaces_what_a_provider_reported() {
    let (pool, user_id) = a_pool_with_a_user().await;
    let metrics = SqliteDailyMetricCommandRepository::new(pool.clone());

    metrics
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(1_000),
            from_provider(),
        )])
        .await
        .unwrap();
    metrics
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();

    let stored = stored_for(&pool, &user_id, "2026-08-20").await;

    assert_eq!(stored[0].value(), &steps(8_412));
    assert_eq!(stored[0].source(), &Source::Manual);
}

#[tokio::test]
async fn a_provider_is_remembered_as_the_source() {
    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(1_000),
            from_provider(),
        )])
        .await
        .unwrap();

    let stored = stored_for(&pool, &user_id, "2026-08-20").await;

    assert_eq!(stored[0].source(), &from_provider());
}

#[tokio::test]
async fn a_row_of_a_kind_this_build_does_not_know_is_skipped_and_its_neighbours_survive() {
    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();

    sqlx::query("INSERT INTO daily_metrics (user_id, date, kind, value, provider) VALUES (?, ?, ?, ?, NULL)")
        .bind(user_id.value().to_string())
        .bind("2026-08-20")
        .bind("telepathy")
        .bind(42)
        .execute(&pool)
        .await
        .unwrap();

    let stored = stored_for(&pool, &user_id, "2026-08-20").await;

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].value(), &steps(8_412));
}

#[tokio::test]
async fn a_value_outside_its_range_is_skipped_and_its_neighbours_survive() {
    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-19"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();

    sqlx::query("INSERT INTO daily_metrics (user_id, date, kind, value, provider) VALUES (?, ?, ?, ?, NULL)")
        .bind(user_id.value().to_string())
        .bind("2026-08-20")
        .bind("steps")
        .bind(900_000)
        .execute(&pool)
        .await
        .unwrap();

    let span = DateSpan::new(on("2026-08-19"), on("2026-08-20")).unwrap();
    let stored = SqliteDailyMetricQueryRepository::new(pool.clone(), a_trace(pool.clone()))
        .find_by_span(&user_id, &span)
        .await
        .unwrap();

    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].date(), &on("2026-08-19"));
}

#[tokio::test]
async fn days_outside_the_span_are_not_returned() {
    let (pool, user_id) = a_pool_with_a_user().await;
    let metrics = SqliteDailyMetricCommandRepository::new(pool.clone());

    for day in ["2026-07-31", "2026-08-01", "2026-08-31", "2026-09-01"] {
        metrics
            .save(&[DailyMetric::new(
                user_id.clone(),
                on(day),
                steps(8_412),
                Source::Manual,
            )])
            .await
            .unwrap();
    }

    let span = DateSpan::new(on("2026-08-01"), on("2026-08-31")).unwrap();
    let stored = SqliteDailyMetricQueryRepository::new(pool.clone(), a_trace(pool.clone()))
        .find_by_span(&user_id, &span)
        .await
        .unwrap();

    let days: Vec<String> = stored
        .iter()
        .map(|metric| metric.date().to_string())
        .collect();

    assert_eq!(days, ["2026-08-01", "2026-08-31"]);
}

#[tokio::test]
async fn clearing_a_users_data_removes_their_days() {
    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();

    SqliteCascadeDeleteRepository::new(pool.clone())
        .delete_all_user_data(&user_id)
        .await
        .unwrap();

    assert!(stored_for(&pool, &user_id, "2026-08-20").await.is_empty());
}

#[tokio::test]
async fn deleting_an_account_removes_its_days() {
    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-20"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();

    SqliteCascadeDeleteRepository::new(pool.clone())
        .delete_user_account(&user_id)
        .await
        .unwrap();

    let remaining: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM daily_metrics")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(remaining.0, 0);
}

#[tokio::test]
async fn clearing_a_kind_deletes_only_that_row() {
    let (pool, user_id) = a_pool_with_a_user().await;
    let metrics = SqliteDailyMetricCommandRepository::new(pool.clone());

    metrics
        .save(&[
            DailyMetric::new(
                user_id.clone(),
                on("2026-08-20"),
                steps(8_412),
                Source::Manual,
            ),
            DailyMetric::new(
                user_id.clone(),
                on("2026-08-20"),
                MetricValue::Hrv(Hrv::new(61).unwrap()),
                Source::Manual,
            ),
            DailyMetric::new(
                user_id.clone(),
                on("2026-08-21"),
                steps(9_000),
                Source::Manual,
            ),
        ])
        .await
        .unwrap();

    metrics
        .delete(&user_id, &on("2026-08-20"), &[MetricKind::Steps])
        .await
        .unwrap();

    let remaining = stored_for(&pool, &user_id, "2026-08-20").await;

    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].kind(), MetricKind::Hrv);
    assert_eq!(stored_for(&pool, &user_id, "2026-08-21").await.len(), 1);
}

#[tokio::test]
async fn clearing_a_kind_that_was_never_stored_is_not_an_error() {
    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .delete(&user_id, &on("2026-08-20"), &[MetricKind::Steps])
        .await
        .unwrap();

    assert!(stored_for(&pool, &user_id, "2026-08-20").await.is_empty());
}

#[tokio::test]
async fn clearing_a_kind_leaves_another_accounts_day_alone() {
    let (pool, mine) = a_pool_with_a_user().await;
    let theirs = test_user("bob");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&theirs)
        .await
        .unwrap();

    let metrics = SqliteDailyMetricCommandRepository::new(pool.clone());
    for owner in [&mine, theirs.id()] {
        metrics
            .save(&[DailyMetric::new(
                owner.clone(),
                on("2026-08-20"),
                steps(8_412),
                Source::Manual,
            )])
            .await
            .unwrap();
    }

    metrics
        .delete(&mine, &on("2026-08-20"), &[MetricKind::Steps])
        .await
        .unwrap();

    assert!(stored_for(&pool, &mine, "2026-08-20").await.is_empty());
    assert_eq!(
        stored_for(&pool, theirs.id(), "2026-08-20").await.len(),
        1,
        "the other account's day was cleared too"
    );
}

#[tokio::test]
async fn a_stored_row_this_build_cannot_read_lands_in_the_rejection_trace() {
    use domain::ports::RejectionQueryPort;
    use domain::rejection::RejectionOrigin;

    let (pool, user_id) = a_pool_with_a_user().await;

    SqliteDailyMetricCommandRepository::new(pool.clone())
        .save(&[DailyMetric::new(
            user_id.clone(),
            on("2026-08-19"),
            steps(8_412),
            Source::Manual,
        )])
        .await
        .unwrap();

    for (kind, value) in [("telepathy", 42), ("steps", 900_000)] {
        sqlx::query(
            "INSERT INTO daily_metrics (user_id, date, kind, value, provider) VALUES (?, ?, ?, ?, NULL)",
        )
        .bind(user_id.value().to_string())
        .bind("2026-08-20")
        .bind(kind)
        .bind(value)
        .execute(&pool)
        .await
        .unwrap();
    }

    let span = DateSpan::new(on("2026-08-19"), on("2026-08-20")).unwrap();
    let readable = SqliteDailyMetricQueryRepository::new(pool.clone(), a_trace(pool.clone()))
        .find_by_span(&user_id, &span)
        .await
        .unwrap();

    let trace = SqliteRejectionRepository::new(pool.clone(), REJECTIONS_KEPT)
        .find_recent_by_user(&user_id)
        .await
        .unwrap();

    assert_eq!(readable.len(), 1, "the good row still comes back");
    assert_eq!(trace.len(), 2, "both unreadable rows are recorded");
    assert!(
        trace
            .iter()
            .all(|entry| entry.origin() == RejectionOrigin::StoredRow)
    );

    let mut kinds: Vec<&str> = trace.iter().map(|entry| entry.detail().kind()).collect();
    kinds.sort_unstable();
    assert_eq!(kinds, ["steps", "telepathy"]);
}
