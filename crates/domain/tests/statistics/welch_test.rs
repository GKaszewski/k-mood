use domain::statistics::{welch_satterthwaite_degrees_of_freedom, welch_two_sample};

#[test]
fn welch_scores_a_higher_second_sample_as_positive() {
    let z = welch_two_sample(&[1.0, 2.0, 3.0, 4.0], &[3.0, 4.0, 5.0, 6.0]).unwrap();
    assert!((z - 2.1909).abs() < 1e-3);
}

#[test]
fn welch_needs_two_values_on_each_side() {
    assert_eq!(welch_two_sample(&[1.0], &[2.0, 3.0]), None);
}

#[test]
fn welch_has_nothing_to_say_about_two_constant_samples() {
    assert_eq!(welch_two_sample(&[3.0, 3.0], &[3.0, 3.0]), None);
}

#[test]
fn two_equally_spread_samples_of_ten_have_eighteen_degrees_of_freedom() {
    let first = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let second = [11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];

    let degrees_of_freedom = welch_satterthwaite_degrees_of_freedom(&first, &second).unwrap();

    assert!((degrees_of_freedom - 18.0).abs() < 1e-9);
}

#[test]
fn unequal_spreads_lose_degrees_of_freedom() {
    let narrow = [5.0, 5.5, 4.5, 5.0, 5.5, 4.5, 5.0, 5.5, 4.5, 5.0];
    let wide = [1.0, 9.0, 2.0, 8.0, 3.0, 7.0, 1.0, 9.0, 2.0, 8.0];

    let degrees_of_freedom = welch_satterthwaite_degrees_of_freedom(&narrow, &wide).unwrap();

    assert!(degrees_of_freedom < 18.0);
    assert!(degrees_of_freedom > 9.0 - 1e-9);
}

#[test]
fn degrees_of_freedom_need_some_spread() {
    assert_eq!(
        welch_satterthwaite_degrees_of_freedom(&[3.0, 3.0], &[3.0, 3.0]),
        None
    );
}
