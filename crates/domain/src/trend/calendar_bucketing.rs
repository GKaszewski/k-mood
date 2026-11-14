use chrono::{Datelike, Days, Months, NaiveDate};

use crate::entry::{Date, DateSpan};

use super::bucket_window::BucketWindow;
use super::{Granularity, TrendBucketing, TrendDay, TrendPoint, WeekStart};

const DAYS_IN_WEEK: u32 = 7;
const FIRST_OF_MONTH: u32 = 1;
const ONE_DAY: Days = Days::new(1);
const ONE_WEEK: Days = Days::new(DAYS_IN_WEEK as u64);
const ONE_MONTH: Months = Months::new(1);

pub struct CalendarBucketing {
    week_start: WeekStart,
}

impl CalendarBucketing {
    pub fn new(week_start: WeekStart) -> Self {
        Self { week_start }
    }
}

impl CalendarBucketing {
    fn bucket_start(&self, day: NaiveDate, granularity: Granularity) -> Option<NaiveDate> {
        match granularity {
            Granularity::Day => Some(day),
            Granularity::Week => {
                day.checked_sub_days(Days::new(u64::from(self.days_into_week(day))))
            }
            Granularity::Month => day.with_day(FIRST_OF_MONTH),
        }
    }

    fn days_into_week(&self, day: NaiveDate) -> u32 {
        let weekday_index = day.weekday().num_days_from_monday();
        let week_start_index = self.week_start.weekday().num_days_from_monday();
        (weekday_index + DAYS_IN_WEEK - week_start_index) % DAYS_IN_WEEK
    }

    fn next_bucket_start(start: NaiveDate, granularity: Granularity) -> Option<NaiveDate> {
        match granularity {
            Granularity::Day => start.checked_add_days(ONE_DAY),
            Granularity::Week => start.checked_add_days(ONE_WEEK),
            Granularity::Month => start.checked_add_months(ONE_MONTH),
        }
    }
}

impl TrendBucketing for CalendarBucketing {
    fn bucket(
        &self,
        span: &DateSpan,
        days: &[TrendDay],
        granularity: Granularity,
    ) -> Vec<TrendPoint> {
        let mut points = Vec::new();
        let mut current = self.bucket_start(span.start().value(), granularity);

        while let Some(start) = current.filter(|start| *start <= span.end().value()) {
            let next = Self::next_bucket_start(start, granularity);
            let last_day = next.and_then(|next| next.pred_opt());
            let window = last_day.and_then(|last| {
                BucketWindow::clipped_to(
                    Date::from_persistence(start),
                    Date::from_persistence(last),
                    span,
                )
            });
            if let Some(window) = window {
                points.push(window.into_point(points.len(), days));
            }
            current = next;
        }

        points
    }
}
