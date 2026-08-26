use md5::{Digest, Md5};
use rand::Rng;

use domain::errors::DomainError;
use domain::ports::NowPlayingPort;
use domain::song::Song;

use super::credential::SubsonicCredential;
use super::response::parse_now_playing;

const API_VERSION: &str = "1.16.1";
const CLIENT_NAME: &str = "k-mood";
const SALT_BYTES: usize = 12;

pub struct SubsonicNowPlayingAdapter {
    http: reqwest::Client,
}

impl SubsonicNowPlayingAdapter {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }
}

pub fn auth_token(password: &str, salt: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn random_salt() -> String {
    let mut bytes = [0u8; SALT_BYTES];
    rand::rng().fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[async_trait::async_trait]
impl NowPlayingPort for SubsonicNowPlayingAdapter {
    fn provider(&self) -> &str {
        "subsonic"
    }

    async fn now_playing(&self, credential: &[u8]) -> Result<Option<Song>, DomainError> {
        let credential = SubsonicCredential::parse(credential)?;
        let salt = random_salt();
        let token = auth_token(&credential.password, &salt);

        let url = format!("{}/rest/getNowPlaying", credential.base_url());

        let response = self
            .http
            .get(&url)
            .query(&[
                ("u", credential.username.as_str()),
                ("t", token.as_str()),
                ("s", salt.as_str()),
                ("v", API_VERSION),
                ("c", CLIENT_NAME),
                ("f", "json"),
            ])
            .send()
            .await
            .map_err(|e| {
                tracing::warn!(%e, "subsonic request failed");
                DomainError::InvalidInput("could not reach the music provider".into())
            })?;

        let body = response.text().await.map_err(|e| {
            tracing::warn!(%e, "subsonic response could not be read");
            DomainError::InvalidInput("could not read the music provider's response".into())
        })?;

        parse_now_playing(&body)
    }
}
