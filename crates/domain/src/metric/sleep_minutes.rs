use crate::macros::bounded_metric;

const MIN_SLEEP_MINUTES: u16 = 0;
const MAX_SLEEP_MINUTES: u16 = 1_440;

bounded_metric!(
    SleepMinutes,
    u16,
    MIN_SLEEP_MINUTES,
    MAX_SLEEP_MINUTES,
    "sleep minutes"
);
