const WRITE_METRICS: &str = "writeMetrics";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenScope {
    WriteMetrics,
}

impl TokenScope {
    pub fn name(&self) -> &'static str {
        match self {
            Self::WriteMetrics => WRITE_METRICS,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            WRITE_METRICS => Some(Self::WriteMetrics),
            _ => None,
        }
    }
}
