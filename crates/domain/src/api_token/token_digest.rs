#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TokenDigest(String);

impl TokenDigest {
    pub fn from_persistence(digest: String) -> Self {
        Self(digest)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for TokenDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TokenDigest(<redacted>)")
    }
}
