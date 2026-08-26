use sqlx::sqlite::SqlitePoolOptions;

use domain::entry::MoodEntryId;
use domain::job::{JobKind, JobStatus, JobSubject};
use domain::ports::{JobQueueCommandPort, JobQueueQueryPort};

use sqlite::repositories::SqliteJobQueueRepository;

const KIND: JobKind = JobKind::BackfillRecordingIdentity;

async fn a_queue() -> (sqlx::SqlitePool, SqliteJobQueueRepository) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    (pool.clone(), SqliteJobQueueRepository::new(pool))
}

fn about(entry_id: &MoodEntryId) -> JobSubject {
    JobSubject::Entry(entry_id.clone())
}

async fn status_of(pool: &sqlx::SqlitePool) -> Vec<(String, i64, Option<String>)> {
    sqlx::query_as("SELECT status, attempts, last_error FROM jobs ORDER BY enqueued_at")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn the_same_work_cannot_be_queued_twice() {
    let (pool, queue) = a_queue().await;
    let subject = about(&MoodEntryId::generate());

    assert!(queue.enqueue(KIND, &subject).await.unwrap());
    assert!(!queue.enqueue(KIND, &subject).await.unwrap());

    let rows: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(rows.0, 1);
}

#[tokio::test]
async fn claiming_marks_a_job_as_running_so_another_worker_leaves_it_alone() {
    let (pool, queue) = a_queue().await;
    queue
        .enqueue(KIND, &about(&MoodEntryId::generate()))
        .await
        .unwrap();

    let claimed = queue.claim(KIND, 10).await.unwrap();
    let claimed_again = queue.claim(KIND, 10).await.unwrap();

    assert_eq!(claimed.len(), 1);
    assert!(
        claimed_again.is_empty(),
        "a running job is not claimed twice"
    );
    assert_eq!(status_of(&pool).await[0].0, JobStatus::Running.name());
}

#[tokio::test]
async fn claiming_is_bounded_and_takes_the_oldest_first() {
    let (_, queue) = a_queue().await;
    for _ in 0..5 {
        queue
            .enqueue(KIND, &about(&MoodEntryId::generate()))
            .await
            .unwrap();
    }

    let claimed = queue.claim(KIND, 2).await.unwrap();

    assert_eq!(claimed.len(), 2);
}

#[tokio::test]
async fn finishing_a_job_removes_it() {
    let (pool, queue) = a_queue().await;
    queue
        .enqueue(KIND, &about(&MoodEntryId::generate()))
        .await
        .unwrap();
    let claimed = queue.claim(KIND, 1).await.unwrap();

    queue.finish(claimed[0].id()).await.unwrap();

    assert!(status_of(&pool).await.is_empty());
}

#[tokio::test]
async fn releasing_a_job_counts_the_attempt_and_keeps_the_reason() {
    let (pool, queue) = a_queue().await;
    queue
        .enqueue(KIND, &about(&MoodEntryId::generate()))
        .await
        .unwrap();
    let claimed = queue.claim(KIND, 1).await.unwrap();

    queue
        .release(claimed[0].id(), "musicbrainz timed out")
        .await
        .unwrap();

    let stored = status_of(&pool).await;
    assert_eq!(stored[0].0, JobStatus::Pending.name());
    assert_eq!(stored[0].1, 1);
    assert_eq!(stored[0].2.as_deref(), Some("musicbrainz timed out"));
    assert_eq!(
        queue.claim(KIND, 1).await.unwrap().len(),
        1,
        "and it is claimable again"
    );
}

#[tokio::test]
async fn an_exhausted_job_is_never_claimed_but_can_still_be_seen() {
    let (_, queue) = a_queue().await;
    queue
        .enqueue(KIND, &about(&MoodEntryId::generate()))
        .await
        .unwrap();
    let claimed = queue.claim(KIND, 1).await.unwrap();

    queue.exhaust(claimed[0].id(), "gave up").await.unwrap();

    assert!(queue.claim(KIND, 10).await.unwrap().is_empty());
    let visible = queue.find_exhausted(10).await.unwrap();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].last_error(), Some("gave up"));
}

#[tokio::test]
async fn a_job_left_running_by_a_dead_worker_becomes_claimable_again() {
    let (_, queue) = a_queue().await;
    queue
        .enqueue(KIND, &about(&MoodEntryId::generate()))
        .await
        .unwrap();
    queue.claim(KIND, 1).await.unwrap();

    let reclaimed = queue.reclaim_stalled(0).await.unwrap();

    assert_eq!(reclaimed, 1);
    assert_eq!(queue.claim(KIND, 1).await.unwrap().len(), 1);
}

#[tokio::test]
async fn a_job_still_being_worked_on_is_not_reclaimed() {
    let (_, queue) = a_queue().await;
    queue
        .enqueue(KIND, &about(&MoodEntryId::generate()))
        .await
        .unwrap();
    queue.claim(KIND, 1).await.unwrap();

    let reclaimed = queue.reclaim_stalled(300).await.unwrap();

    assert_eq!(reclaimed, 0, "five minutes have not passed");
}

#[tokio::test]
async fn a_row_of_a_kind_this_build_does_not_know_is_never_claimed() {
    let (pool, queue) = a_queue().await;

    sqlx::query(
        "INSERT INTO jobs (id, kind, subject, status, attempts, last_error, enqueued_at, updated_at)
         VALUES (?, 'summonRain', ?, 'pending', 0, NULL, ?, ?)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&pool)
    .await
    .unwrap();

    assert!(queue.claim(KIND, 10).await.unwrap().is_empty());
}

#[tokio::test]
async fn a_job_this_build_cannot_read_is_given_up_on_rather_than_claimed_forever() {
    let (pool, queue) = a_queue().await;

    sqlx::query(
        "INSERT INTO jobs (id, kind, subject, status, attempts, last_error, enqueued_at, updated_at)
         VALUES (?, 'backfillRecordingIdentity', ?, 'pending', 0, NULL, 'the day before yesterday', ?)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&pool)
    .await
    .unwrap();

    assert!(queue.claim(KIND, 10).await.unwrap().is_empty());

    let stored = status_of(&pool).await;
    assert_eq!(
        stored[0].0,
        JobStatus::Exhausted.name(),
        "an unreadable job must stop churning through claim and reclaim"
    );
    assert!(stored[0].2.is_some(), "and must say why it was given up on");

    assert_eq!(queue.reclaim_stalled(0).await.unwrap(), 0);
    assert!(queue.claim(KIND, 10).await.unwrap().is_empty());
}
