use sqlx::sqlite::SqlitePoolOptions;

use domain::dimension::{DimensionKind, DimensionValue};
use domain::entry::{Mood, MoodEntry, MoodEntryId};
use domain::location::Coordinates;
use domain::ports::{EntryDimensionPort, MoodEntryCommandPort, UserCommandPort};
use domain::provider::ProviderName;
use domain::testing::test_user;
use domain::weather::{Celsius, Condition, Weather};

use sqlite::repositories::{
    SqliteEntryCommandRepository, SqliteLocationDimensionRepository, SqliteUserCommandRepository,
    SqliteWeatherDimensionRepository,
};

async fn an_entry() -> (sqlx::SqlitePool, MoodEntryId) {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlite::run_migrations(&pool).await.unwrap();

    let user = test_user("alice");
    SqliteUserCommandRepository::new(pool.clone())
        .save(&user)
        .await
        .unwrap();

    let entry = MoodEntry::new(
        user.id().clone(),
        Mood::Good,
        chrono::DateTime::parse_from_rfc3339("2026-08-20T14:00:00+02:00").unwrap(),
    );
    SqliteEntryCommandRepository::new(pool.clone())
        .save(&entry)
        .await
        .unwrap();

    (pool, entry.id().clone())
}

fn observed(condition: Condition, degrees: f64) -> DimensionValue {
    DimensionValue::Weather(Weather::new(
        condition,
        Celsius::new(degrees).unwrap(),
        ProviderName::new("open-meteo").unwrap(),
    ))
}

async fn weather_of(pool: &sqlx::SqlitePool, entry_id: &MoodEntryId) -> Option<DimensionValue> {
    SqliteWeatherDimensionRepository::new(pool.clone())
        .load(std::slice::from_ref(entry_id))
        .await
        .unwrap()
        .remove(entry_id)
}

#[tokio::test]
async fn observed_weather_is_stored_and_read_back_whole() {
    let (pool, entry_id) = an_entry().await;
    let weather = SqliteWeatherDimensionRepository::new(pool.clone());

    weather
        .save(&entry_id, &[observed(Condition::Rain, 11.5)])
        .await
        .unwrap();

    let Some(DimensionValue::Weather(stored)) = weather_of(&pool, &entry_id).await else {
        panic!("the weather was not stored");
    };

    assert_eq!(stored.condition(), Condition::Rain);
    assert!((stored.temperature().value() - 11.5).abs() < f64::EPSILON);
    assert_eq!(stored.observed_by().value(), "open-meteo");
}

#[tokio::test]
async fn editing_an_entry_cannot_erase_what_a_provider_observed() {
    let (pool, entry_id) = an_entry().await;
    let weather = SqliteWeatherDimensionRepository::new(pool.clone());
    weather
        .save(&entry_id, &[observed(Condition::Rain, 11.5)])
        .await
        .unwrap();

    let an_edit_that_says_nothing_about_weather = vec![DimensionValue::Location(
        Coordinates::new(52.2297, 21.0122).unwrap(),
    )];
    weather
        .save(&entry_id, &an_edit_that_says_nothing_about_weather)
        .await
        .unwrap();

    assert!(
        weather_of(&pool, &entry_id).await.is_some(),
        "weather is observed, not stated: an edit that omits it is not a request to delete it"
    );
}

#[tokio::test]
async fn a_later_observation_replaces_an_earlier_one() {
    let (pool, entry_id) = an_entry().await;
    let weather = SqliteWeatherDimensionRepository::new(pool.clone());

    weather
        .save(&entry_id, &[observed(Condition::Clear, 20.0)])
        .await
        .unwrap();
    weather
        .save(&entry_id, &[observed(Condition::Snow, -2.0)])
        .await
        .unwrap();

    let Some(DimensionValue::Weather(stored)) = weather_of(&pool, &entry_id).await else {
        panic!("the weather was lost");
    };

    assert_eq!(stored.condition(), Condition::Snow);

    let rows: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM entry_weather")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(rows.0, 1);
}

#[tokio::test]
async fn a_condition_this_build_does_not_know_is_skipped() {
    let (pool, entry_id) = an_entry().await;

    sqlx::query(
        "INSERT INTO entry_weather (entry_id, condition, temperature, observed_by)
         VALUES (?, 'raining frogs', 11.5, 'open-meteo')",
    )
    .bind(entry_id.value().to_string())
    .execute(&pool)
    .await
    .unwrap();

    assert!(weather_of(&pool, &entry_id).await.is_none());
}

