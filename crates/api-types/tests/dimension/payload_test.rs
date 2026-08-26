use api_types::dimension::DimensionPayload;
use api_types::requests::CreateEntryRequest;

#[test]
fn a_known_dimension_kind_is_accepted() {
    let payload: DimensionPayload =
        serde_json::from_str(r#"{"kind":"content","text":"went for a walk"}"#).unwrap();

    assert!(matches!(payload, DimensionPayload::Content { text } if text == "went for a walk"));
}

#[test]
fn an_unknown_dimension_kind_is_rejected() {
    let result = serde_json::from_str::<DimensionPayload>(r#"{"kind":"telepathy","text":"hi"}"#);

    assert!(result.is_err());
}

#[test]
fn an_entry_request_carrying_an_unknown_dimension_kind_is_rejected() {
    let result = serde_json::from_str::<CreateEntryRequest>(
        r#"{"mood":4,"dimensions":[{"kind":"telepathy","text":"hi"}]}"#,
    );

    assert!(result.is_err());
}

#[test]
fn an_entry_request_with_no_dimensions_is_accepted() {
    let request: CreateEntryRequest = serde_json::from_str(r#"{"mood":4}"#).unwrap();

    assert!(request.dimensions.is_none());
}

#[test]
fn a_location_payload_carries_both_coordinates() {
    let payload: DimensionPayload =
        serde_json::from_str(r#"{"kind":"location","latitude":52.2297,"longitude":21.0122}"#)
            .unwrap();

    assert!(matches!(
        payload,
        DimensionPayload::Location { latitude, longitude }
            if (latitude - 52.2297).abs() < f64::EPSILON
                && (longitude - 21.0122).abs() < f64::EPSILON
    ));
}

#[test]
fn a_song_payload_needs_only_a_title_and_artist() {
    let payload: DimensionPayload =
        serde_json::from_str(r#"{"kind":"song","title":"Airbag","artist":"Radiohead"}"#).unwrap();

    assert!(matches!(
        payload,
        DimensionPayload::Song {
            album: None,
            recording_id: None,
            ..
        }
    ));
}

#[test]
fn a_song_payload_carries_its_album_and_recording_id_when_present() {
    let payload: DimensionPayload = serde_json::from_str(
        r#"{"kind":"song","title":"Airbag","artist":"Radiohead","album":"OK Computer","recordingId":"f5c7e7a2-0000-4000-8000-000000000001"}"#,
    )
    .unwrap();

    assert!(matches!(
        payload,
        DimensionPayload::Song {
            album: Some(_),
            recording_id: Some(_),
            ..
        }
    ));
}
