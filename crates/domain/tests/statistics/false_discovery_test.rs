use domain::statistics::{FalseDiscoveryControl, PValue};

fn p_values(values: &[f64]) -> Vec<PValue> {
    values
        .iter()
        .map(|value| PValue::new(*value).unwrap())
        .collect()
}

#[test]
fn the_step_up_procedure_keeps_everything_below_the_largest_passing_rank() {
    let adjustment = FalseDiscoveryControl::at(0.10);

    let held = adjustment.holds_up(&p_values(&[0.001, 0.008, 0.039, 0.041, 0.42]));

    assert_eq!(held, [true, true, true, true, false]);
}

#[test]
fn a_result_below_its_own_threshold_is_carried_by_a_stronger_one() {
    let adjustment = FalseDiscoveryControl::at(0.10);

    let held = adjustment.holds_up(&p_values(&[0.001, 0.079]));

    assert_eq!(
        held,
        [true, true],
        "0.079 exceeds rank 1's threshold but rank 2 passes, so both hold"
    );
}

#[test]
fn order_of_the_set_does_not_change_who_holds_up() {
    let adjustment = FalseDiscoveryControl::at(0.10);

    let held = adjustment.holds_up(&p_values(&[0.42, 0.041, 0.001, 0.039, 0.008]));

    assert_eq!(held, [false, true, true, true, true]);
}

#[test]
fn nothing_holds_up_when_nothing_is_small_enough() {
    let adjustment = FalseDiscoveryControl::at(0.10);

    let held = adjustment.holds_up(&p_values(&[0.4, 0.5, 0.6]));

    assert_eq!(held, [false, false, false]);
}

#[test]
fn a_larger_set_makes_each_result_work_harder() {
    let adjustment = FalseDiscoveryControl::at(0.10);

    let alone = adjustment.holds_up(&p_values(&[0.04]));
    let among_ten = adjustment.holds_up(&p_values(&[
        0.04, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    ]));

    assert_eq!(alone, [true]);
    assert!(!among_ten[0], "0.04 needs to beat 0.01 once ten are tested");
}

#[test]
fn the_threshold_is_whatever_it_is_configured_to_be() {
    let strict = FalseDiscoveryControl::at(0.01);
    let loose = FalseDiscoveryControl::at(0.20);

    let borderline = p_values(&[0.03, 0.5]);

    assert_eq!(strict.holds_up(&borderline), [false, false]);
    assert_eq!(loose.holds_up(&borderline), [true, false]);
}

#[test]
fn an_empty_set_holds_nothing_up_and_does_not_panic() {
    let adjustment = FalseDiscoveryControl::at(0.10);

    assert!(adjustment.holds_up(&[]).is_empty());
}
