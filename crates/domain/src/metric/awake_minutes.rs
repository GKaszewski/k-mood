use crate::macros::bounded_metric;

const MIN_AWAKE_MINUTES: u16 = 0;
const MAX_AWAKE_MINUTES: u16 = 1_440;

bounded_metric!(
    AwakeMinutes,
    u16,
    MIN_AWAKE_MINUTES,
    MAX_AWAKE_MINUTES,
    "awake minutes"
);
