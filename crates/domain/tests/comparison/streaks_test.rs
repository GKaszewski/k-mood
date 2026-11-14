use domain::comparison::PeriodSummary;

use super::period_fixtures::{days_on, october_input};

#[test]
fn a_gap_breaks_a_logging_streak() {
    let days = days_on(&[(1, 3), (2, 3), (3, 3), (5, 3), (6, 3)]);

    let summary = PeriodSummary::of(&october_input(days));

    assert_eq!(summary.longest_logging_streak(), 3);
}

#[test]
fn a_good_streak_needs_good_or_better_days() {
    let days = days_on(&[(1, 4), (2, 5), (3, 3), (4, 4), (5, 4), (6, 4)]);

    let summary = PeriodSummary::of(&october_input(days));

    assert_eq!(summary.longest_good_streak(), 3);
}

#[test]
fn an_empty_period_has_no_streaks() {
    let summary = PeriodSummary::of(&october_input(Vec::new()));

    assert_eq!(summary.longest_logging_streak(), 0);
    assert_eq!(summary.longest_good_streak(), 0);
}
