use std::collections::HashMap;

use domain::dimension::ComposedEntry;
use domain::errors::DomainError;
use domain::ports::SharedExtract;

const HEADING: &str = "# Mood journal";
const PREAMBLE: &str = "A shareable extract: mood, what was written, and what was tagged. It deliberately carries \
     nothing else — no places, no health readings, no cycle records — and it is not a backup.";

pub struct MarkdownExtractWriter;

#[async_trait::async_trait]
impl domain::ports::ExtractWriterPort for MarkdownExtractWriter {
    async fn write(&self, extract: &SharedExtract) -> Result<Vec<u8>, DomainError> {
        let names: HashMap<String, String> = extract
            .activities
            .iter()
            .map(|activity| {
                (
                    activity.id().value().to_string(),
                    activity.name().value().to_string(),
                )
            })
            .collect();

        let mut entries: Vec<&ComposedEntry> = extract.entries.iter().collect();
        entries.sort_by_key(|composed| *composed.entry.logged_at());

        let mut document = format!("{HEADING}\n\n{PREAMBLE}\n");
        let mut current_day = String::new();

        for composed in entries {
            let logged_at = composed.entry.logged_at();
            let day = logged_at.format("%A %-d %B %Y").to_string();

            if day != current_day {
                document.push_str(&format!("\n## {day}\n"));
                current_day = day;
            }

            document.push_str(&format!(
                "\n**{}** — {:?}\n",
                logged_at.format("%H:%M"),
                composed.entry.mood()
            ));

            let tagged = tags(composed, &names);
            if !tagged.is_empty() {
                document.push_str(&format!("\n_{}_\n", tagged.join(", ")));
            }

            if let Some(content) = composed.content() {
                document.push_str(&format!("\n{}\n", content.value()));
            }
        }

        Ok(document.into_bytes())
    }
}

fn tags(composed: &ComposedEntry, names: &HashMap<String, String>) -> Vec<String> {
    composed
        .activities()
        .iter()
        .map(|id| {
            let id = id.value().to_string();
            names.get(&id).cloned().unwrap_or(id)
        })
        .collect()
}
