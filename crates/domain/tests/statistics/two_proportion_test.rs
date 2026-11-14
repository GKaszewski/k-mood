use domain::statistics::two_proportion;

#[test]
fn two_proportion_matches_a_textbook_value() {
    assert!((two_proportion(18, 100, 31, 100).unwrap() - 2.1373).abs() < 1e-3);
}

#[test]
fn two_proportion_refuses_an_empty_side() {
    assert_eq!(two_proportion(0, 0, 3, 10), None);
}

#[test]
fn two_proportion_refuses_when_everyone_or_no_one_did_it() {
    assert_eq!(two_proportion(10, 10, 5, 5), None);
}
