pub(super) fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}
