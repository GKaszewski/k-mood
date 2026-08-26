use domain::entry::MoodEntryId;
use domain::job::{Job, JobKind, JobStatus, JobSubject};

fn a_job() -> Job {
    Job::pending(
        JobKind::BackfillRecordingIdentity,
        JobSubject::Entry(MoodEntryId::generate()),
    )
}

#[test]
fn a_new_job_is_waiting_and_has_never_been_tried() {
    let job = a_job();

    assert_eq!(job.status(), JobStatus::Pending);
    assert_eq!(job.attempts(), 0);
    assert!(job.last_error().is_none());
}

#[test]
fn a_job_with_attempts_left_is_worth_trying_again() {
    let mut job = a_job();

    job.attempted();

    assert_eq!(job.attempts(), 1);
    assert!(job.is_worth_another_attempt(3));
}

#[test]
fn a_job_that_has_used_every_attempt_is_not_tried_again() {
    let mut job = a_job();

    for _ in 0..3 {
        job.attempted();
    }

    assert_eq!(job.attempts(), 3);
    assert!(!job.is_worth_another_attempt(3));
}

#[test]
fn every_status_survives_a_round_trip_through_its_name() {
    for status in [JobStatus::Pending, JobStatus::Running, JobStatus::Exhausted] {
        assert_eq!(JobStatus::from_name(status.name()), Some(status));
    }

    assert_eq!(JobStatus::from_name("halfway"), None);
}

#[test]
fn every_kind_survives_a_round_trip_through_its_name() {
    for kind in JobKind::ALL {
        assert_eq!(JobKind::from_name(kind.name()), Some(kind));
    }

    assert_eq!(JobKind::from_name("summon-rain"), None);
}

#[test]
fn a_subject_names_the_thing_the_work_is_about() {
    let entry_id = MoodEntryId::generate();
    let subject = JobSubject::Entry(entry_id.clone());

    assert_eq!(subject.key(), entry_id.value().to_string());
    assert_eq!(
        JobSubject::from_key(subject.key().as_str()),
        Some(JobSubject::Entry(entry_id))
    );
}

#[test]
fn a_subject_that_is_not_an_identity_names_nothing() {
    assert!(JobSubject::from_key("not-a-uuid").is_none());
}
