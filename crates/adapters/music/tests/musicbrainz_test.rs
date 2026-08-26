use music::musicbrainz::parse_first_recording;

#[test]
fn the_first_recording_is_taken_as_the_match() {
    let body = r#"{"recordings":[
        {"id":"f5c7e7a2-0000-4000-8000-000000000001","title":"Paranoid Android"},
        {"id":"f5c7e7a2-0000-4000-8000-000000000002","title":"Paranoid Android (live)"}
    ]}"#;

    let recording = parse_first_recording(body).unwrap();

    assert_eq!(
        recording.value().to_string(),
        "f5c7e7a2-0000-4000-8000-000000000001"
    );
}

#[test]
fn no_match_yields_nothing_rather_than_an_error() {
    assert!(parse_first_recording(r#"{"recordings":[]}"#).is_none());
    assert!(parse_first_recording(r#"{}"#).is_none());
    assert!(parse_first_recording("not json").is_none());
}

#[test]
fn a_recording_id_that_is_not_a_uuid_is_ignored() {
    let body = r#"{"recordings":[{"id":"not-a-uuid"}]}"#;

    assert!(parse_first_recording(body).is_none());
}
