use domain::activity::ActivityId;
use domain::correlation::{CorrelationInput, CorrelationStrategy, Observation, SeriesShape};
use domain::entry::{DayMood, Mood};
use domain::metric::MetricKind;

fn day(mood: Mood) -> DayMood {
    DayMood::of(&[mood]).unwrap()
}

fn observations(pairs: &[(f64, Mood)]) -> Vec<Observation> {
    pairs
        .iter()
        .map(|(value, mood)| Observation::new(*value, day(*mood)))
        .collect()
}

fn score(strategy: CorrelationStrategy, pairs: &[(f64, Mood)]) -> Option<f64> {
    strategy
        .score(&observations(pairs))
        .map(|coefficient| coefficient.value())
}

const TIED: [(f64, Mood); 6] = [
    (1_000.0, Mood::Meh),
    (2_000.0, Mood::Meh),
    (3_000.0, Mood::Meh),
    (4_000.0, Mood::Good),
    (5_000.0, Mood::Good),
    (6_000.0, Mood::Rad),
];

#[test]
fn kendall_corrects_for_ties_rather_than_counting_pairs_alone() {
    let score = score(CorrelationStrategy::Kendall, &TIED).unwrap();

    let tau_b = 0.856_348_838_6;
    let tau_a = 0.733_333_333_3;

    assert!((score - tau_b).abs() < 1e-9, "expected tau-b, got {score}");
    assert!(
        (score - tau_a).abs() > 1e-9,
        "this is tau-a, which overstates disagreement when ranks are tied"
    );
}

#[test]
fn kendall_reaches_both_extremes() {
    let rising = score(
        CorrelationStrategy::Kendall,
        &[
            (1.0, Mood::Awful),
            (2.0, Mood::Bad),
            (3.0, Mood::Meh),
            (4.0, Mood::Good),
        ],
    )
    .unwrap();

    let falling = score(
        CorrelationStrategy::Kendall,
        &[
            (1.0, Mood::Good),
            (2.0, Mood::Meh),
            (3.0, Mood::Bad),
            (4.0, Mood::Awful),
        ],
    )
    .unwrap();

    assert!((rising - 1.0).abs() < 1e-12);
    assert!((falling + 1.0).abs() < 1e-12);
}

#[test]
fn pearson_measures_the_line_where_the_rank_methods_measure_the_order() {
    let curved = [
        (1.0, Mood::Awful),
        (2.0, Mood::Bad),
        (3.0, Mood::Meh),
        (100.0, Mood::Good),
    ];

    let pearson = score(CorrelationStrategy::Pearson, &curved).unwrap();
    let spearman = score(CorrelationStrategy::Spearman, &curved).unwrap();

    assert!((spearman - 1.0).abs() < 1e-12, "the order is perfect");
    assert!(
        (pearson - 0.785_026_421).abs() < 1e-9,
        "the line is not, got {pearson}"
    );
}

#[test]
fn a_mean_difference_is_told_as_a_share_of_the_mood_scale() {
    let score = score(
        CorrelationStrategy::MeanDifference,
        &[
            (1.0, Mood::Good),
            (1.0, Mood::Rad),
            (1.0, Mood::Good),
            (1.0, Mood::Rad),
            (0.0, Mood::Bad),
            (0.0, Mood::Meh),
            (0.0, Mood::Bad),
            (0.0, Mood::Awful),
        ],
    )
    .unwrap();

    assert!(
        (score - 0.625).abs() < 1e-12,
        "two and a half mood points out of four, got {score}"
    );
}

#[test]
fn a_mean_difference_needs_days_on_both_sides() {
    let only_present = score(
        CorrelationStrategy::MeanDifference,
        &[(1.0, Mood::Good), (1.0, Mood::Rad)],
    );

    let only_absent = score(
        CorrelationStrategy::MeanDifference,
        &[(0.0, Mood::Good), (0.0, Mood::Rad)],
    );

    assert!(only_present.is_none());
    assert!(only_absent.is_none());
}

#[test]
fn a_strategy_scores_one_shape_of_series_and_an_input_has_one() {
    assert_eq!(
        CorrelationStrategy::MeanDifference.scores(),
        SeriesShape::Presence
    );
    assert_eq!(
        CorrelationInput::Activity(ActivityId::generate()).series(),
        SeriesShape::Presence
    );

    for continuous in [
        CorrelationStrategy::Pearson,
        CorrelationStrategy::Spearman,
        CorrelationStrategy::Kendall,
    ] {
        assert_eq!(continuous.scores(), SeriesShape::Continuous);
    }

    assert_eq!(
        CorrelationInput::Metric(MetricKind::Steps).series(),
        SeriesShape::Continuous
    );
    assert_eq!(
        CorrelationInput::MoonPhase.series(),
        SeriesShape::Continuous
    );
}

#[test]
fn only_matching_strategies_are_offered_for_an_input() {
    let steps = CorrelationInput::Metric(MetricKind::Steps);
    let exercise = CorrelationInput::Activity(ActivityId::generate());

    let for_steps: Vec<&str> = CorrelationStrategy::ALL
        .into_iter()
        .filter(|strategy| strategy.can_score(&steps))
        .map(|strategy| strategy.name())
        .collect();

    let for_exercise: Vec<&str> = CorrelationStrategy::ALL
        .into_iter()
        .filter(|strategy| strategy.can_score(&exercise))
        .map(|strategy| strategy.name())
        .collect();

    assert_eq!(for_steps, ["pearson", "spearman", "kendall"]);
    assert_eq!(for_exercise, ["meanDifference"]);
}
