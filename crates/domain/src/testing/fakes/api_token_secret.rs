use std::sync::atomic::{AtomicUsize, Ordering};

use crate::api_token::TokenDigest;

pub struct FakeApiTokenSecret {
    minted: AtomicUsize,
}

impl FakeApiTokenSecret {
    pub fn new() -> Self {
        Self {
            minted: AtomicUsize::new(0),
        }
    }
}

impl Default for FakeApiTokenSecret {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::ports::ApiTokenSecretPort for FakeApiTokenSecret {
    fn mint(&self) -> String {
        let number = self.minted.fetch_add(1, Ordering::SeqCst);

        format!("kmood_secret_{number}")
    }

    fn digest(&self, secret: &str) -> TokenDigest {
        TokenDigest::from_persistence(format!("digest-of-{secret}"))
    }
}
