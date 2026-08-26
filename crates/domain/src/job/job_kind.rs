const BACKFILL_RECORDING_IDENTITY: &str = "backfillRecordingIdentity";
const OBSERVE_WEATHER: &str = "observeWeather";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobKind {
    BackfillRecordingIdentity,
    ObserveWeather,
}

impl JobKind {
    pub const ALL: [JobKind; 2] = [Self::BackfillRecordingIdentity, Self::ObserveWeather];

    pub fn name(&self) -> &'static str {
        match self {
            Self::BackfillRecordingIdentity => BACKFILL_RECORDING_IDENTITY,
            Self::ObserveWeather => OBSERVE_WEATHER,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            BACKFILL_RECORDING_IDENTITY => Some(Self::BackfillRecordingIdentity),
            OBSERVE_WEATHER => Some(Self::ObserveWeather),
            _ => None,
        }
    }
}
