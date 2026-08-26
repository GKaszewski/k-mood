const IMPORT: &str = "import";
const STORED_ROW: &str = "storedRow";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RejectionOrigin {
    Import,
    StoredRow,
}

impl RejectionOrigin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Import => IMPORT,
            Self::StoredRow => STORED_ROW,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            IMPORT => Some(Self::Import),
            STORED_ROW => Some(Self::StoredRow),
            _ => None,
        }
    }
}
