use crate::macros::bounded_metric;

const MIN_HRV: u16 = 1;
const MAX_HRV: u16 = 300;

bounded_metric!(Hrv, u16, MIN_HRV, MAX_HRV, "heart rate variability");
