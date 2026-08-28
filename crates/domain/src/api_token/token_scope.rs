const READ_JOURNAL: &str = "readJournal";
const WRITE_JOURNAL: &str = "writeJournal";
const WRITE_METRICS: &str = "writeMetrics";
const READ_PROFILE: &str = "readProfile";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenScope {
    ReadJournal,
    WriteJournal,
    WriteMetrics,
    ReadProfile,
}

impl TokenScope {
    pub const ALL: [TokenScope; 4] = [
        Self::ReadJournal,
        Self::WriteJournal,
        Self::WriteMetrics,
        Self::ReadProfile,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::ReadJournal => READ_JOURNAL,
            Self::WriteJournal => WRITE_JOURNAL,
            Self::WriteMetrics => WRITE_METRICS,
            Self::ReadProfile => READ_PROFILE,
        }
    }

    pub fn describes(&self) -> &'static str {
        match self {
            Self::ReadJournal => {
                "read entries, activities, the calendar, statistics, correlations, cycle records and daily metrics"
            }
            Self::WriteJournal => {
                "create, change and delete entries, activities, cycle records, reminders and media"
            }
            Self::WriteMetrics => "write daily metrics, attributed to this token's name",
            Self::ReadProfile => "read the account profile, its preferences and its connections",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            READ_JOURNAL => Some(Self::ReadJournal),
            WRITE_JOURNAL => Some(Self::WriteJournal),
            WRITE_METRICS => Some(Self::WriteMetrics),
            READ_PROFILE => Some(Self::ReadProfile),
            _ => None,
        }
    }
}

impl std::fmt::Display for TokenScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
