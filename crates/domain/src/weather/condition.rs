const CLEAR: &str = "clear";
const CLOUDY: &str = "cloudy";
const FOG: &str = "fog";
const DRIZZLE: &str = "drizzle";
const RAIN: &str = "rain";
const SNOW: &str = "snow";
const THUNDERSTORM: &str = "thunderstorm";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Condition {
    Clear,
    Cloudy,
    Fog,
    Drizzle,
    Rain,
    Snow,
    Thunderstorm,
}

impl Condition {
    pub const ALL: [Condition; 7] = [
        Self::Clear,
        Self::Cloudy,
        Self::Fog,
        Self::Drizzle,
        Self::Rain,
        Self::Snow,
        Self::Thunderstorm,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Clear => CLEAR,
            Self::Cloudy => CLOUDY,
            Self::Fog => FOG,
            Self::Drizzle => DRIZZLE,
            Self::Rain => RAIN,
            Self::Snow => SNOW,
            Self::Thunderstorm => THUNDERSTORM,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            CLEAR => Some(Self::Clear),
            CLOUDY => Some(Self::Cloudy),
            FOG => Some(Self::Fog),
            DRIZZLE => Some(Self::Drizzle),
            RAIN => Some(Self::Rain),
            SNOW => Some(Self::Snow),
            THUNDERSTORM => Some(Self::Thunderstorm),
            _ => None,
        }
    }
}
