use domain::comparison::Measure;
use domain::statistics::{
    PValue, two_proportion, welch_satterthwaite_degrees_of_freedom, welch_two_sample,
};

#[test]
fn a_figure_carries_its_values_and_is_never_scored() {
    let figure = Measure::Figure {
        a: Some(3.0),
        b: None,
    };

    assert_eq!(figure.values(), (Some(3.0), None));
    assert_eq!(figure.p_value(), None);
    assert!(figure.expected_counts_suffice());
}

#[test]
fn a_mean_averages_each_side_and_counts_its_samples() {
    let mean = Measure::Mean {
        a: vec![2.0, 4.0],
        b: vec![5.0, 5.0, 2.0],
    };

    assert_eq!(mean.values(), (Some(3.0), Some(4.0)));
    assert_eq!(mean.samples(), (2, 3));
    let first = [2.0, 4.0];
    let second = [5.0, 5.0, 2.0];
    assert_eq!(
        mean.p_value(),
        Some(PValue::from_t_statistic(
            welch_two_sample(&first, &second).unwrap(),
            welch_satterthwaite_degrees_of_freedom(&first, &second).unwrap(),
        ))
    );
    assert!(mean.expected_counts_suffice());
}

#[test]
fn a_mean_with_an_empty_side_has_no_value_for_it() {
    let mean = Measure::Mean {
        a: Vec::new(),
        b: vec![4.0],
    };

    assert_eq!(mean.values(), (None, Some(4.0)));
    assert_eq!(mean.samples(), (0, 1));
    assert_eq!(mean.p_value(), None);
}

#[test]
fn a_share_is_a_fraction_of_its_total() {
    let share = Measure::Share {
        hits_a: 3,
        total_a: 12,
        hits_b: 9,
        total_b: 18,
    };

    assert_eq!(share.values(), (Some(0.25), Some(0.5)));
    assert_eq!(share.samples(), (12, 18));
    assert_eq!(
        share.p_value(),
        two_proportion(3, 12, 9, 18).map(PValue::from_standard_score)
    );
}

#[test]
fn a_share_of_nothing_has_no_value() {
    let share = Measure::Share {
        hits_a: 0,
        total_a: 0,
        hits_b: 1,
        total_b: 2,
    };

    assert_eq!(share.values(), (None, Some(0.5)));
    assert_eq!(share.p_value(), None);
}

#[test]
fn a_share_with_five_expected_in_every_cell_can_be_tested() {
    let share = Measure::Share {
        hits_a: 3,
        total_a: 20,
        hits_b: 7,
        total_b: 20,
    };

    assert!(share.expected_counts_suffice());
}

#[test]
fn a_share_with_fewer_than_five_expected_in_any_cell_cannot_be_tested() {
    let rare = Measure::Share {
        hits_a: 0,
        total_a: 10,
        hits_b: 4,
        total_b: 10,
    };
    let nearly_universal = Measure::Share {
        hits_a: 10,
        total_a: 10,
        hits_b: 6,
        total_b: 10,
    };

    assert!(!rare.expected_counts_suffice());
    assert!(!nearly_universal.expected_counts_suffice());
}
