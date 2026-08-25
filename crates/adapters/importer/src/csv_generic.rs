use domain::errors::DomainError;
use domain::ports::ImportedRow;

pub struct CsvImportConfig {
    pub date_column: usize,
    pub time_column: usize,
    pub mood_column: usize,
    pub activities_column: Option<usize>,
    pub note_column: Option<usize>,
    pub activities_separator: String,
    pub mood_mapping: Vec<(String, u8)>,
    pub delimiter: u8,
}

impl Default for CsvImportConfig {
    fn default() -> Self {
        Self {
            date_column: 0,
            time_column: 1,
            mood_column: 2,
            activities_column: Some(3),
            note_column: Some(4),
            activities_separator: "|".into(),
            mood_mapping: vec![
                ("1".into(), 1),
                ("2".into(), 2),
                ("3".into(), 3),
                ("4".into(), 4),
                ("5".into(), 5),
                ("awful".into(), 1),
                ("bad".into(), 2),
                ("meh".into(), 3),
                ("good".into(), 4),
                ("rad".into(), 5),
            ],
            delimiter: b',',
        }
    }
}

pub struct CsvImportAdapter {
    config: CsvImportConfig,
}

impl CsvImportAdapter {
    pub fn new(config: CsvImportConfig) -> Self {
        Self { config }
    }

    fn map_mood(&self, value: &str) -> Result<u8, DomainError> {
        let normalized = value.trim().to_lowercase();

        if let Ok(num) = normalized.parse::<u8>()
            && (1..=5).contains(&num)
        {
            return Ok(num);
        }

        self.config
            .mood_mapping
            .iter()
            .find(|(label, _)| label.to_lowercase() == normalized)
            .map(|(_, value)| *value)
            .ok_or_else(|| DomainError::InvalidInput(format!("unknown mood value: {value}")))
    }
}

#[async_trait::async_trait]
impl domain::ports::ImportSourcePort for CsvImportAdapter {
    async fn read_entries(&self, data: &[u8]) -> Result<Vec<ImportedRow>, DomainError> {
        let content = std::str::from_utf8(data)
            .map_err(|e| DomainError::InvalidInput(format!("invalid UTF-8: {e}")))?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(self.config.delimiter)
            .from_reader(content.as_bytes());

        let mut rows = Vec::new();

        for result in reader.records() {
            let record =
                result.map_err(|e| DomainError::InvalidInput(format!("CSV parse error: {e}")))?;

            let mood_str = record.get(self.config.mood_column).unwrap_or("").trim();
            let mood = self.map_mood(mood_str)?;

            let date = record
                .get(self.config.date_column)
                .unwrap_or("")
                .trim()
                .to_string();
            let time = record
                .get(self.config.time_column)
                .unwrap_or("")
                .trim()
                .to_string();

            let activities = match self.config.activities_column {
                Some(col) => {
                    let raw = record.get(col).unwrap_or("").trim();
                    if raw.is_empty() {
                        Vec::new()
                    } else {
                        raw.split(&self.config.activities_separator)
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect()
                    }
                }
                None => Vec::new(),
            };

            let note = self
                .config
                .note_column
                .and_then(|col| record.get(col))
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

        tracing::info!(row_count = rows.len(), "parsed CSV import");
        Ok(rows)
    }
}
