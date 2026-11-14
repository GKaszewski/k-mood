use chrono::NaiveDate;
use domain::entry::{Date, DateSpan, DayMood, Mood};
use domain::trend::{Granularity, SpanStartBucketing, TrendBucketing, TrendDay};

fn date(year: i32, month: u32, day: u32) -> Date {
    Date::from_persistence(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

fn span(start: Date, end: Date) -> DateSpan {
    DateSpan::new(start, end).unwrap()
}

fn logged(date: Date, mood: Mood, entries: usize) -> TrendDay {
    TrendDay::new(date, DayMood::of(&[mood]).unwrap(), entries)
}

#[test]
fn span_start_weeks_count_from_the_first_day_whatever_weekday_it_is() {
    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 7), date(2026, 10, 20)),
        &[],
        Granularity::Week,
    );

    assert_eq!(points.len(), 2);
    assert_eq!(*points[0].start(), date(2026, 10, 7));
    assert_eq!(*points[1].start(), date(2026, 10, 14));
    assert_eq!(points[0].days_covered(), 7);
    assert_eq!(points[1].days_covered(), 7);
}

#[test]
fn span_start_last_window_may_be_partial() {
    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 1), date(2026, 10, 10)),
        &[],
        Granularity::Week,
    );

    assert_eq!(points.len(), 2);
    assert_eq!(points[1].days_covered(), 3);
}

#[test]
fn span_start_offsets_are_contiguous_from_zero() {
    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 1), date(2026, 10, 3)),
        &[],
        Granularity::Day,
    );

    let offsets: Vec<usize> = points.iter().map(|point| point.offset()).collect();
    assert_eq!(offsets, vec![0, 1, 2]);
}

#[test]
fn span_start_months_are_thirty_day_windows() {
    let points = SpanStartBucketing.bucket(
        &span(date(2026, 1, 1), date(2026, 3, 15)),
        &[],
        Granularity::Month,
    );

    assert_eq!(points.len(), 3);
    assert_eq!(*points[1].start(), date(2026, 1, 31));
    assert_eq!(points[2].days_covered(), 14);
}

#[test]
fn a_bucket_with_no_logged_days_is_a_gap_not_a_zero() {
    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 1), date(2026, 10, 7)),
        &[],
        Granularity::Week,
    );

    assert_eq!(points[0].mean_mood(), None);
    assert_eq!(points[0].logged_days(), 0);
    assert_eq!(points[0].entries(), 0);
}

#[test]
fn the_mean_is_taken_over_logged_days_only() {
    let days = [
        logged(date(2026, 10, 1), Mood::Rad, 1),
        logged(date(2026, 10, 3), Mood::Bad, 1),
    ];

    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 1), date(2026, 10, 7)),
        &days,
        Granularity::Week,
    );

    let expected = (f64::from(Mood::Rad.value()) + f64::from(Mood::Bad.value())) / 2.0;
    assert_eq!(points[0].mean_mood(), Some(expected));
    assert_eq!(points[0].logged_days(), 2);
}

#[test]
fn entries_are_summed_per_bucket() {
    let days = [
        logged(date(2026, 10, 1), Mood::Good, 2),
        logged(date(2026, 10, 2), Mood::Good, 3),
    ];

    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 1), date(2026, 10, 7)),
        &days,
        Granularity::Week,
    );

    assert_eq!(points[0].entries(), 5);
}

#[test]
fn days_outside_the_span_are_ignored() {
    let days = [logged(date(2026, 9, 30), Mood::Good, 4)];

    let points = SpanStartBucketing.bucket(
        &span(date(2026, 10, 1), date(2026, 10, 7)),
        &days,
        Granularity::Week,
    );

    assert_eq!(points[0].logged_days(), 0);
    assert_eq!(points[0].entries(), 0);
}
