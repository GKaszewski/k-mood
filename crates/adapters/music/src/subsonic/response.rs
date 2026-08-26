use domain::errors::DomainError;
use domain::song::Song;

#[derive(serde::Deserialize)]
struct Envelope {
    #[serde(rename = "subsonic-response")]
    response: SubsonicResponse,
}

#[derive(serde::Deserialize)]
struct SubsonicResponse {
    status: String,
    #[serde(rename = "nowPlaying")]
    now_playing: Option<NowPlaying>,
    error: Option<SubsonicError>,
}

#[derive(serde::Deserialize)]
struct SubsonicError {
    message: Option<String>,
}

#[derive(serde::Deserialize)]
struct NowPlaying {
    #[serde(default)]
    entry: Vec<NowPlayingEntry>,
}

#[derive(serde::Deserialize)]
struct NowPlayingEntry {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
}

pub fn parse_now_playing(body: &str) -> Result<Option<Song>, DomainError> {
    let envelope: Envelope = serde_json::from_str(body)
        .map_err(|_| DomainError::InvalidInput("unrecognised subsonic response".into()))?;

    if envelope.response.status != "ok" {
        let message = envelope
            .response
            .error
            .and_then(|error| error.message)
            .unwrap_or_else(|| "subsonic rejected the request".into());
        return Err(DomainError::InvalidInput(message));
    }

    let Some(entry) = envelope
        .response
        .now_playing
        .and_then(|playing| playing.entry.into_iter().next())
    else {
        return Ok(None);
    };

    let (Some(title), Some(artist)) = (entry.title, entry.artist) else {
        return Ok(None);
    };

    Ok(Some(Song::new(title, artist, entry.album, None)?))
}
