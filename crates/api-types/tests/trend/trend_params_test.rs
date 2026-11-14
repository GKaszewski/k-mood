use api_types::errors::ApiValidationError;
use api_types::params::TrendParams;
use domain::trend::WeekStart;

fn params(week_starts_on: Option<&str>) -> TrendParams {
    TrendParams {
        from: "2026-09-01".into(),
        to: "2026-09-30".into(),
        week_starts_on: week_starts_on.map(Into::into),
    }
}

#[test]
fn an_omitted_week_start_is_monday() {
    let (_, week_start) = params(None).into_query_parts().unwrap();

    assert_eq!(week_start, WeekStart::Monday);
}

#[test]
fn a_named_week_start_is_honoured() {
    let (_, week_start) = params(Some("sunday")).into_query_parts().unwrap();

    assert_eq!(week_start, WeekStart::Sunday);
}

#[test]
fn an_unknown_week_start_is_refused() {
    let refused = params(Some("someday")).into_query_parts();

    assert!(matches!(refused, Err(ApiValidationError::Invalid(_))));
}
