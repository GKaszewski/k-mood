use super::ApiToken;

pub struct MintedApiToken {
    token: ApiToken,
    secret: String,
}

impl MintedApiToken {
    pub fn new(token: ApiToken, secret: String) -> Self {
        Self { token, secret }
    }

    pub fn token(&self) -> &ApiToken {
        &self.token
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }
}

impl std::fmt::Debug for MintedApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MintedApiToken")
            .field("token", &self.token)
            .field("secret", &"<shown once, never logged>")
            .finish()
    }
}
