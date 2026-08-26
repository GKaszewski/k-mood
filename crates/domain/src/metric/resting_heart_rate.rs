use crate::macros::bounded_metric;

const MIN_RESTING_HEART_RATE: u16 = 25;
const MAX_RESTING_HEART_RATE: u16 = 120;

bounded_metric!(
    RestingHeartRate,
    u16,
    MIN_RESTING_HEART_RATE,
    MAX_RESTING_HEART_RATE,
    "resting heart rate"
);
