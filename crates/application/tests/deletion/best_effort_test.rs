use std::sync::Arc;

use domain::api_token::{ApiToken, TokenDigest};
use domain::attachment::PhotoId;
use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::{Mood, MoodEntry};
use domain::ports::{
    ApiTokenSecretPort, DailyMetricQueryPort, EntryDimensionPort, MoodEntryCommandPort,
};
use domain::provider::ProviderName;
use domain::testing::{
    FakeApiTokenSecret, FakeMediaStorage, InMemoryDimensionStore, InMemoryStore, OneTokenStore,
    RefusingApiTokenStore, RefusingRejectionTrace,
};
use domain::user::UserId;

use application::api_token::use_cases::authenticate_api_token;
use application::entry::commands::UpdateEntryCommand;
use application::entry::use_cases::update_entry;
use application::import::commands::{ImportDailyMetricsCommand, ImportedDay, ImportedMetric};
use application::import::use_cases::import_daily_metrics;

#[tokio::test]
async fn a_blob_that_refuses_to_go_does_not_stop_the_entry_being_edited() {
    let store = Arc::new(InMemoryStore::new());
    let photos = Arc::new(InMemoryDimensionStore::new(DimensionKind::Photos));
    let media = Arc::new(FakeMediaStorage::refusing_to_delete());

    let owner = UserId::generate();
    let entry = MoodEntry::new(
        owner.clone(),
        Mood::Good,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T12:00:00+02:00").unwrap(),
    );
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();

    let removed = PhotoId::generate();
    media.put_photo(&removed, b"a photograph");
    photos
        .save(entry.id(), &[DimensionValue::Photos(vec![removed.clone()])])
        .await
        .unwrap();

    let deps = update_entry::Deps {
        command: store.clone(),
        dimensions: vec![photos.clone() as Arc<dyn EntryDimensionPort>],
        query: store.clone(),
        media_storage: media.clone(),
        events: store.clone(),
    };

    let edited = update_entry::execute(
        UpdateEntryCommand {
            entry_id: entry.id().clone(),
            mood: Mood::Rad,
            logged_at: Some(
                chrono::DateTime::parse_from_rfc3339("2026-08-20T12:00:00+02:00").unwrap(),
            ),
            dimensions: Vec::new(),
        },
        owner,
        &deps,
    )
    .await;

    assert!(
        edited.is_ok(),
        "a blob store that will not delete must not block an edit: {edited:?}"
    );
    assert_eq!(edited.unwrap().mood(), Mood::Rad);
    assert_eq!(
        media.deletions_attempted().len(),
        1,
        "the deletion was still attempted"
    );
}

#[tokio::test]
async fn an_unwritable_rejection_trace_does_not_fail_the_import() {
    let store = Arc::new(InMemoryStore::new());
    let user_id = UserId::generate();

    let deps = import_daily_metrics::Deps {
        metrics: store.clone(),
        rejections: Arc::new(RefusingRejectionTrace),
    };

    let outcome = import_daily_metrics::execute(
        ImportDailyMetricsCommand {
            user_id: user_id.clone(),
            provider: ProviderName::new("healthkit").unwrap(),
            days: vec![ImportedDay {
                date: "2026-08-20".into(),
                metrics: vec![
                    ImportedMetric {
                        kind: "steps".into(),
                        value: Some(8_412),
                    },
                    ImportedMetric {
                        kind: "hrv".into(),
                        value: Some(9_999),
                    },
                ],
            }],
            maximum_days: 90,
        },
        &deps,
    )
    .await
    .expect("a trace that cannot be written must not lose the good readings");

    assert_eq!(outcome.accepted, 1);
    assert_eq!(outcome.rejected.len(), 1, "and the caller is still told");

    let span = domain::entry::DateSpan::new(
        domain::entry::Date::from_persistence("2026-08-20".parse().unwrap()),
        domain::entry::Date::from_persistence("2026-08-20".parse().unwrap()),
    )
    .unwrap();
    let stored = DailyMetricQueryPort::find_by_span(store.as_ref(), &user_id, &span)
        .await
        .unwrap();
    assert_eq!(stored.len(), 1, "the valid reading was still stored");
}

#[tokio::test]
async fn a_token_store_that_cannot_record_a_use_still_authenticates() {
    let secrets = Arc::new(FakeApiTokenSecret::new());
    let secret = secrets.mint();

    let token = ApiToken::new(
        UserId::generate(),
        ProviderName::new("iphone-shortcuts").unwrap(),
        secrets.digest(&secret),
    );

    let deps = authenticate_api_token::Deps {
        query: Arc::new(OneTokenStore(token.clone())),
        command: Arc::new(RefusingApiTokenStore),
        secrets,
    };

    let authenticated = authenticate_api_token::execute(&secret, &deps).await;

    assert!(
        authenticated.is_ok(),
        "a token must keep working when its last-used stamp cannot be written: {authenticated:?}"
    );
    assert_eq!(authenticated.unwrap().id(), token.id());
}

#[tokio::test]
async fn a_digest_that_matches_nothing_is_still_refused() {
    let secrets = Arc::new(FakeApiTokenSecret::new());
    let real = secrets.mint();

    let token = ApiToken::new(
        UserId::generate(),
        ProviderName::new("iphone-shortcuts").unwrap(),
        TokenDigest::from_persistence("some other digest".into()),
    );

    let deps = authenticate_api_token::Deps {
        query: Arc::new(OneTokenStore(token)),
        command: Arc::new(RefusingApiTokenStore),
        secrets,
    };

    assert!(
        authenticate_api_token::execute(&real, &deps).await.is_err(),
        "degrading on a write failure must not degrade into accepting anything"
    );
}
