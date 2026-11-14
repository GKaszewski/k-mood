use crate::entry::DateSpan;

use super::{Granularity, TrendDay, TrendPoint};

pub trait TrendBucketing {
    fn bucket(
        &self,
        span: &DateSpan,
        days: &[TrendDay],
        granularity: Granularity,
    ) -> Vec<TrendPoint>;
}
