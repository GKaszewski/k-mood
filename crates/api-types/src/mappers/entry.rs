use chrono::Utc;

use config::EntryConfig;
use domain::entry::{Content, DateRange, Mood, MoodEntry, MoodEntryId};
use domain::user::UserId;

use application::entry::commands::{CreateEntryCommand, UpdateEntryCommand};
use application::entry::queries::{
    FilterByActivityQuery, FilterByMoodQuery, ListEntriesQuery, MoodStatsQuery,
};

use crate::errors::ApiValidationError;
use crate::requests::{CreateEntryRequest, DateRangeParams, ListEntriesParams, UpdateEntryRequest};
use crate::responses::EntryResponse;

use super::shared::{parse_datetime, parse_uuids_as, validate_content_length};

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
        let content = parse_content(self.content, config)?;
        let activities = parse_uuids_as(
            &self.activity_ids.unwrap_or_default(),
            config.max_activities_per_entry,
            "activities",
        )?;
        let photos = parse_uuids_as(
            &self.photo_ids.unwrap_or_default(),
            config.max_photos,
            "photos",
        )?;
        let voice_memos = parse_uuids_as(
            &self.voice_memo_ids.unwrap_or_default(),
            config.max_voice_memos,
            "voice memos",
        )?;

        Ok(CreateEntryCommand {
            user_id,
            mood,
            logged_at,
            activities,
            content,
            photos,
            voice_memos,
        })
    }
}

impl UpdateEntryRequest {
    pub fn into_command(
        self,
        entry_id: MoodEntryId,
        config: &EntryConfig,
    ) -> Result<UpdateEntryCommand, ApiValidationError> {
        let mood = Mood::try_from(self.mood)?;
        let logged_at = match self.logged_at {
            Some(dt) => parse_datetime(&dt)?,
            None => Utc::now().fixed_offset(),
        };
        let content = parse_content(self.content, config)?;
        let activities = parse_uuids_as(
            &self.activity_ids.unwrap_or_default(),
            config.max_activities_per_entry,
            "activities",
        )?;
        let photos = parse_uuids_as(
            &self.photo_ids.unwrap_or_default(),
            config.max_photos,
            "photos",
        )?;
        let voice_memos = parse_uuids_as(
            &self.voice_memo_ids.unwrap_or_default(),
            config.max_voice_memos,
            "voice memos",
        )?;

        Ok(UpdateEntryCommand {
            entry_id,
            mood,
            logged_at,
            activities,
            content,
            photos,
            voice_memos,
        })
    }
}

impl ListEntriesParams {
    pub fn into_query(self, user_id: UserId) -> Result<ListEntriesQuery, ApiValidationError> {
        let range = match (self.from, self.to) {
            (Some(from), Some(to)) => Some(DateRange::new(
                parse_datetime(&from)?,
                parse_datetime(&to)?,
            )?),
            _ => None,
        };
        Ok(ListEntriesQuery {
            user_id,
            range,
            limit: self.limit,
            offset: self.offset,
        })
    }
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

    pub fn into_mood_filter(
        self,
        user_id: UserId,
        mood: u8,
    ) -> Result<FilterByMoodQuery, ApiValidationError> {
        let mood = Mood::try_from(mood)?;
        Ok(FilterByMoodQuery { user_id, mood })
    }

    pub fn into_activity_filter(
        self,
        user_id: UserId,
        activity_id: &str,
    ) -> Result<FilterByActivityQuery, ApiValidationError> {
        let activity_id = super::shared::parse_uuid(activity_id)?.into();
        Ok(FilterByActivityQuery {
            user_id,
            activity_id,
        })
    }
}

impl From<MoodEntry> for EntryResponse {
    fn from(entry: MoodEntry) -> Self {
        let photo_ids: Vec<uuid::Uuid> = entry.photos().iter().map(|p| p.value()).collect();
        let voice_memo_ids: Vec<uuid::Uuid> =
            entry.voice_memos().iter().map(|v| v.value()).collect();

        Self {
            id: entry.id().value(),
            user_id: entry.user_id().value(),
            mood: entry.mood().value(),
            mood_label: format!("{:?}", entry.mood()),
            logged_at: *entry.logged_at(),
            activities: entry.activities().iter().map(|a| a.value()).collect(),
            content: entry.content().map(|c| c.value().to_string()),
            photo_urls: photo_ids
                .iter()
                .map(|id| format!("/api/v1/media/photos/{id}"))
                .collect(),
            photos: photo_ids,
            voice_memo_urls: voice_memo_ids
                .iter()
                .map(|id| format!("/api/v1/media/voice-memos/{id}"))
                .collect(),
            voice_memos: voice_memo_ids,
            created_at: *entry.created_at(),
            updated_at: *entry.updated_at(),
        }
    }
}

fn parse_content(
    content: Option<String>,
    config: &EntryConfig,
) -> Result<Option<Content>, ApiValidationError> {
    content
        .filter(|s| !s.trim().is_empty())
        .map(|text| {
            validate_content_length(&text, config.max_content_length)?;
            Content::new(text).map_err(ApiValidationError::from)
        })
        .transpose()
}
