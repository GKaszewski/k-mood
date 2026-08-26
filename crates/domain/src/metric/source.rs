use crate::provider::ProviderName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Manual,
    Provider(ProviderName),
}

impl Source {
    pub fn is_manual(&self) -> bool {
        matches!(self, Self::Manual)
    }

    pub fn is_superseded_by(&self, incoming: &Source) -> bool {
        incoming.is_manual() || !self.is_manual()
    }

    pub fn provider(&self) -> Option<&ProviderName> {
        match self {
            Self::Manual => None,
            Self::Provider(name) => Some(name),
        }
    }
}
