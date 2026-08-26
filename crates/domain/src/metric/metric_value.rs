use crate::errors::DomainError;

use super::{
    AlcoholicDrinks, AwakeMinutes, ExerciseMinutes, Hrv, MetricKind, RestingHeartRate,
    ScreenTimeMinutes, SleepMinutes, Steps,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricValue {
    Steps(Steps),
    SleepMinutes(SleepMinutes),
    AwakeMinutes(AwakeMinutes),
    RestingHeartRate(RestingHeartRate),
    Hrv(Hrv),
    ExerciseMinutes(ExerciseMinutes),
    ScreenTimeMinutes(ScreenTimeMinutes),
    AlcoholicDrinks(AlcoholicDrinks),
}

impl MetricValue {
    pub fn of_kind(kind: MetricKind, count: i64) -> Result<Self, DomainError> {
        match kind {
            MetricKind::Steps => Ok(Self::Steps(Steps::new(narrow(count, kind)?)?)),
            MetricKind::SleepMinutes => {
                Ok(Self::SleepMinutes(SleepMinutes::new(narrow(count, kind)?)?))
            }
            MetricKind::AwakeMinutes => {
                Ok(Self::AwakeMinutes(AwakeMinutes::new(narrow(count, kind)?)?))
            }
            MetricKind::RestingHeartRate => Ok(Self::RestingHeartRate(RestingHeartRate::new(
                narrow(count, kind)?,
            )?)),
            MetricKind::Hrv => Ok(Self::Hrv(Hrv::new(narrow(count, kind)?)?)),
            MetricKind::ExerciseMinutes => Ok(Self::ExerciseMinutes(ExerciseMinutes::new(
                narrow(count, kind)?,
            )?)),
            MetricKind::ScreenTimeMinutes => Ok(Self::ScreenTimeMinutes(ScreenTimeMinutes::new(
                narrow(count, kind)?,
            )?)),
            MetricKind::AlcoholicDrinks => Ok(Self::AlcoholicDrinks(AlcoholicDrinks::new(
                narrow(count, kind)?,
            )?)),
        }
    }
}

impl MetricValue {
    pub fn kind(&self) -> MetricKind {
        match self {
            Self::Steps(_) => MetricKind::Steps,
            Self::SleepMinutes(_) => MetricKind::SleepMinutes,
            Self::AwakeMinutes(_) => MetricKind::AwakeMinutes,
            Self::RestingHeartRate(_) => MetricKind::RestingHeartRate,
            Self::Hrv(_) => MetricKind::Hrv,
            Self::ExerciseMinutes(_) => MetricKind::ExerciseMinutes,
            Self::ScreenTimeMinutes(_) => MetricKind::ScreenTimeMinutes,
            Self::AlcoholicDrinks(_) => MetricKind::AlcoholicDrinks,
        }
    }

    pub fn count(&self) -> i64 {
        match self {
            Self::Steps(steps) => i64::from(steps.value()),
            Self::SleepMinutes(minutes) => i64::from(minutes.value()),
            Self::AwakeMinutes(minutes) => i64::from(minutes.value()),
            Self::RestingHeartRate(beats) => i64::from(beats.value()),
            Self::Hrv(milliseconds) => i64::from(milliseconds.value()),
            Self::ExerciseMinutes(minutes) => i64::from(minutes.value()),
            Self::ScreenTimeMinutes(minutes) => i64::from(minutes.value()),
            Self::AlcoholicDrinks(drinks) => i64::from(drinks.value()),
        }
    }
}

fn narrow<T: TryFrom<i64>>(count: i64, kind: MetricKind) -> Result<T, DomainError> {
    T::try_from(count).map_err(|_| {
        DomainError::InvalidInput(format!("{} cannot hold the value {count}", kind.name()))
    })
}
