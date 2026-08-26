use config::EntryConfig;
use domain::entry::MoodEntryId;

use api_types::requests::UpdateEntryRequest;

fn command_from(body: &str) -> application::entry::commands::UpdateEntryCommand {
    let request: UpdateEntryRequest = serde_json::from_str(body).unwrap();
    request
        .into_command(MoodEntryId::generate(), &EntryConfig::default())
        .unwrap()
}

#[test]
fn an_update_omitting_logged_at_asks_for_no_date_change() {
    let cmd = command_from(r#"{"mood":3}"#);

    assert!(cmd.logged_at.is_none());
}

#[test]
fn an_update_naming_logged_at_asks_to_move_the_entry() {
    let cmd = command_from(r#"{"mood":3,"loggedAt":"2024-01-02T09:30:00+01:00"}"#);

    assert_eq!(
        cmd.logged_at.unwrap().to_rfc3339(),
        "2024-01-02T09:30:00+01:00"
    );
}
