use chrono::{DateTime, FixedOffset};

use crate::errors::DomainError;
use crate::location::Coordinates;
use crate::provider::ProviderName;
use crate::weather::Weather;

pub struct FakeWeatherLookup {
    provider: ProviderName,
    observed: Option<Weather>,
    failing_with: Option<String>,
    asked: std::sync::atomic::AtomicUsize,
}

impl FakeWeatherLookup {
    pub fn observing(observed: Option<Weather>) -> Self {
        Self {
            provider: ProviderName::from_persistence("open-meteo".into()),
            observed,
            failing_with: None,
            asked: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn failing(reason: impl Into<String>) -> Self {
        Self {
            provider: ProviderName::from_persistence("open-meteo".into()),
            observed: None,
            failing_with: Some(reason.into()),
            asked: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn times_asked(&self) -> usize {
        self.asked.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait::async_trait]
impl crate::ports::WeatherLookupPort for FakeWeatherLookup {
    fn provider(&self) -> &ProviderName {
        &self.provider
    }

    async fn observed_at(
        &self,
        _coordinates: &Coordinates,
        _instant: &DateTime<FixedOffset>,
    ) -> Result<Option<Weather>, DomainError> {
        self.asked.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        if let Some(reason) = &self.failing_with {
            return Err(DomainError::InvalidInput(reason.clone()));
        }

        Ok(self.observed.clone())
    }
}
