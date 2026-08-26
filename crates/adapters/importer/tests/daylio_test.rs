use domain::ports::ImportSourcePort;

use importer::DaylioImportAdapter;

const CURRENT_EXPORT: &str = "full_date,date,weekday,time,mood,activities,scales,note_title,note\n\
2026-08-25,25 Aug,Tuesday,8:00 PM,meh,,,\"\",\"\"\n\
2026-08-24,24 Aug,Monday,8:00 PM,rad,\"friends | walk\",,\"\",\"I KISSED OLA\"\n";

const OLDER_EXPORT_WITHOUT_SCALES: &str = "full_date,date,weekday,time,mood,activities,note_title,note\n\
2026-08-24,24 Aug,Monday,8:00 PM,good,walk,\"A title\",\"A note\"\n";

const COLUMNS_IN_A_DIFFERENT_ORDER: &str = "note,mood,time,full_date,activities\n\
\"reordered\",bad,9:15 PM,2026-08-23,reading\n";

async fn read(csv: &str) -> Vec<domain::ports::ImportedRow> {
    DaylioImportAdapter
        .read_entries(csv.as_bytes())
        .await
        .unwrap()
}

#[tokio::test]
async fn the_note_is_read_from_the_note_column_not_the_title() {
    let rows = read(CURRENT_EXPORT).await;

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].note, None, "an empty note is no note");
    assert_eq!(
        rows[1].note.as_deref(),
        Some("I KISSED OLA"),
        "the note column sits after note_title, and it is the one worth keeping"
    );
}

#[tokio::test]
async fn a_title_and_a_note_are_both_kept() {
    let rows = read(OLDER_EXPORT_WITHOUT_SCALES).await;

    assert_eq!(
        rows[0].note.as_deref(),
        Some("A title\n\nA note"),
        "a Daylio note can have a title, and losing either is losing writing"
    );
}

#[tokio::test]
async fn an_export_without_the_scales_column_still_reads() {
    let rows = read(OLDER_EXPORT_WITHOUT_SCALES).await;

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].mood, 4);
    assert_eq!(rows[0].date, "2026-08-24");
    assert_eq!(rows[0].time, "8:00 PM");
    assert_eq!(rows[0].activities, ["walk"]);
}

#[tokio::test]
async fn columns_are_found_by_name_rather_than_by_position() {
    let rows = read(COLUMNS_IN_A_DIFFERENT_ORDER).await;

    assert_eq!(rows[0].mood, 2);
    assert_eq!(rows[0].date, "2026-08-23");
    assert_eq!(rows[0].time, "9:15 PM");
    assert_eq!(rows[0].note.as_deref(), Some("reordered"));
    assert_eq!(rows[0].activities, ["reading"]);
}

#[tokio::test]
async fn activities_are_split_on_the_pipe_and_trimmed() {
    let rows = read(CURRENT_EXPORT).await;

    assert_eq!(rows[1].activities, ["friends", "walk"]);
}

#[tokio::test]
async fn a_file_that_is_not_a_daylio_export_is_refused_by_name() {
    let refused = DaylioImportAdapter
        .read_entries(b"when,how_i_felt\n2026-08-25,fine\n")
        .await
        .unwrap_err();

    assert!(
        refused
            .to_string()
            .contains("does not look like a Daylio export"),
        "a file with none of the columns is refused for that reason, not for a bad mood: {refused}"
    );
}

#[tokio::test]
async fn a_mood_daylio_never_writes_is_refused() {
    let refused = DaylioImportAdapter
        .read_entries(b"full_date,time,mood\n2026-08-25,8:00 PM,ecstatic\n")
        .await
        .unwrap_err();

    assert!(refused.to_string().contains("ecstatic"));
}
