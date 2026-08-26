use domain::song::{RecordingId, Song};

#[test]
fn a_song_needs_only_a_title_and_an_artist() {
    let song = Song::new("Paranoid Android", "Radiohead", None, None).unwrap();

    assert_eq!(song.title().value(), "Paranoid Android");
    assert_eq!(song.artist().value(), "Radiohead");
    assert!(song.album().is_none());
    assert!(song.recording_id().is_none());
}

#[test]
fn a_song_records_its_album_and_external_identity_when_known() {
    let recording = RecordingId::new("f5c7e7a2-0000-4000-8000-000000000001").unwrap();
    let song = Song::new(
        "Paranoid Android",
        "Radiohead",
        Some("OK Computer".into()),
        Some(recording),
    )
    .unwrap();

    assert_eq!(song.album().map(|a| a.value()), Some("OK Computer"));
    assert!(song.recording_id().is_some());
}

#[test]
fn a_song_without_a_title_or_artist_is_rejected() {
    assert!(Song::new("", "Radiohead", None, None).is_err());
    assert!(Song::new("Paranoid Android", "   ", None, None).is_err());
}

#[test]
fn a_recording_id_that_is_not_a_uuid_is_rejected() {
    assert!(RecordingId::new("not-a-musicbrainz-id").is_err());
}

#[test]
fn a_blank_album_is_treated_as_absent() {
    let song = Song::new("Airbag", "Radiohead", Some("   ".into()), None).unwrap();

    assert!(song.album().is_none());
}
