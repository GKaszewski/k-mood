use api_types::errors::ApiValidationError;
use api_types::params::ComparedSpansParams;

fn params(a_from: &str, a_to: &str, b_from: &str, b_to: &str) -> ComparedSpansParams {
    ComparedSpansParams {
        a_from: a_from.into(),
        a_to: a_to.into(),
        b_from: b_from.into(),
        b_to: b_to.into(),
    }
}

#[test]
fn well_formed_params_become_two_spans() {
    let (a, b) = params("2026-09-01", "2026-09-30", "2026-10-01", "2026-10-31")
        .into_spans()
        .unwrap();

    assert_eq!(a.start().to_string(), "2026-09-01");
    assert_eq!(b.end().to_string(), "2026-10-31");
}

#[test]
fn a_reversed_span_is_refused() {
    let refused = params("2026-10-31", "2026-10-01", "2026-09-01", "2026-09-30").into_spans();

    assert!(matches!(refused, Err(ApiValidationError::Domain(_))));
}

#[test]
fn an_unparseable_date_is_refused() {
    let refused = params("2026-09-01", "soon", "2026-10-01", "2026-10-31").into_spans();

    assert!(matches!(refused, Err(ApiValidationError::Invalid(_))));
}
