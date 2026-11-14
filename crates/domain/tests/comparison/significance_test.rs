use std::collections::BTreeMap;

use domain::comparison::{Quantity, Significance};
use domain::entry::{Date, Mood};
use domain::metric::MetricKind;
use domain::statistics::{
    FalseDiscoveryControl, PValue, welch_satterthwaite_degrees_of_freedom, welch_two_sample,
};

use super::comparison_fixtures::{
    FALSE_DISCOVERY_RATE, alternating, compare, october, row, september,
};
use super::period_fixtures::date;

const PERIOD_LENGTH: usize = 30;
const TOO_FEW_DAYS: usize = 5;
const MINIMUM_PERIOD_LENGTH: usize = 10;
const AWFUL: u8 = 1;
const MEH: u8 = 3;
const GOOD: u8 = 4;
const STEPS_LOW: f64 = 4000.0;
const STEPS_HIGH: f64 = 6000.0;
const STEPS_SMALL_RISE: f64 = 540.0;

#[test]
fn descriptive_rows_are_not_tested() {
    let comparison = compare(
        september(&alternating(2, 3, PERIOD_LENGTH)),
        october(&alternating(4, 5, PERIOD_LENGTH)),
    );

    assert_eq!(
        row(&comparison, &Quantity::Coverage).significance(),
        Significance::NotTested
    );
    assert_eq!(
        row(&comparison, &Quantity::LongestGoodStreak).significance(),
        Significance::NotTested
    );
}

#[test]
fn overlapping_spans_test_nothing() {
    let comparison = compare(
        october(&alternating(2, 3, PERIOD_LENGTH)),
        october(&alternating(2, 3, PERIOD_LENGTH)),
    );

    assert!(comparison.overlap());
    assert!(
        comparison
            .rows()
            .iter()
            .all(|row| row.significance() == Significance::NotTested)
    );
}

#[test]
fn a_side_below_the_minimum_has_not_enough_data() {
    let comparison = compare(
        september(&alternating(2, 3, TOO_FEW_DAYS)),
        october(&alternating(4, 5, PERIOD_LENGTH)),
    );

    let average = row(&comparison, &Quantity::AverageMood);
    assert_eq!(average.significance(), Significance::NotEnoughData);
    assert_eq!(average.a(), Some(2.4));
    assert_eq!(average.b(), Some(4.5));
    assert_eq!((average.sample_a(), average.sample_b()), (5, 30));
}

#[test]
fn a_clear_rise_in_mood_is_likely_real() {
    let comparison = compare(
        september(&alternating(2, 3, PERIOD_LENGTH)),
        october(&alternating(4, 5, PERIOD_LENGTH)),
    );

    let average = row(&comparison, &Quantity::AverageMood);
    assert_eq!(average.significance(), Significance::LikelyReal);
    assert_eq!(average.absolute_change(), Some(2.0));
    assert_eq!(average.relative_change(), Some(0.8));
}

#[test]
fn a_trivial_wobble_is_noise() {
    let mut wobbled = alternating(3, 4, PERIOD_LENGTH);
    wobbled[0] = 4;

    let comparison = compare(
        september(&alternating(3, 4, PERIOD_LENGTH)),
        october(&wobbled),
    );

    assert_eq!(
        row(&comparison, &Quantity::AverageMood).significance(),
        Significance::Noise
    );
}

fn steps_over(start: &str, rise: f64) -> BTreeMap<Date, f64> {
    let mut current = date(start);
    let mut steps = BTreeMap::new();
    for index in 0..PERIOD_LENGTH {
        let base = if index % 2 == 0 {
            STEPS_LOW
        } else {
            STEPS_HIGH
        };
        steps.insert(current, base + rise);
        current = current.next();
    }
    steps
}

#[test]
fn the_correction_runs_across_every_tested_row() {
    let steps_a = steps_over("2026-09-01", 0.0);
    let steps_b = steps_over("2026-10-01", STEPS_SMALL_RISE);
    let values_a = steps_a.values().copied().collect::<Vec<_>>();
    let values_b = steps_b.values().copied().collect::<Vec<_>>();
    let alone = PValue::from_t_statistic(
        welch_two_sample(&values_a, &values_b).unwrap(),
        welch_satterthwaite_degrees_of_freedom(&values_a, &values_b).unwrap(),
    );
    assert!(alone.value() < FALSE_DISCOVERY_RATE);
    assert_eq!(
        FalseDiscoveryControl::at(FALSE_DISCOVERY_RATE).holds_up(&[alone]),
        vec![true]
    );

    let same_moods = alternating(3, 4, PERIOD_LENGTH);
    let mut a = september(&same_moods);
    a.metrics = BTreeMap::from([(MetricKind::Steps, steps_a)]);
    let mut b = october(&same_moods);
    b.metrics = BTreeMap::from([(MetricKind::Steps, steps_b)]);

    let comparison = compare(a, b);

    let tested = comparison
        .rows()
        .iter()
        .filter(|row| {
            matches!(
                row.significance(),
                Significance::LikelyReal | Significance::Noise
            )
        })
        .count();
    assert_eq!(tested, 4);
    assert_eq!(
        row(&comparison, &Quantity::MetricMean(MetricKind::Steps)).significance(),
        Significance::Noise
    );
}

#[test]
fn a_mood_felt_on_no_day_of_either_period_is_not_tested() {
    let comparison = compare(
        september(&alternating(MEH, GOOD, PERIOD_LENGTH)),
        october(&alternating(MEH, GOOD, PERIOD_LENGTH)),
    );

    let awful = row(&comparison, &Quantity::MoodShare(Mood::Awful));
    assert_eq!((awful.sample_a(), awful.sample_b()), (30, 30));
    assert_eq!((awful.a(), awful.b()), (Some(0.0), Some(0.0)));
    assert_eq!(awful.significance(), Significance::NotTested);
}

#[test]
fn a_share_with_too_few_expected_occurrences_has_not_enough_data() {
    let mut with_awful_days = alternating(MEH, GOOD, MINIMUM_PERIOD_LENGTH);
    with_awful_days[..4].fill(AWFUL);

    let comparison = compare(
        september(&alternating(MEH, GOOD, MINIMUM_PERIOD_LENGTH)),
        october(&with_awful_days),
    );

    let awful = row(&comparison, &Quantity::MoodShare(Mood::Awful));
    assert_eq!((awful.sample_a(), awful.sample_b()), (10, 10));
    assert_eq!((awful.a(), awful.b()), (Some(0.0), Some(0.4)));
    assert_eq!(awful.significance(), Significance::NotEnoughData);
}
