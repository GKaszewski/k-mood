use std::sync::Arc;

use domain::provider::ProviderName;
use domain::song::{RecordingId, Song};
use domain::testing::{
    FakeCredentialCipher, FakeNowPlaying, FakeRecordingLookup, InMemoryStore, test_user,
};

use application::provider::commands::ConnectProviderCommand;
use application::provider::use_cases::{connect_provider, get_now_playing};

const RECORDING: &str = "f5c7e7a2-0000-4000-8000-000000000001";

async fn connected_store() -> (Arc<InMemoryStore>, domain::user::UserId) {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("alice");

    connect_provider::execute(
        ConnectProviderCommand {
            user_id: user.id().clone(),
            provider: ProviderName::new("subsonic").unwrap(),
            credential: br#"{"url":"https://music.example"}"#.to_vec(),
        },
        &connect_provider::Deps {
            command: store.clone(),
            cipher: Arc::new(FakeCredentialCipher),
        },
    )
    .await
    .unwrap();

    (store, user.id().clone())
}

fn song() -> Song {
    Song::new("Paranoid Android", "Radiohead", None, None).unwrap()
}

#[tokio::test]
async fn a_playing_song_is_returned_enriched_with_its_recording_id() {
    let (store, user_id) = connected_store().await;

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::playing("subsonic", song())),
        recordings: Arc::new(FakeRecordingLookup::finding(Some(
            RecordingId::new(RECORDING).unwrap(),
        ))),
    };

    let result = get_now_playing::execute(user_id, &deps).await.unwrap();
    let found = result.unwrap();

    assert_eq!(found.title().value(), "Paranoid Android");
    assert_eq!(
        found.recording_id().map(|id| id.value().to_string()),
        Some(RECORDING.to_string())
    );
}

#[tokio::test]
async fn a_song_with_no_musicbrainz_match_is_still_returned() {
    let (store, user_id) = connected_store().await;

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::playing("subsonic", song())),
        recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
    };

    let found = get_now_playing::execute(user_id, &deps)
        .await
        .unwrap()
        .unwrap();

    assert!(found.recording_id().is_none());
}

#[tokio::test]
async fn nothing_playing_is_not_an_error() {
    let (store, user_id) = connected_store().await;

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::silent("subsonic")),
        recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
    };

    assert!(
        get_now_playing::execute(user_id, &deps)
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn a_user_with_no_connection_gets_a_clear_error() {
    let store = Arc::new(InMemoryStore::new());
    let stranger = test_user("bob");

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::playing("subsonic", song())),
        recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
    };

    assert!(
        get_now_playing::execute(stranger.id().clone(), &deps)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn a_provider_that_cannot_be_reached_offers_nothing_rather_than_failing() {
    let (store, user_id) = connected_store().await;

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::failing("subsonic", "connection refused")),
        recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
    };

    let asked = get_now_playing::execute(user_id, &deps).await;

    assert!(
        asked.is_ok(),
        "the user must still be able to type a song by hand: {asked:?}"
    );
    assert!(asked.unwrap().is_none(), "nothing found, not an error");
}

#[tokio::test]
async fn a_recording_lookup_that_fails_yields_a_song_with_no_identity() {
    let (store, user_id) = connected_store().await;

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::playing("subsonic", song())),
        recordings: Arc::new(FakeRecordingLookup::failing("musicbrainz is rate limiting")),
    };

    let playing = get_now_playing::execute(user_id, &deps)
        .await
        .expect("a failing enrichment must not fail the lookup")
        .expect("the song is still playing");

    assert_eq!(playing.title().value(), song().title().value());
    assert!(
        playing.recording_id().is_none(),
        "an identity that could not be found is simply absent"
    );
}

#[tokio::test]
async fn a_recording_lookup_that_finds_nothing_yields_a_song_with_no_identity() {
    let (store, user_id) = connected_store().await;

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::playing("subsonic", song())),
        recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
    };

    let playing = get_now_playing::execute(user_id, &deps)
        .await
        .unwrap()
        .unwrap();

    assert!(playing.recording_id().is_none());
}

#[tokio::test]
async fn an_account_with_no_connection_is_told_so_rather_than_offered_silence() {
    let store = Arc::new(InMemoryStore::new());
    let user = test_user("nobody");

    let deps = get_now_playing::Deps {
        query: store.clone(),
        cipher: Arc::new(FakeCredentialCipher),
        now_playing: Arc::new(FakeNowPlaying::playing("subsonic", song())),
        recordings: Arc::new(FakeRecordingLookup::finding_nothing()),
    };

    let asked = get_now_playing::execute(user.id().clone(), &deps).await;

    assert!(
        asked.is_err(),
        "nothing to connect to is actionable, unlike a provider being down"
    );
}
