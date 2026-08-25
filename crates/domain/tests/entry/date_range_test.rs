use chrono::{FixedOffset, TimeZone};

use domain::entry::DateRange;
use domain::errors::DomainError;

fn utc_dt(year: i32, month: u32, day: u32) -> chrono::DateTime<FixedOffset> {
    FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(year, month, day, 0, 0, 0)
        .unwrap()
}

#[test]
fn valid_range_is_created() {
    let start = utc_dt(2025, 1, 1);
    let end = utc_dt(2025, 1, 31);
    let range = DateRange::new(start, end).unwrap();

    assert_eq!(*range.start(), start);
    assert_eq!(*range.end(), end);
}

#[test]
fn start_equal_to_end_is_valid() {
    let dt = utc_dt(2025, 6, 15);
    let range = DateRange::new(dt, dt);
    assert!(range.is_ok());
}

#[test]
fn start_after_end_is_rejected() {
    let start = utc_dt(2025, 3, 15);
    let end = utc_dt(2025, 3, 1);
    let result = DateRange::new(start, end);
    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}
