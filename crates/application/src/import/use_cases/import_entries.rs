use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::FixedOffset;

use config::PresetConfig;
use domain::activity::{ActivityId, ActivityName, CategoryName};
use domain::entry::{Content, Mood, MoodEntry};
use domain::ports::{
    ActivityCommandPort, ActivityQueryPort, ImportSourcePort, MoodEntryCommandPort,
    MoodEntryQueryPort,
};

use crate::errors::ApplicationError;

use super::super::commands::ImportCommand;

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
    pub preset: PresetConfig,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(cmd: ImportCommand, deps: &Deps) -> Result<ImportResult, ApplicationError> {
    let rows = deps.source.read_entries(&cmd.data).await?;
    tracing::info!(row_count = rows.len(), "parsed import data");

    let existing_entries = deps
        .entry_query
        .find_by_user(&cmd.user_id, None, None)
        .await?;
    let existing_keys: HashSet<(String, u8)> = existing_entries
        .iter()
        .map(|e| (e.logged_at().to_rfc3339(), e.mood().value()))
        .collect();

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
            &existing_keys,
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
    deps.entry_command.save_batch(&batch).await?;

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
    existing_keys: &HashSet<(String, u8)>,
    deps: &Deps,
) -> Result<Option<MoodEntry>, ApplicationError> {
    let mood = Mood::try_from(row.mood)?;
    let logged_at = parse_datetime(&row.date, &row.time)?;

    let key = (logged_at.to_rfc3339(), mood.value());
    if existing_keys.contains(&key) {
        return Ok(None);
    }

    let mut activity_ids = Vec::new();
    for activity_name in &row.activities {
        let id =
            resolve_activity(activity_name, user_id, activity_cache, category_map, deps).await?;
        activity_ids.push(id);
    }

    let content = row.note.as_ref().and_then(|n| Content::new(n).ok());

    let mut entry = MoodEntry::new(user_id.clone(), mood, logged_at);
    entry.set_activities(activity_ids);
    entry.set_content(content);

    Ok(Some(entry))
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

fn parse_datetime(
    date: &str,
    time: &str,
) -> Result<chrono::DateTime<FixedOffset>, ApplicationError> {
    let datetime_str = format!("{date} {time}");

    let naive = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %I:%M %p")
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M"))
        .map_err(|e| ApplicationError::Validation(format!("invalid date/time: {e}")))?;

    let offset = FixedOffset::east_opt(0).expect("UTC offset is always valid");
    Ok(naive.and_local_timezone(offset).unwrap())
}
