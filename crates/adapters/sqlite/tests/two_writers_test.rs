use domain::entry::MoodEntryId;
use domain::job::{JobKind, JobSubject};
use domain::ports::JobQueueCommandPort;

use sqlite::repositories::SqliteJobQueueRepository;

const KIND: JobKind = JobKind::BackfillRecordingIdentity;

fn a_file() -> String {
    let name = format!("k-mood-two-writers-{}.sqlite", uuid::Uuid::new_v4());

    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .to_string()
}

#[tokio::test]
async fn two_processes_writing_one_file_both_succeed() {
    let path = a_file();
    let url = format!("sqlite://{path}");

    let server = sqlite::create_pool(&url).await.unwrap();
    sqlite::run_migrations(&server).await.unwrap();

    let worker = sqlite::create_pool(&url).await.unwrap();

    let enqueuing = tokio::spawn({
        let queue = SqliteJobQueueRepository::new(server.clone());
        async move {
            for _ in 0..80 {
                queue
                    .enqueue(KIND, &JobSubject::Entry(MoodEntryId::generate()))
                    .await
                    .expect("the server's write should not be refused");
            }
        }
    });

    let working = tokio::spawn({
        let queue = SqliteJobQueueRepository::new(worker.clone());
        async move {
            let mut claimed = 0;
            for _ in 0..80 {
                let batch = queue
                    .claim(KIND, 5)
                    .await
                    .expect("the worker's write should not be refused");

                for job in &batch {
                    queue
                        .finish(job.id())
                        .await
                        .expect("finishing should not be refused");
                }

                claimed += batch.len();
            }

            claimed
        }
    });

    enqueuing.await.unwrap();
    let claimed = working.await.unwrap();

    assert!(claimed > 0, "the worker never saw any of the server's work");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}-wal"));
    let _ = std::fs::remove_file(format!("{path}-shm"));
}

#[tokio::test]
async fn a_writer_waits_for_a_held_lock_rather_than_being_refused() {
    let path = a_file();
    let url = format!("sqlite://{path}");

    let holder = sqlite::create_pool(&url).await.unwrap();
    sqlite::run_migrations(&holder).await.unwrap();
    let other = sqlite::create_pool(&url).await.unwrap();

    let mut held = holder.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO jobs (id, kind, subject, status, attempts, last_error, enqueued_at, updated_at)
         VALUES (?, 'backfillRecordingIdentity', ?, 'pending', 0, NULL, ?, ?)",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(&mut *held)
    .await
    .unwrap();

    let waiting = tokio::spawn({
        let queue = SqliteJobQueueRepository::new(other);
        async move {
            queue
                .enqueue(KIND, &JobSubject::Entry(MoodEntryId::generate()))
                .await
        }
    });

    tokio::time::sleep(std::time::Duration::from_millis(400)).await;
    held.commit().await.unwrap();

    let written = waiting.await.unwrap();

    assert!(
        written.is_ok(),
        "the second writer was refused instead of waiting: {written:?}"
    );

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(format!("{path}-wal"));
    let _ = std::fs::remove_file(format!("{path}-shm"));
}
