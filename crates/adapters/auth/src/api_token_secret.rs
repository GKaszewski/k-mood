use base64::Engine;
use rand::RngCore;
use sha2::{Digest, Sha256};

use domain::api_token::TokenDigest;

const SECRET_PREFIX: &str = "kmood_";
const SECRET_BYTES: usize = 32;

pub struct ApiTokenSecret;

impl domain::ports::ApiTokenSecretPort for ApiTokenSecret {
    fn mint(&self) -> String {
        let mut bytes = [0u8; SECRET_BYTES];
        rand::rng().fill_bytes(&mut bytes);

        let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

        format!("{SECRET_PREFIX}{body}")
    }

    fn digest(&self, secret: &str) -> TokenDigest {
        let digested = Sha256::digest(secret.as_bytes());

        TokenDigest::from_persistence(hex(&digested))
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
