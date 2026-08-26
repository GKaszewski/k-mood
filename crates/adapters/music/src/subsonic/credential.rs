use domain::errors::DomainError;

#[derive(serde::Deserialize)]
pub struct SubsonicCredential {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl SubsonicCredential {
    pub fn parse(bytes: &[u8]) -> Result<Self, DomainError> {
        let credential: Self = serde_json::from_slice(bytes).map_err(|_| {
            DomainError::InvalidInput(
                "subsonic credential must carry a url, username and password".into(),
            )
        })?;

        if credential.url.trim().is_empty()
            || credential.username.trim().is_empty()
            || credential.password.is_empty()
        {
            return Err(DomainError::InvalidInput(
                "subsonic credential must carry a url, username and password".into(),
            ));
        }

        Ok(credential)
    }

    pub fn base_url(&self) -> &str {
        self.url.trim().trim_end_matches('/')
    }
}

impl std::fmt::Debug for SubsonicCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubsonicCredential")
            .field("url", &self.url)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}
