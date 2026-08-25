use domain::errors::DomainError;
use domain::ports::ImportedRow;

pub struct DaylioImportAdapter;

#[async_trait::async_trait]
impl domain::ports::ImportSourcePort for DaylioImportAdapter {
    async fn read_entries(&self, data: &[u8]) -> Result<Vec<ImportedRow>, DomainError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| DomainError::InvalidInput(format!("invalid UTF-8: {e}")))?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(content.as_bytes());

        let mut rows = Vec::new();

        for result in reader.records() {
            let record =
                result.map_err(|e| DomainError::InvalidInput(format!("CSV parse error: {e}")))?;

            let mood_str = record.get(4).unwrap_or("").trim();
            let mood = map_daylio_mood(mood_str)?;

            let date = record.get(0).unwrap_or("").trim().to_string();
            let time = record.get(3).unwrap_or("").trim().to_string();

            let activities_str = record.get(5).unwrap_or("").trim();
            let activities = if activities_str.is_empty() {
                Vec::new()
            } else {
                activities_str
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            };

            let note = record
                .get(7)
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            rows.push(ImportedRow {
                mood,
                date,
                time,
                activities,
                note,
            });
        }

        tracing::info!(row_count = rows.len(), "parsed Daylio export");
        Ok(rows)
    }
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
