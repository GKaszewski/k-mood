use domain::correlation::{CorrelationStrategy, Observation};
use domain::entry::{DayMood, Mood};

fn day(mood: Mood) -> DayMood {
    DayMood::of(&[mood]).unwrap()
}

fn observations(pairs: &[(f64, Mood)]) -> Vec<Observation> {
    pairs
        .iter()
        .map(|(value, mood)| Observation::new(*value, day(*mood)))
        .collect()
}

fn spearman(pairs: &[(f64, Mood)]) -> Option<f64> {
    CorrelationStrategy::Spearman
        .score(&observations(pairs))
        .map(|coefficient| coefficient.value())
}

#[test]
fn a_perfectly_ordered_pair_of_series_scores_one() {
    let score = spearman(&[
        (1.0, Mood::Awful),
        (2.0, Mood::Bad),
        (3.0, Mood::Meh),
        (4.0, Mood::Good),
    ])
    .unwrap();

    assert!((score - 1.0).abs() < 1e-12);
}

#[test]
fn a_perfectly_inverted_pair_of_series_scores_minus_one() {
    let score = spearman(&[
        (1.0, Mood::Rad),
        (2.0, Mood::Good),
        (3.0, Mood::Meh),
        (4.0, Mood::Bad),
    ])
    .unwrap();

    assert!((score + 1.0).abs() < 1e-12);
}

#[test]
fn only_the_order_matters_not_the_distance() {
    let gentle = spearman(&[
        (1.0, Mood::Awful),
        (2.0, Mood::Bad),
        (3.0, Mood::Meh),
        (4.0, Mood::Good),
    ])
    .unwrap();

    let one_enormous_day = spearman(&[
        (1.0, Mood::Awful),
        (2.0, Mood::Bad),
        (3.0, Mood::Meh),
        (100_000.0, Mood::Good),
    ])
    .unwrap();

    assert!((gentle - one_enormous_day).abs() < f64::EPSILON);
}

#[test]
fn tied_moods_share_their_average_rank_rather_than_taking_the_shortcut() {
    let score = spearman(&[
        (1_000.0, Mood::Meh),
        (2_000.0, Mood::Meh),
        (3_000.0, Mood::Meh),
        (4_000.0, Mood::Good),
        (5_000.0, Mood::Good),
        (6_000.0, Mood::Rad),
    ])
    .unwrap();

    let tie_corrected = 0.925_820_099_8;
    let textbook_shortcut = 0.928_571_428_6;

    assert!(
        (score - tie_corrected).abs() < 1e-9,
        "expected the tie-corrected coefficient, got {score}"
    );
    assert!(
        (score - textbook_shortcut).abs() > 1e-9,
        "this is the 6*d^2 shortcut, which is wrong when ranks are tied"
    );
}

#[test]
fn a_day_count_below_two_scores_nothing() {
    assert!(spearman(&[(1.0, Mood::Good)]).is_none());
    assert!(spearman(&[]).is_none());
}

#[test]
fn an_unvarying_series_scores_nothing() {
    let same_mood_every_day = spearman(&[
        (1.0, Mood::Meh),
        (2.0, Mood::Meh),
        (3.0, Mood::Meh),
        (4.0, Mood::Meh),
    ]);

    let same_value_every_day = spearman(&[
        (5_000.0, Mood::Awful),
        (5_000.0, Mood::Bad),
        (5_000.0, Mood::Good),
        (5_000.0, Mood::Rad),
    ]);

    assert!(same_mood_every_day.is_none());
    assert!(same_value_every_day.is_none());
}
