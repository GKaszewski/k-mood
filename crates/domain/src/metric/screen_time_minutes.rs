use crate::macros::bounded_metric;

const MIN_SCREEN_TIME_MINUTES: u16 = 0;
const MAX_SCREEN_TIME_MINUTES: u16 = 1_440;

bounded_metric!(
    ScreenTimeMinutes,
    u16,
    MIN_SCREEN_TIME_MINUTES,
    MAX_SCREEN_TIME_MINUTES,
    "screen time minutes"
);
