use domain::comparison::{
    ComparedSpans, Comparison, ComparisonRow, ComparisonRules, MaximumSpan, PeriodDay, PeriodInput,
    PeriodSummary, Quantity,
};
use std::collections::BTreeMap;

use domain::trend::TrendDay;

use super::period_fixtures::{date, day_on, input_over};

pub const MINIMUM_SAMPLE_SIZE: usize = 10;
pub const FALSE_DISCOVERY_RATE: f64 = 0.10;
pub const RULES: ComparisonRules = ComparisonRules {
    minimum_sample_size: MINIMUM_SAMPLE_SIZE,
    false_discovery_rate: FALSE_DISCOVERY_RATE,
};
const LONGEST_COMPARED_SPAN: i64 = 366;

pub fn consecutive_days(start: &str, moods: &[u8]) -> Vec<PeriodDay> {
    let mut current = date(start);
    moods
        .iter()
        .map(|mood| {
            let mut day = day_on(1, *mood);
            day.date = current;
            current = current.next();
            day
        })
        .collect()
}

pub fn alternating(first: u8, second: u8, count: usize) -> Vec<u8> {
    (0..count)
        .map(|index| if index % 2 == 0 { first } else { second })
        .collect()
}

pub fn compare(a: PeriodInput, b: PeriodInput) -> Comparison {
    compare_with(a, b, &[], &[], RULES)
}

pub fn compare_with(
    a: PeriodInput,
    b: PeriodInput,
    trend_a: &[TrendDay],
    trend_b: &[TrendDay],
    rules: ComparisonRules,
) -> Comparison {
    let spans = ComparedSpans::new(
        *a.span.span(),
        *b.span.span(),
        MaximumSpan::of_days(LONGEST_COMPARED_SPAN),
    )
    .unwrap();
    Comparison::between(
        PeriodSummary::of(&a),
        PeriodSummary::of(&b),
        &spans,
        trend_a,
        trend_b,
        rules,
    )
}

pub fn row<'a>(comparison: &'a Comparison, quantity: &Quantity) -> &'a ComparisonRow {
    comparison
        .rows()
        .iter()
        .find(|row| row.quantity() == quantity)
        .unwrap()
}

pub fn period(start: &str, end: &str, moods: &[u8]) -> PeriodInput {
    input_over(start, end, consecutive_days(start, moods), BTreeMap::new())
}

pub fn september(moods: &[u8]) -> PeriodInput {
    period("2026-09-01", "2026-09-30", moods)
}

pub fn october(moods: &[u8]) -> PeriodInput {
    period("2026-10-01", "2026-10-31", moods)
}
