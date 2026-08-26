use domain::metric::{MetricKind, MetricValue, Steps};

#[test]
fn a_values_kind_is_derived_from_the_value_itself() {
    let value = MetricValue::Steps(Steps::new(8_412).unwrap());

    assert_eq!(value.kind(), MetricKind::Steps);
}
