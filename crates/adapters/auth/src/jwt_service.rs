use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use config::AuthConfig;
use domain::auth::GeneratedToken;
use domain::errors::DomainError;
use domain::user::UserId;

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

pub struct JwtAuthService {
    secret: String,
    ttl_seconds: i64,
}

impl JwtAuthService {
    pub fn new(config: &AuthConfig) -> Result<Self, DomainError> {
        let secret = config
            .jwt_secret
            .clone()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| DomainError::InvalidInput("JWT secret must be configured".into()))?;

        Ok(Self {
            secret,
            ttl_seconds: config.access_token_ttl_seconds as i64,
        })
    }
}

#[async_trait::async_trait]
impl domain::ports::AuthServicePort for JwtAuthService {
    async fn generate_token(&self, user_id: &UserId) -> Result<GeneratedToken, DomainError> {
        let expires_at = Utc::now() + Duration::seconds(self.ttl_seconds);

        let claims = Claims {
            sub: user_id.value().to_string(),
            exp: expires_at.timestamp() as u64,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::InvalidInput(format!("failed to generate token: {e}")))?;

        Ok(GeneratedToken::new(token, expires_at))
    }

    async fn validate_token(&self, token: &str) -> Result<UserId, DomainError> {
        let data = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::Unauthorized("invalid or expired token".into()))?;

        let uuid: uuid::Uuid = data
            .claims
            .sub
            .parse()
            .map_err(|_| DomainError::Unauthorized("invalid token subject".into()))?;

        Ok(UserId::from_uuid(uuid))
    }
}
