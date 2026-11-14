use std::collections::BTreeMap;

use crate::entry::{ClippedSpan, Date};
use crate::metric::MetricKind;

use super::PeriodDay;

#[derive(Debug, Clone, PartialEq)]
pub struct PeriodInput {
    pub span: ClippedSpan,
    pub days: Vec<PeriodDay>,
    pub metrics: BTreeMap<MetricKind, BTreeMap<Date, f64>>,
    pub tracks_cycle: bool,
    pub top_music_count: usize,
}
