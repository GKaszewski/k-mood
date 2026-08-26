use crate::activity::ActivityId;
use crate::attachment::{PhotoId, VoiceMemoId};
use crate::entry::Content;
use crate::location::Coordinates;
use crate::song::Song;
use crate::weather::Weather;

use super::DimensionKind;

#[derive(Debug, Clone, PartialEq)]
pub enum DimensionValue {
    Content(Content),
    Activities(Vec<ActivityId>),
    Photos(Vec<PhotoId>),
    VoiceMemos(Vec<VoiceMemoId>),
    Location(Coordinates),
    Song(Song),
    Weather(Weather),
}

impl DimensionValue {
    pub fn activities(mut ids: Vec<ActivityId>) -> Self {
        ids.sort();
        ids.dedup();
        Self::Activities(ids)
    }

    pub fn kind(&self) -> DimensionKind {
        match self {
            Self::Content(_) => DimensionKind::Content,
            Self::Activities(_) => DimensionKind::Activities,
            Self::Photos(_) => DimensionKind::Photos,
            Self::VoiceMemos(_) => DimensionKind::VoiceMemos,
            Self::Location(_) => DimensionKind::Location,
            Self::Song(_) => DimensionKind::Song,
            Self::Weather(_) => DimensionKind::Weather,
        }
    }
}
