use std::collections::BTreeMap;

use domain::activity::ActivityId;
use domain::comparison::{CycleQuarter, PeriodSummary};
use domain::cycle::{CycleDay, CyclePosition};
use domain::entry::Mood;
use domain::metric::MetricKind;

use super::period_fixtures::{
    activity_set, date, day_on, days_on, input_over, october_day, october_input,
};

const LOGGED_DAYS: u32 = 27;
const DAYS_IN_OCTOBER: f64 = 31.0;
const MEH: u8 = 3;
const HIGH_STEPS: f64 = 9000.0;
const LOW_STEPS: f64 = 4000.0;
const THIRD_QUARTER_PROGRESS: f64 = 0.6;

#[test]
fn coverage_counts_logged_days_against_the_days_in_the_span() {
    let days: Vec<_> = (1..=LOGGED_DAYS).map(|day| day_on(day, MEH)).collect();

    let summary = PeriodSummary::of(&october_input(days));

    let coverage = summary.coverage();
    assert_eq!(coverage.days_in_span, 31);
    assert_eq!(coverage.logged_days, 27);
    assert_eq!(coverage.entries, 27);
    assert_eq!(
        coverage.ratio(),
        Some(f64::from(LOGGED_DAYS) / DAYS_IN_OCTOBER)
    );
    assert_eq!(coverage.entries_per_logged_day(), Some(1.0));
}

#[test]
fn an_empty_period_has_no_ratio_of_entries_per_day() {
    let summary = PeriodSummary::of(&october_input(Vec::new()));

    assert_eq!(summary.coverage().entries_per_logged_day(), None);
    assert_eq!(summary.coverage().ratio(), Some(0.0));
}

#[test]
fn days_outside_the_span_are_ignored() {
    let mut days = days_on(&[(1, 3)]);
    days.push(domain_day_in_november());

    let summary = PeriodSummary::of(&october_input(days));

    assert_eq!(summary.coverage().logged_days, 1);
    assert_eq!(summary.day_moods(), vec![3.0]);
}

fn domain_day_in_november() -> domain::comparison::PeriodDay {
    let mut day = day_on(1, 5);
    day.date = date("2026-11-02");
    day
}

#[test]
fn mood_counts_include_moods_never_felt() {
    let summary = PeriodSummary::of(&october_input(days_on(&[(1, 3), (2, 3)])));

    let counts = summary.mood_counts();

    assert_eq!(counts.len(), Mood::ALL.len());
    assert_eq!(counts[&Mood::Awful], 0);
    assert_eq!(counts[&Mood::Meh], 2);
}

#[test]
fn mood_with_an_activity_only_counts_days_carrying_it() {
    let running = ActivityId::generate();
    let mut with_activity = day_on(1, 5);
    with_activity.activities = activity_set(&running);
    let days = vec![with_activity, day_on(2, 1)];

    let summary = PeriodSummary::of(&october_input(days));

    assert_eq!(summary.activity_days(&running), 1);
    assert_eq!(summary.moods_with_activity(&running), vec![5.0]);
    assert_eq!(summary.activities(), activity_set(&running));
}

#[test]
fn metrics_are_counted_on_days_with_data_even_without_entries() {
    let metrics = BTreeMap::from([(
        MetricKind::Steps,
        BTreeMap::from([
            (october_day(1), HIGH_STEPS),
            (october_day(2), LOW_STEPS),
            (date("2026-11-03"), HIGH_STEPS),
        ]),
    )]);
    let input = input_over("2026-10-01", "2026-10-31", Vec::new(), metrics);

    let summary = PeriodSummary::of(&input);

    assert_eq!(
        summary.metric_values(MetricKind::Steps),
        vec![HIGH_STEPS, LOW_STEPS]
    );
    assert!(summary.metric_values(MetricKind::Hrv).is_empty());
}

#[test]
fn cycle_quarters_collect_the_moods_of_days_in_them() {
    let mut cycling = day_on(1, 4);
    cycling.cycle = Some(CyclePosition::new(
        CycleDay::new(17).unwrap(),
        THIRD_QUARTER_PROGRESS,
    ));
    let mut input = october_input(vec![cycling, day_on(2, 2)]);
    input.tracks_cycle = true;

    let summary = PeriodSummary::of(&input);

    assert!(summary.tracks_cycle());
    assert_eq!(summary.moods_in_quarter(CycleQuarter::Third), vec![4.0]);
    assert!(summary.moods_in_quarter(CycleQuarter::First).is_empty());
}

#[test]
fn cycle_progress_of_point_six_is_the_third_quarter() {
    assert_eq!(
        CycleQuarter::of(THIRD_QUARTER_PROGRESS),
        CycleQuarter::Third
    );
}

#[test]
fn cycle_quarter_boundaries_belong_to_the_later_quarter() {
    assert_eq!(CycleQuarter::of(0.0), CycleQuarter::First);
    assert_eq!(CycleQuarter::of(0.25), CycleQuarter::Second);
    assert_eq!(CycleQuarter::of(0.5), CycleQuarter::Third);
    assert_eq!(CycleQuarter::of(0.75), CycleQuarter::Fourth);
    assert_eq!(CycleQuarter::of(1.0), CycleQuarter::Fourth);
}
