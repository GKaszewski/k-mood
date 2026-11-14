use crate::entry::DateSpan;

use super::bucket_window::{BucketWindow, days_after};
use super::{Granularity, TrendBucketing, TrendDay, TrendPoint};

const DAY_WINDOW: u64 = 1;
const WEEK_WINDOW: u64 = 7;
const MONTH_WINDOW: u64 = 30;
const LAST_DAY_OFFSET: u64 = 1;

pub struct SpanStartBucketing;

impl SpanStartBucketing {
    fn window_length(granularity: Granularity) -> u64 {
        match granularity {
            Granularity::Day => DAY_WINDOW,
            Granularity::Week => WEEK_WINDOW,
            Granularity::Month => MONTH_WINDOW,
        }
    }
}

impl TrendBucketing for SpanStartBucketing {
    fn bucket(
        &self,
        span: &DateSpan,
        days: &[TrendDay],
        granularity: Granularity,
    ) -> Vec<TrendPoint> {
        let window_length = Self::window_length(granularity);
        let mut points = Vec::new();
        let mut window_start = Some(*span.start());

        while let Some(start) = window_start.filter(|start| start <= span.end()) {
            let window_end = days_after(start, window_length - LAST_DAY_OFFSET);
            let window = window_end.and_then(|end| BucketWindow::clipped_to(start, end, span));
            if let Some(window) = window {
                points.push(window.into_point(points.len(), days));
            }
            window_start = days_after(start, window_length);
        }

        points
    }
}
