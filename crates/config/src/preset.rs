#[derive(Debug, Clone, serde::Deserialize)]
pub struct PresetActivity {
    pub name: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct PresetConfig {
    pub activities: Vec<PresetActivity>,
}

impl Default for PresetConfig {
    fn default() -> Self {
        Self {
            activities: default_activities(),
        }
    }
}

fn default_activities() -> Vec<PresetActivity> {
    let groups: &[(&str, &[&str])] = &[
        ("social", &["family", "friends", "date", "party"]),
        (
            "hobbies",
            &["movies & tv", "reading", "gaming", "sport", "relax"],
        ),
        (
            "sleep",
            &["sleep early", "good sleep", "medium sleep", "bad sleep"],
        ),
        (
            "health",
            &["exercise", "eat healthy", "drink water", "walk"],
        ),
        ("chores", &["shopping", "cleaning", "cooking", "laundry"]),
    ];

    groups
        .iter()
        .flat_map(|(category, names)| {
            names.iter().map(move |name| PresetActivity {
                name: (*name).into(),
                category: Some((*category).into()),
            })
        })
        .collect()
}
