const PENDING: &str = "pending";
const RUNNING: &str = "running";
const EXHAUSTED: &str = "exhausted";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobStatus {
    Pending,
    Running,
    Exhausted,
}

impl JobStatus {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Pending => PENDING,
            Self::Running => RUNNING,
            Self::Exhausted => EXHAUSTED,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            PENDING => Some(Self::Pending),
            RUNNING => Some(Self::Running),
            EXHAUSTED => Some(Self::Exhausted),
            _ => None,
        }
    }
}
