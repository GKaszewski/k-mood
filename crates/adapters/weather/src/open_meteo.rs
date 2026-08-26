use chrono::{DateTime, FixedOffset};

use domain::errors::DomainError;
use domain::location::Coordinates;
use domain::provider::ProviderName;
use domain::weather::{Celsius, Weather};

use super::wmo::condition_for;

const ARCHIVE_URL: &str = "https://archive-api.open-meteo.com/v1/archive";
const FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast";
const PROVIDER: &str = "open-meteo";
const HOURLY_FIELDS: &str = "temperature_2m,weather_code";
const DAYS_THE_ARCHIVE_LAGS_BEHIND: i64 = 6;

pub struct OpenMeteoWeatherLookup {
    http: reqwest::Client,
    provider: ProviderName,
}

impl OpenMeteoWeatherLookup {
    pub fn new(http: reqwest::Client) -> Self {
        Self {
            http,
            provider: ProviderName::from_persistence(PROVIDER.into()),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct HourlyResponse {
    hourly: Option<Hourly>,
}

#[derive(Debug, serde::Deserialize)]
struct Hourly {
    time: Vec<String>,
    #[serde(rename = "temperature_2m")]
    temperature: Vec<Option<f64>>,
    #[serde(rename = "weather_code")]
    code: Vec<Option<u8>>,
}

#[async_trait::async_trait]
impl domain::ports::WeatherLookupPort for OpenMeteoWeatherLookup {
    fn provider(&self) -> &ProviderName {
        &self.provider
    }

    async fn observed_at(
        &self,
        coordinates: &Coordinates,
        instant: &DateTime<FixedOffset>,
    ) -> Result<Option<Weather>, DomainError> {
        let day = instant.date_naive().to_string();
        let url = url_for(instant);

        let response = self
            .http
            .get(url)
            .query(&[
                ("latitude", coordinates.latitude().value().to_string()),
                ("longitude", coordinates.longitude().value().to_string()),
                ("start_date", day.clone()),
                ("end_date", day),
                ("hourly", HOURLY_FIELDS.to_string()),
                ("timezone", "UTC".to_string()),
            ])
            .send()
            .await
            .map_err(|error| {
                DomainError::InvalidInput(format!("weather lookup failed: {error}"))
            })?;

        if !response.status().is_success() {
            return Err(DomainError::InvalidInput(format!(
                "the weather provider answered {}",
                response.status()
            )));
        }

        let hourly: HourlyResponse = response.json().await.map_err(|error| {
            DomainError::InvalidInput(format!(
                "the weather provider sent something unreadable: {error}"
            ))
        })?;

        Ok(self.nearest_hour(hourly.hourly, instant))
    }
}

impl OpenMeteoWeatherLookup {
    fn nearest_hour(
        &self,
        hourly: Option<Hourly>,
        instant: &DateTime<FixedOffset>,
    ) -> Option<Weather> {
        let hourly = hourly?;
        let wanted = instant.naive_utc().format("%Y-%m-%dT%H:00").to_string();

        let at = hourly.time.iter().position(|hour| hour == &wanted)?;

        let code = (*hourly.code.get(at)?)?;
        let degrees = (*hourly.temperature.get(at)?)?;

        Some(Weather::new(
            condition_for(code)?,
            Celsius::new(degrees).ok()?,
            self.provider.clone(),
        ))
    }
}

fn url_for(instant: &DateTime<FixedOffset>) -> &'static str {
    let lag = chrono::Utc::now() - instant.with_timezone(&chrono::Utc);

    if lag.num_days() >= DAYS_THE_ARCHIVE_LAGS_BEHIND {
        return ARCHIVE_URL;
    }

    FORECAST_URL
}
