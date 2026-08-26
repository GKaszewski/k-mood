use crate::macros::bounded_metric;

const MIN_STEPS_PER_DAY: u32 = 0;
const MAX_STEPS_PER_DAY: u32 = 200_000;

bounded_metric!(
    Steps,
    u32,
    MIN_STEPS_PER_DAY,
    MAX_STEPS_PER_DAY,
    "step count"
);
