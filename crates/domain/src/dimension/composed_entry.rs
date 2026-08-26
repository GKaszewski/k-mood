use crate::activity::ActivityId;
use crate::attachment::{PhotoId, VoiceMemoId};
use crate::entry::{Content, MoodEntry};
use crate::location::Coordinates;
use crate::song::Song;
use crate::weather::Weather;

use super::DimensionValue;
use super::lookup;

pub struct ComposedEntry {
    pub entry: MoodEntry,
    pub dimensions: Vec<DimensionValue>,
}

impl ComposedEntry {
    pub fn content(&self) -> Option<&Content> {
        lookup::content_in(&self.dimensions)
    }

    pub fn activities(&self) -> &[ActivityId] {
        lookup::activities_in(&self.dimensions)
    }

    pub fn photos(&self) -> &[PhotoId] {
        lookup::photos_in(&self.dimensions)
    }

    pub fn voice_memos(&self) -> &[VoiceMemoId] {
        lookup::voice_memos_in(&self.dimensions)
    }

    pub fn location(&self) -> Option<&Coordinates> {
        lookup::location_in(&self.dimensions)
    }

    pub fn song(&self) -> Option<&Song> {
        lookup::song_in(&self.dimensions)
    }

    pub fn weather(&self) -> Option<&Weather> {
        lookup::weather_in(&self.dimensions)
    }
}
