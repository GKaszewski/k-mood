const STEPS: &str = "steps";
const SLEEP_MINUTES: &str = "sleepMinutes";
const AWAKE_MINUTES: &str = "awakeMinutes";
const RESTING_HEART_RATE: &str = "restingHeartRate";
const HRV: &str = "hrv";
const EXERCISE_MINUTES: &str = "exerciseMinutes";
const SCREEN_TIME_MINUTES: &str = "screenTimeMinutes";
const ALCOHOLIC_DRINKS: &str = "alcoholicDrinks";

const STEPS_UNIT: &str = "steps";
const MINUTES_UNIT: &str = "minutes";
const BEATS_PER_MINUTE_UNIT: &str = "bpm";
const MILLISECONDS_UNIT: &str = "milliseconds";
const DRINKS_UNIT: &str = "drinks";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetricKind {
    Steps,
    SleepMinutes,
    AwakeMinutes,
    RestingHeartRate,
    Hrv,
    ExerciseMinutes,
    ScreenTimeMinutes,
    AlcoholicDrinks,
}

impl MetricKind {
    pub const ALL: [MetricKind; 8] = [
        Self::Steps,
        Self::SleepMinutes,
        Self::AwakeMinutes,
        Self::RestingHeartRate,
        Self::Hrv,
        Self::ExerciseMinutes,
        Self::ScreenTimeMinutes,
        Self::AlcoholicDrinks,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Steps => STEPS,
            Self::SleepMinutes => SLEEP_MINUTES,
            Self::AwakeMinutes => AWAKE_MINUTES,
            Self::RestingHeartRate => RESTING_HEART_RATE,
            Self::Hrv => HRV,
            Self::ExerciseMinutes => EXERCISE_MINUTES,
            Self::ScreenTimeMinutes => SCREEN_TIME_MINUTES,
            Self::AlcoholicDrinks => ALCOHOLIC_DRINKS,
        }
    }

    pub fn unit(&self) -> &'static str {
        match self {
            Self::Steps => STEPS_UNIT,
            Self::SleepMinutes | Self::AwakeMinutes => MINUTES_UNIT,
            Self::RestingHeartRate => BEATS_PER_MINUTE_UNIT,
            Self::Hrv => MILLISECONDS_UNIT,
            Self::ExerciseMinutes | Self::ScreenTimeMinutes => MINUTES_UNIT,
            Self::AlcoholicDrinks => DRINKS_UNIT,
        }
    }

    pub fn bounds(&self) -> (i64, i64) {
        use crate::metric::{
            AlcoholicDrinks, AwakeMinutes, ExerciseMinutes, Hrv, RestingHeartRate,
            ScreenTimeMinutes, SleepMinutes, Steps,
        };

        match self {
            Self::Steps => (Steps::MINIMUM.into(), Steps::MAXIMUM.into()),
            Self::SleepMinutes => (SleepMinutes::MINIMUM.into(), SleepMinutes::MAXIMUM.into()),
            Self::AwakeMinutes => (AwakeMinutes::MINIMUM.into(), AwakeMinutes::MAXIMUM.into()),
            Self::RestingHeartRate => (
                RestingHeartRate::MINIMUM.into(),
                RestingHeartRate::MAXIMUM.into(),
            ),
            Self::Hrv => (Hrv::MINIMUM.into(), Hrv::MAXIMUM.into()),
            Self::ExerciseMinutes => (
                ExerciseMinutes::MINIMUM.into(),
                ExerciseMinutes::MAXIMUM.into(),
            ),
            Self::ScreenTimeMinutes => (
                ScreenTimeMinutes::MINIMUM.into(),
                ScreenTimeMinutes::MAXIMUM.into(),
            ),
            Self::AlcoholicDrinks => (
                AlcoholicDrinks::MINIMUM.into(),
                AlcoholicDrinks::MAXIMUM.into(),
            ),
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            STEPS => Some(Self::Steps),
            SLEEP_MINUTES => Some(Self::SleepMinutes),
            AWAKE_MINUTES => Some(Self::AwakeMinutes),
            RESTING_HEART_RATE => Some(Self::RestingHeartRate),
            HRV => Some(Self::Hrv),
            EXERCISE_MINUTES => Some(Self::ExerciseMinutes),
            SCREEN_TIME_MINUTES => Some(Self::ScreenTimeMinutes),
            ALCOHOLIC_DRINKS => Some(Self::AlcoholicDrinks),
            _ => None,
        }
    }
}
