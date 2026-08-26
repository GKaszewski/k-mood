use domain::dimension::DimensionValue;
use domain::entry::Content;
use domain::location::Coordinates;
use domain::provider::ProviderName;
use domain::song::{RecordingId, Song};
use domain::weather::{Celsius, Condition, Weather};

use crate::errors::ApiValidationError;

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum DimensionPayload {
    Content {
        text: String,
    },
    Activities {
        ids: Vec<String>,
    },
    Photos {
        ids: Vec<String>,
    },
    VoiceMemos {
        ids: Vec<String>,
    },
    Location {
        latitude: f64,
        longitude: f64,
    },
    Song {
        title: String,
        artist: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        album: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        recording_id: Option<String>,
    },
    Weather {
        condition: String,
        temperature: f64,
        observed_by: String,
    },
}

impl From<&DimensionValue> for DimensionPayload {
    fn from(value: &DimensionValue) -> Self {
        match value {
            DimensionValue::Content(content) => Self::Content {
                text: content.value().to_string(),
            },
            DimensionValue::Activities(ids) => Self::Activities {
                ids: ids.iter().map(|id| id.value().to_string()).collect(),
            },
            DimensionValue::Photos(ids) => Self::Photos {
                ids: ids.iter().map(|id| id.value().to_string()).collect(),
            },
            DimensionValue::VoiceMemos(ids) => Self::VoiceMemos {
                ids: ids.iter().map(|id| id.value().to_string()).collect(),
            },
            DimensionValue::Location(coordinates) => Self::Location {
                latitude: coordinates.latitude().value(),
                longitude: coordinates.longitude().value(),
            },
            DimensionValue::Weather(weather) => Self::Weather {
                condition: weather.condition().name().to_string(),
                temperature: weather.temperature().value(),
                observed_by: weather.observed_by().value().to_string(),
            },
            DimensionValue::Song(song) => Self::Song {
                title: song.title().value().to_string(),
                artist: song.artist().value().to_string(),
                album: song.album().map(|album| album.value().to_string()),
                recording_id: song.recording_id().map(|id| id.value().to_string()),
            },
        }
    }
}

impl DimensionPayload {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Content { text } => text.trim().is_empty(),
            Self::Activities { ids } | Self::Photos { ids } | Self::VoiceMemos { ids } => {
                ids.is_empty()
            }
            Self::Location { .. } | Self::Song { .. } | Self::Weather { .. } => false,
        }
    }

    pub fn into_dimension(self) -> Result<DimensionValue, ApiValidationError> {
        match self {
            Self::Content { text } => Ok(DimensionValue::Content(Content::new(text)?)),
            Self::Activities { ids } => Ok(DimensionValue::activities(all_ids(&ids)?)),
            Self::Photos { ids } => Ok(DimensionValue::Photos(all_ids(&ids)?)),
            Self::VoiceMemos { ids } => Ok(DimensionValue::VoiceMemos(all_ids(&ids)?)),
            Self::Location {
                latitude,
                longitude,
            } => Ok(DimensionValue::Location(Coordinates::new(
                latitude, longitude,
            )?)),
            Self::Weather {
                condition,
                temperature,
                observed_by,
            } => {
                let condition = Condition::from_name(&condition).ok_or_else(|| {
                    ApiValidationError::Invalid(format!("unknown weather condition: {condition}"))
                })?;

                Ok(DimensionValue::Weather(Weather::new(
                    condition,
                    Celsius::new(temperature)?,
                    ProviderName::new(observed_by)?,
                )))
            }
            Self::Song {
                title,
                artist,
                album,
                recording_id,
            } => {
                let recording_id = recording_id.as_deref().map(RecordingId::new).transpose()?;

                Ok(DimensionValue::Song(Song::new(
                    title,
                    artist,
                    album,
                    recording_id,
                )?))
            }
        }
    }
}

fn all_ids<T: From<uuid::Uuid>>(ids: &[String]) -> Result<Vec<T>, ApiValidationError> {
    crate::mappers::shared::parse_uuids_as(ids, ids.len(), "identifiers")
}
