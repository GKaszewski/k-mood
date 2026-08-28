use std::collections::HashMap;
use std::sync::Arc;

use chrono::FixedOffset;

use config::PresetConfig;
use domain::activity::{ActivityId, ActivityName, CategoryName};
use domain::dimension::{ComposedEntry, DimensionValue};
use domain::entry::{Content, Mood, MoodEntry};
use domain::ports::{
    ActivityCommandPort, ActivityQueryPort, EntryDimensionPort, ImportSourcePort,
    MoodEntryCommandPort, MoodEntryQueryPort, UserQueryPort,
};
use domain::user::Timezone;

use crate::errors::ApplicationError;
use crate::import::already_here::AlreadyHere;

use super::super::commands::ImportCommand;

#[derive(Debug)]
pub struct ImportResult {
    pub imported: u64,
    pub skipped: u64,
    pub errors: Vec<String>,
}

pub struct Deps {
    pub source: Arc<dyn ImportSourcePort>,
    pub entry_command: Arc<dyn MoodEntryCommandPort>,
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
    pub activity_command: Arc<dyn ActivityCommandPort>,
    pub activity_query: Arc<dyn ActivityQueryPort>,
    pub dimensions: Vec<Arc<dyn EntryDimensionPort>>,
    pub users: Arc<dyn UserQueryPort>,
    pub preset: PresetConfig,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: ImportCommand, deps: &Deps) -> Result<ImportResult, ApplicationError> {
    let rows = deps.source.read_entries(&cmd.data).await?;
    tracing::info!(row_count = rows.len(), "parsed import data");

    let timezone = crate::day::timezone_for(&cmd.user_id, &deps.users).await?;

    let existing_entries = deps.entry_query.find_all_by_user(&cmd.user_id).await?;
    let mut already_here = AlreadyHere::holding(&existing_entries);

    let existing_activities = deps.activity_query.find_by_user(&cmd.user_id).await?;
    let mut activity_cache: HashMap<String, ActivityId> = existing_activities
        .iter()
        .map(|a| (a.name().value().to_string(), a.id().clone()))
        .collect();

    let category_map: HashMap<String, String> = deps
        .preset
        .activities
        .iter()
        .filter_map(|a| a.category.as_ref().map(|c| (a.name.clone(), c.clone())))
        .collect();

    let mut batch = Vec::new();
    let mut skipped = 0u64;
    let mut errors = Vec::new();

    for (index, row) in rows.into_iter().enumerate() {
        match build_entry(
            &row,
            &cmd.user_id,
            &mut activity_cache,
            &category_map,
            &mut already_here,
            &timezone,
            deps,
        )
        .await
        {
            Ok(Some(entry)) => batch.push(entry),
            Ok(None) => skipped += 1,
            Err(e) => {
                tracing::warn!(row = index + 1, error = %e, "skipped row during import");
                errors.push(format!("row {}: {e}", index + 1));
                skipped += 1;
            }
        }
    }

    let imported = batch.len() as u64;

    let entries: Vec<MoodEntry> = batch
        .iter()
        .map(|composed| composed.entry.clone())
        .collect();
    deps.entry_command.save_batch(&entries).await?;

    for composed in &batch {
        for port in &deps.dimensions {
            port.save(composed.entry.id(), &composed.dimensions).await?;
        }
    }

    tracing::info!(imported, skipped, "import completed");

    Ok(ImportResult {
        imported,
        skipped,
        errors,
    })
}

async fn build_entry(
    row: &domain::ports::ImportedRow,
    user_id: &domain::user::UserId,
    activity_cache: &mut HashMap<String, ActivityId>,
    category_map: &HashMap<String, String>,
    already_here: &mut AlreadyHere,
    timezone: &Timezone,
    deps: &Deps,
) -> Result<Option<ComposedEntry>, ApplicationError> {
    let mood = Mood::try_from(row.mood)?;
    let logged_at = the_users_own_wall_clock(&row.date, &row.time, timezone)?;

    if already_here.holds(&logged_at, mood) {
        return Ok(None);
    }
    already_here.remember(&logged_at, mood);

    let mut activity_ids = Vec::new();
    for activity_name in &row.activities {
        let id =
            resolve_activity(activity_name, user_id, activity_cache, category_map, deps).await?;
        activity_ids.push(id);
    }

    let mut dimensions: Vec<DimensionValue> = row
        .note
        .as_ref()
        .and_then(|note| Content::new(note).ok())
        .map(DimensionValue::Content)
        .into_iter()
        .collect();

    if !activity_ids.is_empty() {
        dimensions.push(DimensionValue::activities(activity_ids));
    }

    let entry = MoodEntry::new(user_id.clone(), mood, logged_at);

    Ok(Some(ComposedEntry { entry, dimensions }))
}

async fn resolve_activity(
    name: &str,
    user_id: &domain::user::UserId,
    cache: &mut HashMap<String, ActivityId>,
    category_map: &HashMap<String, String>,
    deps: &Deps,
) -> Result<ActivityId, ApplicationError> {
    if let Some(id) = cache.get(name) {
        return Ok(id.clone());
    }

    tracing::debug!(activity_name = name, "creating new activity from import");
    let activity_name = ActivityName::new(name)?;
    let category = category_map
        .get(name)
        .and_then(|c| CategoryName::new(c).ok());

    let activity = domain::activity::Activity::new(user_id.clone(), activity_name, category);
    deps.activity_command.save(&activity).await?;

    let id = activity.id().clone();
    cache.insert(name.to_string(), id.clone());

    Ok(id)
}

fn the_users_own_wall_clock(
    date: &str,
    time: &str,
    timezone: &Timezone,
) -> Result<chrono::DateTime<FixedOffset>, ApplicationError> {
    let written = format!("{date} {time}");

    let wall_clock = chrono::NaiveDateTime::parse_from_str(&written, "%Y-%m-%d %I:%M %p")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(&written, "%Y-%m-%d %H:%M"))
        .map_err(|error| ApplicationError::Validation(format!("invalid date/time: {error}")))?;

    Ok(placed_in(wall_clock, timezone).fixed_offset())
}

fn placed_in(
    wall_clock: chrono::NaiveDateTime,
    timezone: &Timezone,
) -> chrono::DateTime<chrono_tz::Tz> {
    use chrono::offset::LocalResult;

    match wall_clock.and_local_timezone(timezone.resolve()) {
        LocalResult::Single(placed) => placed,
        LocalResult::Ambiguous(earlier, _) => earlier,
        LocalResult::None => an_hour_the_clocks_skipped(wall_clock, timezone),
    }
}

fn an_hour_the_clocks_skipped(
    wall_clock: chrono::NaiveDateTime,
    timezone: &Timezone,
) -> chrono::DateTime<chrono_tz::Tz> {
    let after_the_jump = wall_clock + chrono::Duration::hours(1);

    tracing::warn!(
        wall_clock = %wall_clock,
        timezone = timezone.value(),
        "this wall clock time never happened, so the entry lands an hour later"
    );

    after_the_jump
        .and_local_timezone(timezone.resolve())
        .earliest()
        .unwrap_or_else(|| wall_clock.and_utc().with_timezone(&timezone.resolve()))
}
