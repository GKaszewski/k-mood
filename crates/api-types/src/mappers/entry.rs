use chrono::Utc;

use config::EntryConfig;
use domain::activity::ActivityId;
use domain::dimension::{ComposedEntry, DimensionValue};
use domain::entry::{DateRange, EntrySelection, Mood, MoodEntryId, Pagination};
use domain::user::UserId;

use application::entry::commands::{CreateEntryCommand, UpdateEntryCommand};
use application::entry::queries::{ListEntriesQuery, MoodStatsQuery};

use crate::dimension::DimensionPayload;
use crate::errors::ApiValidationError;
use crate::params::{DateRangeParams, ListEntriesParams};
use crate::requests::{CreateEntryRequest, UpdateEntryRequest};
use crate::responses::EntryResponse;

use super::shared::{parse_datetime, validate_content_length};

impl CreateEntryRequest {
    pub fn into_command(
        self,
        user_id: UserId,
        config: &EntryConfig,
    ) -> Result<CreateEntryCommand, ApiValidationError> {
        let mood = Mood::try_from(self.mood)?;
        let logged_at = match self.logged_at {
            Some(dt) => parse_datetime(&dt)?,
            None => Utc::now().fixed_offset(),
        };
        let dimensions = parse_dimensions(self.dimensions, config)?;

        Ok(CreateEntryCommand {
            user_id,
            mood,
            logged_at,
            dimensions,
        })
    }
}

impl UpdateEntryRequest {
    pub fn into_command(
        self,
        entry_id: MoodEntryId,
        config: &EntryConfig,
    ) -> Result<UpdateEntryCommand, ApiValidationError> {
        let mood = self.mood.map(Mood::try_from).transpose()?;
        let logged_at = self.logged_at.as_deref().map(parse_datetime).transpose()?;

        let dimensions = match self.dimensions {
            Some(named) => Some(parse_dimensions(Some(named), config)?),
            None => None,
        };

        Ok(UpdateEntryCommand {
            entry_id,
            mood,
            logged_at,
            dimensions,
        })
    }
}

impl ListEntriesParams {
    pub fn into_query(
        self,
        user_id: UserId,
        config: &EntryConfig,
    ) -> Result<ListEntriesQuery, ApiValidationError> {
        let range = match (self.from, self.to) {
            (Some(from), Some(to)) => Some(DateRange::new(
                parse_datetime(&from)?,
                parse_datetime(&to)?,
            )?),
            _ => None,
        };

        let activity = self
            .activity
            .as_deref()
            .map(parse_activity_id)
            .transpose()?;

        let updated_since = self
            .updated_since
            .as_deref()
            .map(parse_datetime)
            .transpose()?
            .map(|instant| instant.with_timezone(&Utc));

        let selection = EntrySelection::everything_of(user_id)
            .logged_within(range)
            .of_mood(self.mood.map(Mood::try_from).transpose()?)
            .tagged_with(activity)
            .changed_since(updated_since);

        let page = Pagination::new(
            self.limit.unwrap_or(config.max_entries_per_page),
            self.offset.unwrap_or(0),
            config.max_entries_per_page,
        )?;

        Ok(ListEntriesQuery { selection, page })
    }
}

fn parse_activity_id(named: &str) -> Result<ActivityId, ApiValidationError> {
    named
        .parse()
        .map(ActivityId::from_uuid)
        .map_err(|_| ApiValidationError::Invalid(format!("{named} is not an activity id")))
}

impl DateRangeParams {
    pub fn into_date_range(self) -> Result<DateRange, ApiValidationError> {
        let from = parse_datetime(&self.from)?;
        let to = parse_datetime(&self.to)?;
        Ok(DateRange::new(from, to)?)
    }

    pub fn into_stats_query(self, user_id: UserId) -> Result<MoodStatsQuery, ApiValidationError> {
        let range = Some(self.into_date_range()?);
        Ok(MoodStatsQuery { user_id, range })
    }
}

impl From<ComposedEntry> for EntryResponse {
    fn from(composed: ComposedEntry) -> Self {
        let entry = &composed.entry;

        Self {
            id: entry.id().value(),
            user_id: entry.user_id().value(),
            mood: entry.mood().value(),
            mood_label: entry.mood().label().to_string(),
            logged_at: *entry.logged_at(),
            photo_urls: composed
                .photos()
                .iter()
                .map(|id| format!("/api/v1/media/photos/{}", id.value()))
                .collect(),
            voice_memo_urls: composed
                .voice_memos()
                .iter()
                .map(|id| format!("/api/v1/media/voice-memos/{}", id.value()))
                .collect(),
            dimensions: composed.dimensions.iter().map(Into::into).collect(),
            created_at: *entry.created_at(),
            updated_at: *entry.updated_at(),
        }
    }
}

fn parse_dimensions(
    payloads: Option<Vec<DimensionPayload>>,
    config: &EntryConfig,
) -> Result<Vec<DimensionValue>, ApiValidationError> {
    payloads
        .unwrap_or_default()
        .into_iter()
        .filter(|payload| !payload.is_empty())
        .map(|payload| {
            within_limits(&payload, config)?;
            payload.into_dimension()
        })
        .collect()
}

fn within_limits(
    payload: &DimensionPayload,
    config: &EntryConfig,
) -> Result<(), ApiValidationError> {
    let (held, most, label) = match payload {
        DimensionPayload::Content { text } => {
            return validate_content_length(text, config.max_content_length);
        }
        DimensionPayload::Activities { ids } => {
            (ids.len(), config.max_activities_per_entry, "activities")
        }
        DimensionPayload::Photos { ids } => (ids.len(), config.max_photos, "photos"),
        DimensionPayload::VoiceMemos { ids } => (ids.len(), config.max_voice_memos, "voice memos"),
        DimensionPayload::Location { .. }
        | DimensionPayload::Song { .. }
        | DimensionPayload::Weather { .. } => return Ok(()),
    };

    if held > most {
        return Err(ApiValidationError::Invalid(format!(
            "too many {label}, maximum is {most}"
        )));
    }

    Ok(())
}
