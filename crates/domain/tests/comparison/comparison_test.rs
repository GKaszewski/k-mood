use domain::activity::ActivityId;
use domain::comparison::{CycleQuarter, Quantity};
use domain::entry::{DayMood, Mood};
use domain::trend::{Granularity, TrendDay};

use super::comparison_fixtures::{
    RULES, alternating, compare, compare_with, october, period, row, september,
};
use super::period_fixtures::{activity_set, date, weather};

const PERIOD_LENGTH: usize = 30;
const MEH: u8 = 3;
const GOOD: u8 = 4;

#[test]
fn an_empty_period_yields_absent_values_not_nan() {
    let comparison = compare(
        september(&[]),
        october(&alternating(MEH, GOOD, PERIOD_LENGTH)),
    );

    for quantity in [
        Quantity::AverageMood,
        Quantity::MeanTemperature,
        Quantity::MoodInWeather(domain::weather::Condition::ALL[0]),
    ] {
        assert_eq!(row(&comparison, &quantity).a(), None);
        assert_eq!(row(&comparison, &quantity).relative_change(), None);
    }
    assert_eq!(row(&comparison, &Quantity::AverageMood).b(), Some(3.5));
    assert_eq!(row(&comparison, &Quantity::EntriesPerLoggedDay).a(), None);
    for row in comparison.rows() {
        let figures = [
            row.a(),
            row.b(),
            row.absolute_change(),
            row.relative_change(),
        ];
        assert!(figures.into_iter().flatten().all(f64::is_finite), "{row:?}");
    }
}

#[test]
fn relative_change_is_absent_when_a_was_zero() {
    let comparison = compare(
        september(&[MEH; PERIOD_LENGTH]),
        october(&[GOOD; PERIOD_LENGTH]),
    );

    let streak = row(&comparison, &Quantity::LongestGoodStreak);
    assert_eq!(streak.a(), Some(0.0));
    assert_eq!(streak.b(), Some(30.0));
    assert_eq!(streak.absolute_change(), Some(30.0));
    assert_eq!(streak.relative_change(), None);
}

#[test]
fn relative_change_is_measured_against_the_size_of_a_negative_start() {
    let mut a = september(&alternating(MEH, GOOD, PERIOD_LENGTH));
    for day in &mut a.days {
        day.weather = vec![weather(domain::weather::Condition::ALL[0], -2.0)];
    }
    let mut b = october(&alternating(MEH, GOOD, PERIOD_LENGTH));
    for day in &mut b.days {
        day.weather = vec![weather(domain::weather::Condition::ALL[0], 3.0)];
    }

    let comparison = compare(a, b);

    let temperature = row(&comparison, &Quantity::MeanTemperature);
    assert_eq!((temperature.a(), temperature.b()), (Some(-2.0), Some(3.0)));
    assert_eq!(temperature.absolute_change(), Some(5.0));
    assert_eq!(temperature.relative_change(), Some(2.5));
}

#[test]
fn an_activity_seen_only_in_b_still_gets_a_row_with_zero_share_in_a() {
    let running = ActivityId::generate();
    let mut b = october(&[MEH, GOOD, GOOD, MEH]);
    b.days[1].activities = activity_set(&running);

    let comparison = compare(september(&[MEH, MEH]), b);

    let share = row(&comparison, &Quantity::ActivityShare(running.clone()));
    assert_eq!(share.a(), Some(0.0));
    assert_eq!(share.b(), Some(0.25));
    assert_eq!(share.absolute_change(), Some(0.25));
    assert_eq!((share.sample_a(), share.sample_b()), (2, 4));
    let mood_with = row(&comparison, &Quantity::MoodWithActivity(running));
    assert_eq!(mood_with.a(), None);
    assert_eq!(mood_with.b(), Some(4.0));
}

fn cycle_rows(comparison: &domain::comparison::Comparison) -> usize {
    comparison
        .rows()
        .iter()
        .filter(|row| matches!(row.quantity(), Quantity::MoodInCycleQuarter(_)))
        .count()
}

#[test]
fn cycle_rows_appear_only_when_the_cycle_is_tracked() {
    let mut tracking = october(&[MEH]);
    tracking.tracks_cycle = true;
    let one_sided = compare(september(&[MEH]), tracking.clone());
    assert_eq!(cycle_rows(&one_sided), 0);

    let mut also_tracking = september(&[MEH]);
    also_tracking.tracks_cycle = true;
    let both = compare(also_tracking, tracking);
    assert_eq!(cycle_rows(&both), 4);
    assert_eq!(
        both.rows().last().unwrap().quantity(),
        &Quantity::MoodInCycleQuarter(CycleQuarter::Fourth)
    );
}

#[test]
fn both_trends_share_the_granularity_of_the_longer_span() {
    let a = september(&[]);
    let b = period("2026-06-01", "2026-08-29", &[]);
    let mood = DayMood::of(&[Mood::Good]).unwrap();
    let trend_a = [TrendDay::new(date("2026-09-02"), mood, 2)];

    let comparison = compare_with(a, b, &trend_a, &[], RULES);

    assert_eq!(comparison.granularity(), Granularity::Week);
    let (points_a, points_b) = comparison.trend();
    assert_eq!(points_a.len(), 5);
    assert_eq!(points_a[0].mean_mood(), Some(4.0));
    assert_eq!(points_a[0].entries(), 2);
    assert_eq!(points_a[4].days_covered(), 2);
    assert_eq!(points_b.len(), 13);
    assert_eq!(points_b[12].days_covered(), 6);
}
