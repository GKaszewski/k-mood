use crate::errors::DomainError;

use super::{Latitude, Longitude};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coordinates {
    latitude: Latitude,
    longitude: Longitude,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, DomainError> {
        Ok(Self {
            latitude: Latitude::new(latitude)?,
            longitude: Longitude::new(longitude)?,
        })
    }

    pub fn from_persistence(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: Latitude::from_persistence(latitude),
            longitude: Longitude::from_persistence(longitude),
        }
    }

    pub fn latitude(&self) -> Latitude {
        self.latitude
    }

    pub fn longitude(&self) -> Longitude {
        self.longitude
    }
}
