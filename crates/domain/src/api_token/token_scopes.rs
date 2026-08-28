use std::collections::BTreeSet;

use crate::errors::DomainError;

use super::TokenScope;

const SEPARATOR: char = ' ';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenScopes(BTreeSet<TokenScope>);

impl TokenScopes {
    pub fn new(wanted: impl IntoIterator<Item = TokenScope>) -> Result<Self, DomainError> {
        let held: BTreeSet<TokenScope> = wanted.into_iter().collect();

        if held.is_empty() {
            return Err(DomainError::InvalidInput(
                "a token that grants nothing is not worth minting: name at least one scope".into(),
            ));
        }

        Ok(Self(held))
    }

    pub fn from_names<'a>(names: impl IntoIterator<Item = &'a str>) -> Result<Self, DomainError> {
        let mut wanted = BTreeSet::new();

        for name in names {
            let scope = TokenScope::from_name(name).ok_or_else(|| {
                DomainError::InvalidInput(format!("{name} is not a scope this server grants"))
            })?;

            wanted.insert(scope);
        }

        Self::new(wanted)
    }

    pub fn from_persistence(stored: &str) -> Option<Self> {
        let names: Vec<&str> = stored.split(SEPARATOR).filter(|s| !s.is_empty()).collect();

        Self::from_names(names).ok()
    }

    pub fn allows(&self, scope: TokenScope) -> bool {
        self.0.contains(&scope)
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.0.iter().map(TokenScope::name).collect()
    }

    pub fn to_persistence(&self) -> String {
        self.names().join(&SEPARATOR.to_string())
    }
}

impl std::fmt::Display for TokenScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_persistence())
    }
}
