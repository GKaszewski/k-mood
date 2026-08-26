use music::subsonic::{SubsonicCredential, auth_token, parse_now_playing};

#[test]
fn the_auth_token_is_md5_of_password_and_salt() {
    assert_eq!(
        auth_token("hunter2", "c19b2d"),
        "1b41ecef65ff7799cf7a84cf2d505e08"
    );
}

#[test]
fn a_different_salt_produces_a_different_token() {
    assert_ne!(auth_token("hunter2", "aaa"), auth_token("hunter2", "bbb"));
}

#[test]
fn a_credential_needs_a_url_username_and_password() {
    let valid = br#"{"url":"https://music.example/","username":"gabriel","password":"hunter2"}"#;
    let credential = SubsonicCredential::parse(valid).unwrap();

    assert_eq!(credential.base_url(), "https://music.example");
    assert_eq!(credential.username, "gabriel");
}

#[test]
fn an_incomplete_credential_is_rejected() {
    assert!(SubsonicCredential::parse(br#"{"url":"https://music.example"}"#).is_err());
    assert!(SubsonicCredential::parse(br#"{"url":"","username":"g","password":"p"}"#).is_err());
    assert!(SubsonicCredential::parse(b"not json").is_err());
}

#[test]
fn a_credential_never_reveals_its_password_in_debug_output() {
    let credential = SubsonicCredential::parse(
        br#"{"url":"https://music.example","username":"gabriel","password":"hunter2"}"#,
    )
    .unwrap();

    assert!(!format!("{credential:?}").contains("hunter2"));
}

#[test]
fn a_playing_track_becomes_a_song() {
    let body = r#"{"subsonic-response":{"status":"ok","version":"1.16.1","nowPlaying":{"entry":[
        {"title":"Paranoid Android","artist":"Radiohead","album":"OK Computer"}
    ]}}}"#;

    let song = parse_now_playing(body).unwrap().unwrap();

    assert_eq!(song.title().value(), "Paranoid Android");
    assert_eq!(song.artist().value(), "Radiohead");
    assert_eq!(song.album().map(|a| a.value()), Some("OK Computer"));
}

#[test]
fn nothing_playing_is_not_an_error() {
    let empty = r#"{"subsonic-response":{"status":"ok","version":"1.16.1"}}"#;
    let no_entries = r#"{"subsonic-response":{"status":"ok","nowPlaying":{"entry":[]}}}"#;

    assert!(parse_now_playing(empty).unwrap().is_none());
    assert!(parse_now_playing(no_entries).unwrap().is_none());
}

#[test]
fn a_failed_call_is_an_error_even_though_subsonic_answers_with_http_200() {
    let body = r#"{"subsonic-response":{"status":"failed","version":"1.16.1",
        "error":{"code":40,"message":"Wrong username or password."}}}"#;

    let result = parse_now_playing(body);

    assert!(result.is_err());
}

#[test]
fn a_track_missing_its_artist_is_skipped_rather_than_half_stored() {
    let body = r#"{"subsonic-response":{"status":"ok","nowPlaying":{"entry":[
        {"title":"Untitled"}
    ]}}}"#;

    assert!(parse_now_playing(body).unwrap().is_none());
}
