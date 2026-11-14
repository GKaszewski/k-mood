use chrono::NaiveDate;

use domain::comparison::{ComparedSpans, MaximumSpan};
use domain::entry::{Date, DateSpan};
use domain::errors::DomainError;

const MAXIMUM_DAYS: i64 = 1830;

fn date(text: &str) -> Date {
    Date::from_persistence(NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap())
}

fn span(start: &str, end: &str) -> DateSpan {
    DateSpan::new(date(start), date(end)).unwrap()
}

#[test]
fn compared_spans_refuse_a_span_longer_than_the_maximum() {
    let result = ComparedSpans::new(
        span("2020-01-01", "2026-01-01"),
        span("2026-01-01", "2026-01-31"),
        MaximumSpan::of_days(MAXIMUM_DAYS),
    );

    assert!(matches!(result, Err(DomainError::InvalidInput(_))));
}

#[test]
fn compared_spans_allow_overlap_and_say_so() {
    let spans = ComparedSpans::new(
        span("2026-01-01", "2026-06-30"),
        span("2026-06-01", "2026-08-31"),
        MaximumSpan::of_days(MAXIMUM_DAYS),
    )
    .unwrap();

    assert!(spans.overlap());
}
