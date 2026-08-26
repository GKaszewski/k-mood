use crate::entry::Date;
use crate::provider::ProviderName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectionDetail {
    provider: Option<ProviderName>,
    date: Option<Date>,
    kind: String,
    value: Option<i64>,
}

impl RejectionDetail {
    pub fn new(
        provider: Option<ProviderName>,
        date: Option<Date>,
        kind: impl Into<String>,
        value: Option<i64>,
    ) -> Self {
        Self {
            provider,
            date,
            kind: kind.into(),
            value,
        }
    }
}

impl RejectionDetail {
    pub fn provider(&self) -> Option<&ProviderName> {
        self.provider.as_ref()
    }

    pub fn date(&self) -> Option<&Date> {
        self.date.as_ref()
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn value(&self) -> Option<i64> {
        self.value
    }
}