#[tokio::test]
async fn deleting_an_entry_takes_its_weather_with_it() {
    let (pool, entry_id) = an_entry().await;
    SqliteWeatherDimensionRepository::new(pool.clone())
        .save(&entry_id, &[observed(Condition::Rain, 11.5)])
        .await
        .unwrap();

    SqliteEntryCommandRepository::new(pool.clone())
        .delete(&entry_id)
        .await
        .unwrap();

    let rows: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM entry_weather")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(rows.0, 0);
}

#[tokio::test]
async fn weather_reports_its_own_kind() {
    assert_eq!(observed(Condition::Fog, 3.0).kind(), DimensionKind::Weather);
    let _ = SqliteLocationDimensionRepository::new;
}

#[tokio::test]
async fn a_place_with_no_weather_is_backlogged_and_then_is_not() {
    use domain::ports::WeatherBacklogQueryPort;
    use sqlite::repositories::SqliteWeatherBacklogRepository;

    let (pool, entry_id) = an_entry().await;

    SqliteLocationDimensionRepository::new(pool.clone())
        .save(
            &entry_id,
            &[DimensionValue::Location(
                Coordinates::new(52.2297, 21.0122).unwrap(),
            )],
        )
        .await
        .unwrap();

    let backlog = SqliteWeatherBacklogRepository::new(pool.clone());

    let waiting = backlog.find_places_without_weather(50).await.unwrap();
    assert_eq!(waiting.len(), 1);
    assert_eq!(waiting[0].entry_id, entry_id);
    assert!((waiting[0].coordinates.latitude().value() - 52.2297).abs() < 1e-9);

    SqliteWeatherDimensionRepository::new(pool.clone())
        .save(&entry_id, &[observed(Condition::Rain, 11.5)])
        .await
        .unwrap();

    assert!(
        backlog
            .find_places_without_weather(50)
            .await
            .unwrap()
            .is_empty(),
        "an entry that now has weather is no longer waiting for any"
    );
}

#[tokio::test]
async fn only_entries_that_know_where_they_were_are_backlogged() {
    use domain::ports::WeatherBacklogQueryPort;
    use sqlite::repositories::SqliteWeatherBacklogRepository;

    let (pool, somewhere_known) = an_entry().await;
    SqliteLocationDimensionRepository::new(pool.clone())
        .save(
            &somewhere_known,
            &[DimensionValue::Location(
                Coordinates::new(52.2297, 21.0122).unwrap(),
            )],
        )
        .await
        .unwrap();

    let owner: (String,) = sqlx::query_as("SELECT id FROM users LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    let nowhere = MoodEntry::new(
        domain::user::UserId::from_uuid(owner.0.parse().unwrap()),
        Mood::Meh,
        chrono::DateTime::parse_from_rfc3339("2026-08-21T09:00:00+02:00").unwrap(),
    );
    SqliteEntryCommandRepository::new(pool.clone())
        .save(&nowhere)
        .await
        .unwrap();

    let waiting = SqliteWeatherBacklogRepository::new(pool)
        .find_places_without_weather(50)
        .await
        .unwrap();

    let ids: Vec<&MoodEntryId> = waiting.iter().map(|place| &place.entry_id).collect();

    assert_eq!(
        ids,
        [&somewhere_known],
        "weather needs somewhere to have happened"
    );
}

#[tokio::test]
async fn the_backlog_is_bounded_by_what_is_asked_for() {
    use domain::ports::WeatherBacklogQueryPort;
    use sqlite::repositories::SqliteWeatherBacklogRepository;

    let (pool, first) = an_entry().await;
    let locations = SqliteLocationDimensionRepository::new(pool.clone());
    let somewhere = DimensionValue::Location(Coordinates::new(52.2297, 21.0122).unwrap());
    locations
        .save(&first, std::slice::from_ref(&somewhere))
        .await
        .unwrap();

    let owner: (String,) = sqlx::query_as("SELECT id FROM users LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    for hour in 0..5 {
        let entry = MoodEntry::new(
            domain::user::UserId::from_uuid(owner.0.parse().unwrap()),
            Mood::Good,
            chrono::DateTime::parse_from_rfc3339(&format!("2026-08-2{hour}T09:00:00+02:00"))
                .unwrap(),
        );
        SqliteEntryCommandRepository::new(pool.clone())
            .save(&entry)
            .await
            .unwrap();
        locations
            .save(entry.id(), std::slice::from_ref(&somewhere))
            .await
            .unwrap();
    }

    let waiting = SqliteWeatherBacklogRepository::new(pool)
        .find_places_without_weather(3)
        .await
        .unwrap();

    assert_eq!(waiting.len(), 3);
}
