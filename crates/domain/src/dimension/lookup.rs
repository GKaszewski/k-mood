use crate::activity::ActivityId;
use crate::attachment::{PhotoId, VoiceMemoId};
use crate::entry::Content;
use crate::location::Coordinates;
use crate::song::Song;
use crate::weather::Weather;

use super::{DimensionKind, DimensionValue};

pub fn content_in(values: &[DimensionValue]) -> Option<&Content> {
    match find(values, DimensionKind::Content) {
        Some(DimensionValue::Content(content)) => Some(content),
        _ => None,
    }
}

pub fn activities_in(values: &[DimensionValue]) -> &[ActivityId] {
    match find(values, DimensionKind::Activities) {
        Some(DimensionValue::Activities(ids)) => ids,
        _ => &[],
    }
}

pub fn photos_in(values: &[DimensionValue]) -> &[PhotoId] {
    match find(values, DimensionKind::Photos) {
        Some(DimensionValue::Photos(ids)) => ids,
        _ => &[],
    }
}

pub fn voice_memos_in(values: &[DimensionValue]) -> &[VoiceMemoId] {
    match find(values, DimensionKind::VoiceMemos) {
        Some(DimensionValue::VoiceMemos(ids)) => ids,
        _ => &[],
    }
}

pub fn location_in(values: &[DimensionValue]) -> Option<&Coordinates> {
    match find(values, DimensionKind::Location) {
        Some(DimensionValue::Location(coordinates)) => Some(coordinates),
        _ => None,
    }
}

pub fn song_in(values: &[DimensionValue]) -> Option<&Song> {
    match find(values, DimensionKind::Song) {
        Some(DimensionValue::Song(song)) => Some(song),
        _ => None,
    }
}

pub fn weather_in(values: &[DimensionValue]) -> Option<&Weather> {
    match find(values, DimensionKind::Weather) {
        Some(DimensionValue::Weather(weather)) => Some(weather),
        _ => None,
    }
}

fn find(values: &[DimensionValue], kind: DimensionKind) -> Option<&DimensionValue> {
    values.iter().find(|value| value.kind() == kind)
}
