const STEPS: &str = "steps";
const SLEEP_MINUTES: &str = "sleepMinutes";
const AWAKE_MINUTES: &str = "awakeMinutes";
const RESTING_HEART_RATE: &str = "restingHeartRate";
const HRV: &str = "hrv";
const EXERCISE_MINUTES: &str = "exerciseMinutes";
const SCREEN_TIME_MINUTES: &str = "screenTimeMinutes";
const ALCOHOLIC_DRINKS: &str = "alcoholicDrinks";

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
