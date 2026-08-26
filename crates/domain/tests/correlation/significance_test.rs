use domain::correlation::{CorrelationStrategy, Observation, PValue};
use domain::entry::{DayMood, Mood};

fn day(mood: Mood) -> DayMood {
    DayMood::of(&[mood]).unwrap()
}

fn mood_of(value: u8) -> Mood {
    Mood::try_from(value).unwrap()
}

fn loosely_rising() -> Vec<Observation> {
    let moods = [3, 1, 4, 2, 3, 5, 2, 4, 3, 5, 4, 3];

    moods
        .iter()
        .enumerate()
        .map(|(index, mood)| Observation::new(index as f64 + 1.0, day(mood_of(*mood))))
        .collect()
}

fn significance(strategy: CorrelationStrategy, observations: &[Observation]) -> f64 {
    strategy
        .significance(observations)
        .expect("a scored series has a p-value")
        .value()
}

#[test]
fn the_familiar_thresholds_come_out_of_the_normal_tail() {
    assert!((PValue::from_standard_score(1.96).value() - 0.05).abs() < 1e-4);
    assert!((PValue::from_standard_score(1.645).value() - 0.10).abs() < 1e-4);
}

#[test]
fn a_standard_score_of_nothing_is_certain_to_be_nothing() {
    assert!((PValue::from_standard_score(0.0).value() - 1.0).abs() < 1e-9);
}

#[test]
fn direction_does_not_change_how_surprising_a_score_is() {
    let positive = PValue::from_standard_score(2.5).value();
    let negative = PValue::from_standard_score(-2.5).value();

    assert!((positive - negative).abs() < 1e-12);
}

#[test]
fn each_strategy_turns_its_own_statistic_into_a_p_value() {
    let observations = loosely_rising();

    let pearson = significance(CorrelationStrategy::Pearson, &observations);
    let spearman = significance(CorrelationStrategy::Spearman, &observations);
    let kendall = significance(CorrelationStrategy::Kendall, &observations);

    assert!((pearson - 0.198_050_956_8).abs() < 1e-4, "got {pearson}");
    assert!((spearman - 0.241_732_162_6).abs() < 1e-4, "got {spearman}");
    assert!((kendall - 0.201_603_352_7).abs() < 1e-4, "got {kendall}");
}

#[test]
fn a_relationship_that_holds_all_the_way_is_hard_to_put_down_to_chance() {
    let tight: Vec<Observation> = (1..=12)
        .map(|day_number| {
            let mood = mood_of(((day_number - 1) / 3 + 1) as u8);
            Observation::new(day_number as f64, day(mood))
        })
        .collect();

    assert!(significance(CorrelationStrategy::Pearson, &tight) < 0.001);
    assert!(significance(CorrelationStrategy::Spearman, &tight) < 0.001);
    assert!(significance(CorrelationStrategy::Kendall, &tight) < 0.001);
}

#[test]
fn a_mean_difference_is_judged_by_the_spread_within_each_group() {
    let observations: Vec<Observation> = [
        (1.0, Mood::Good),
        (1.0, Mood::Rad),
        (1.0, Mood::Good),
        (1.0, Mood::Rad),
        (0.0, Mood::Bad),
        (0.0, Mood::Meh),
        (0.0, Mood::Bad),
        (0.0, Mood::Awful),
    ]
    .into_iter()
    .map(|(value, mood)| Observation::new(value, day(mood)))
    .collect();

    let p = significance(CorrelationStrategy::MeanDifference, &observations);

    assert!((p - 0.000_000_573_3).abs() < 1e-9, "got {p}");
}

#[test]
fn a_series_that_cannot_be_scored_has_nothing_to_report() {
    let unvarying: Vec<Observation> = (1..=10)
        .map(|day_number| Observation::new(day_number as f64, day(Mood::Meh)))
        .collect();

    assert!(
        CorrelationStrategy::Spearman
            .significance(&unvarying)
            .is_none()
    );
}
