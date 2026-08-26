use crate::activity::ActivityId;
use crate::metric::MetricKind;

use super::{Family, SeriesShape};

const MOON_PHASE: &str = "moonPhase";
const ACTIVITY: &str = "activity";
const CYCLE_PROGRESS: &str = "cycleProgress";
const TEMPERATURE: &str = "temperature";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CorrelationInput {
    Metric(MetricKind),
    MoonPhase,
    CycleProgress,
    Temperature,
    Activity(ActivityId),
}

impl CorrelationInput {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Metric(kind) => kind.name(),
            Self::MoonPhase => MOON_PHASE,
            Self::CycleProgress => CYCLE_PROGRESS,
            Self::Temperature => TEMPERATURE,
            Self::Activity(_) => ACTIVITY,
        }
    }

    pub fn family(&self) -> Family {
        match self {
            Self::Metric(_) | Self::MoonPhase | Self::CycleProgress | Self::Temperature => {
                Family::Measurements
            }
            Self::Activity(_) => Family::Activities,
        }
    }

    pub fn series(&self) -> SeriesShape {
        match self {
            Self::Metric(_) | Self::MoonPhase | Self::CycleProgress | Self::Temperature => {
                SeriesShape::Continuous
            }
            Self::Activity(_) => SeriesShape::Presence,
        }
    }
}
