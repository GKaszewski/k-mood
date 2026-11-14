use chrono::NaiveDate;

use domain::entry::{Date, DateSpan};

fn date(text: &str) -> Date {
    Date::from_persistence(NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap())
}

fn span(start: &str, end: &str) -> DateSpan {
    DateSpan::new(date(start), date(end)).unwrap()
}

#[test]
fn a_span_of_one_day_has_one_day() {
    assert_eq!(span("2026-10-01", "2026-10-01").days(), 1);
}

#[test]
fn october_has_thirty_one_days() {
    assert_eq!(span("2026-10-01", "2026-10-31").days(), 31);
}

#[test]
fn a_running_year_is_clipped_to_today() {
    let clipped = span("2026-01-01", "2026-12-31")
        .clipped_to(date("2026-11-14"))
        .unwrap();

    assert_eq!(clipped.span().days(), 318);
    assert!(clipped.was_clipped());
}

#[test]
fn a_finished_span_is_not_clipped() {
    let clipped = span("2025-10-01", "2025-10-31")
        .clipped_to(date("2026-11-14"))
        .unwrap();

    assert!(!clipped.was_clipped());
}

#[test]
fn a_span_entirely_in_the_future_has_nothing_to_compare() {
    assert!(
        span("2027-01-01", "2027-01-31")
            .clipped_to(date("2026-11-14"))
            .is_none()
    );
}

#[test]
fn touching_spans_overlap_on_the_shared_day() {
    assert!(span("2026-01-01", "2026-01-10").overlaps(&span("2026-01-10", "2026-01-20")));
}
