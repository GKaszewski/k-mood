use chrono::NaiveDate;
use domain::entry::{Date, DateSpan, DayMood, Mood};
use domain::trend::{CalendarBucketing, Granularity, TrendBucketing, TrendDay, WeekStart};

fn date(year: i32, month: u32, day: u32) -> Date {
    Date::from_persistence(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

fn span(start: Date, end: Date) -> DateSpan {
    DateSpan::new(start, end).unwrap()
}

#[test]
fn calendar_weeks_start_on_the_requested_day() {
    let points = CalendarBucketing::new(WeekStart::Sunday).bucket(
        &span(date(2026, 10, 7), date(2026, 10, 20)),
        &[],
        Granularity::Week,
    );

    let starts: Vec<Date> = points.iter().map(|point| *point.start()).collect();
    let offsets: Vec<usize> = points.iter().map(|point| point.offset()).collect();
    assert_eq!(
        starts,
        vec![date(2026, 10, 4), date(2026, 10, 11), date(2026, 10, 18)]
    );
    assert_eq!(offsets, vec![0, 1, 2]);
}

#[test]
fn calendar_weeks_default_to_monday() {
    let points = CalendarBucketing::new(WeekStart::default()).bucket(
        &span(date(2026, 10, 7), date(2026, 10, 7)),
        &[],
        Granularity::Week,
    );

    assert_eq!(*points[0].start(), date(2026, 10, 5));
}

#[test]
fn calendar_edge_buckets_count_only_days_inside_the_span() {
    let points = CalendarBucketing::new(WeekStart::Sunday).bucket(
        &span(date(2026, 10, 7), date(2026, 10, 20)),
        &[],
        Granularity::Week,
    );

    let covered: Vec<usize> = points.iter().map(|point| point.days_covered()).collect();
    assert_eq!(covered, vec![4, 7, 3]);
}

#[test]
fn calendar_months_are_whole_months() {
    let points = CalendarBucketing::new(WeekStart::default()).bucket(
        &span(date(2025, 1, 15), date(2025, 3, 10)),
        &[],
        Granularity::Month,
    );

    let starts: Vec<Date> = points.iter().map(|point| *point.start()).collect();
    assert_eq!(
        starts,
        vec![date(2025, 1, 1), date(2025, 2, 1), date(2025, 3, 1)]
    );
    let covered: Vec<usize> = points.iter().map(|point| point.days_covered()).collect();
    assert_eq!(covered, vec![17, 28, 10]);
}

#[test]
fn calendar_months_roll_over_the_year_boundary() {
    let points = CalendarBucketing::new(WeekStart::default()).bucket(
        &span(date(2025, 12, 20), date(2026, 1, 5)),
        &[],
        Granularity::Month,
    );

    assert_eq!(points.len(), 2);
    assert_eq!(*points[1].start(), date(2026, 1, 1));
}

#[test]
fn calendar_days_are_single_day_buckets() {
    let points = CalendarBucketing::new(WeekStart::default()).bucket(
        &span(date(2026, 10, 1), date(2026, 10, 3)),
        &[],
        Granularity::Day,
    );

    assert_eq!(points.len(), 3);
    assert_eq!(*points[2].start(), date(2026, 10, 3));
}

#[test]
fn calendar_entries_are_summed_per_bucket() {
    let days = [
        TrendDay::new(date(2026, 10, 7), DayMood::of(&[Mood::Good]).unwrap(), 2),
        TrendDay::new(date(2026, 10, 8), DayMood::of(&[Mood::Good]).unwrap(), 3),
    ];

    let points = CalendarBucketing::new(WeekStart::Monday).bucket(
        &span(date(2026, 10, 5), date(2026, 10, 11)),
        &days,
        Granularity::Week,
    );

    assert_eq!(points.len(), 1);
    assert_eq!(points[0].entries(), 5);
    assert_eq!(points[0].logged_days(), 2);
}

#[test]
fn calendar_empty_buckets_in_the_middle_are_emitted_as_gaps() {
    let days = [TrendDay::new(
        date(2026, 10, 5),
        DayMood::of(&[Mood::Good]).unwrap(),
        1,
    )];

    let points = CalendarBucketing::new(WeekStart::Monday).bucket(
        &span(date(2026, 10, 5), date(2026, 10, 18)),
        &days,
        Granularity::Week,
    );

    assert_eq!(points[1].mean_mood(), None);
    assert_eq!(points[1].logged_days(), 0);
}
