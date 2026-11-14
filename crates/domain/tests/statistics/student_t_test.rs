use domain::statistics::{PValue, student_t_two_sided};

const TOLERANCE: f64 = 1e-3;

#[test]
fn a_t_of_two_point_three_on_eighteen_degrees_of_freedom() {
    assert!((student_t_two_sided(2.3, 18.0) - 0.0336).abs() < TOLERANCE);
}

#[test]
fn fewer_degrees_of_freedom_make_the_same_t_less_surprising() {
    assert!((student_t_two_sided(2.3, 9.0) - 0.0470).abs() < TOLERANCE);
}

#[test]
fn a_t_of_zero_is_no_evidence_at_all() {
    assert!((student_t_two_sided(0.0, 18.0) - 1.0).abs() < 1e-9);
}

#[test]
fn the_tail_is_symmetric() {
    assert!((student_t_two_sided(-2.3, 18.0) - student_t_two_sided(2.3, 18.0)).abs() < 1e-12);
}

#[test]
fn many_degrees_of_freedom_converge_to_the_normal_tail() {
    assert!((student_t_two_sided(1.96, 10_000.0) - 0.05).abs() < TOLERANCE);
}

#[test]
fn an_infinite_t_is_certain_evidence() {
    assert_eq!(student_t_two_sided(f64::INFINITY, 18.0), 0.0);
}

#[test]
fn a_p_value_from_a_t_statistic_uses_the_student_tail() {
    assert!((PValue::from_t_statistic(2.3, 18.0).value() - 0.0336).abs() < TOLERANCE);
}
