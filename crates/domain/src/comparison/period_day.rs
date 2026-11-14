use std::collections::BTreeSet;

use crate::activity::ActivityId;
use crate::cycle::CyclePosition;
use crate::entry::{Date, DayMood};
use crate::song::Song;
use crate::weather::Weather;

#[derive(Debug, Clone, PartialEq)]
pub struct PeriodDay {
    pub date: Date,
    pub mood: DayMood,
    pub entries: usize,
    pub activities: BTreeSet<ActivityId>,
    pub weather: Vec<Weather>,
    pub songs: Vec<Song>,
    pub cycle: Option<CyclePosition>,
}
