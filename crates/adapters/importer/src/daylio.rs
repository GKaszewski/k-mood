use std::collections::HashMap;

use domain::errors::DomainError;
use domain::ports::ImportedRow;

const DATE: &str = "full_date";
const TIME: &str = "time";
const MOOD: &str = "mood";
const ACTIVITIES: &str = "activities";
const NOTE: &str = "note";
const NOTE_TITLE: &str = "note_title";

pub struct DaylioImportAdapter;

#[async_trait::async_trait]
impl domain::ports::ImportSourcePort for DaylioImportAdapter {
    async fn read_entries(&self, data: &[u8]) -> Result<Vec<ImportedRow>, DomainError> {
        let content = std::str::from_utf8(data)
            .map_err(|error| DomainError::InvalidInput(format!("invalid UTF-8: {error}")))?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());

        let columns = columns_of(&mut reader)?;
        let mut rows = Vec::new();

        for result in reader.records() {
            let record = result
                .map_err(|error| DomainError::InvalidInput(format!("CSV parse error: {error}")))?;

            let read = |name: &str| {
                columns
                    .get(name)
                    .and_then(|at| record.get(*at))
                    .unwrap_or("")
                    .trim()
            };

            rows.push(ImportedRow {
                mood: map_daylio_mood(read(MOOD))?,
                date: read(DATE).to_string(),
                time: read(TIME).to_string(),
                activities: split_activities(read(ACTIVITIES)),
                note: whatever_was_written(read(NOTE_TITLE), read(NOTE)),
            });
        }

        tracing::info!(row_count = rows.len(), "parsed Daylio export");

        Ok(rows)
    }
}

fn columns_of<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
) -> Result<HashMap<String, usize>, DomainError> {
    let headers = reader
        .headers()
        .map_err(|error| DomainError::InvalidInput(format!("CSV has no header row: {error}")))?;

    let columns: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(at, name)| (name.trim().to_lowercase(), at))
        .collect();

    for required in [DATE, TIME, MOOD] {
        if !columns.contains_key(required) {
            return Err(DomainError::InvalidInput(format!(
                "this does not look like a Daylio export: no {required} column"
            )));
        }
    }

    Ok(columns)
}

fn split_activities(written: &str) -> Vec<String> {
    written
        .split('|')
        .map(|activity| activity.trim().to_string())
        .filter(|activity| !activity.is_empty())
        .collect()
}

fn whatever_was_written(title: &str, note: &str) -> Option<String> {
    let written = [title, note]
        .iter()
        .filter(|part| !part.is_empty())
        .copied()
        .collect::<Vec<&str>>()
        .join("\n\n");

    Some(written).filter(|written| !written.is_empty())
}

fn map_daylio_mood(mood: &str) -> Result<u8, DomainError> {
    match mood.to_lowercase().as_str() {
        "awful" => Ok(1),
        "bad" => Ok(2),
        "meh" => Ok(3),
        "good" => Ok(4),
        "rad" => Ok(5),
        other => Err(DomainError::InvalidInput(format!(
            "unknown Daylio mood: {other}"
        ))),
    }
}
