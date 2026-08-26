use crate::macros::bounded_metric;

const MIN_EXERCISE_MINUTES: u16 = 0;
const MAX_EXERCISE_MINUTES: u16 = 1_440;

bounded_metric!(
    ExerciseMinutes,
    u16,
    MIN_EXERCISE_MINUTES,
    MAX_EXERCISE_MINUTES,
    "exercise minutes"
);
