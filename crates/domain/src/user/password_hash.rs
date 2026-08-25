#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
}

impl PasswordHash {
    pub fn value(&self) -> &str {
        &self.0
    }
}
