use domain::errors::DomainError;
use domain::ports::RecordingLookupPort;
use domain::song::RecordingId;

const SEARCH_URL: &str = "https://musicbrainz.org/ws/2/recording";
pub const USER_AGENT: &str = concat!("k-mood/", env!("CARGO_PKG_VERSION"));

pub struct MusicBrainzRecordingLookup {
    http: reqwest::Client,
}

impl MusicBrainzRecordingLookup {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }
}

#[derive(serde::Deserialize)]
struct SearchResult {
    #[serde(default)]
    recordings: Vec<Recording>,
}

#[derive(serde::Deserialize)]
struct Recording {
    id: String,
}

pub fn parse_first_recording(body: &str) -> Option<RecordingId> {
    let result: SearchResult = serde_json::from_str(body).ok()?;
    let first = result.recordings.first()?;
    RecordingId::new(&first.id).ok()
}

#[async_trait::async_trait]
impl RecordingLookupPort for MusicBrainzRecordingLookup {
    async fn find_recording(
        &self,
        title: &str,
        artist: &str,
    ) -> Result<Option<RecordingId>, DomainError> {
        let query = format!(r#"recording:"{title}" AND artist:"{artist}""#);

        let response = self
            .http
            .get(SEARCH_URL)
            .query(&[("query", query.as_str()), ("fmt", "json"), ("limit", "1")])
            .send()
            .await;

        let Ok(response) = response else {
            tracing::debug!("musicbrainz lookup failed");
            return Ok(None);
        };

        let Ok(body) = response.text().await else {
            return Ok(None);
        };

        Ok(parse_first_recording(&body))
    }
}
