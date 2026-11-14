use std::sync::Arc;

use chrono::{DateTime, FixedOffset, NaiveDate};

use domain::entry::{Date, DateSpan, Mood, MoodEntry};
use domain::ports::{MoodEntryCommandPort, UserCommandPort};
use domain::testing::{InMemoryStore, test_user};
use domain::trend::{Granularity, WeekStart};
use domain::user::{Timezone, User};

use application::entry::queries::MoodTrendQuery;
use application::entry::use_cases::get_mood_trend::{self, MoodTrend};

fn date(year: i32, month: u32, day: u32) -> Date {
    Date::from_persistence(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

fn at(instant: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(instant).unwrap()
}

async fn a_user(store: &Arc<InMemoryStore>) -> User {
    let mut user = test_user("alice");
    user.update_timezone(Some(Timezone::new("UTC").unwrap()));
    UserCommandPort::save(store.as_ref(), &user).await.unwrap();
    user
}

async fn logged(store: &Arc<InMemoryStore>, user: &User, mood: Mood, instant: &str) {
    let entry = MoodEntry::new(user.id().clone(), mood, at(instant));
    MoodEntryCommandPort::save(store.as_ref(), &entry)
        .await
        .unwrap();
}

async fn trend_of(
    store: &Arc<InMemoryStore>,
    user: &User,
    span: DateSpan,
    week_start: WeekStart,
) -> MoodTrend {
    let deps = get_mood_trend::Deps {
        entries: store.clone(),
        users: store.clone(),
    };
    let query = MoodTrendQuery {
        user_id: user.id().clone(),
        span,
        week_start,
    };

    get_mood_trend::execute(query, &deps).await.unwrap()
}

#[tokio::test]
async fn calendar_weeks_honour_a_sunday_start() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user(&store).await;
    let span = DateSpan::new(date(2026, 9, 1), date(2026, 10, 31)).unwrap();

    let sunday = trend_of(&store, &user, span, WeekStart::Sunday).await;
    let monday = trend_of(&store, &user, span, WeekStart::Monday).await;

    assert_eq!(sunday.granularity, Granularity::Week);
    assert_eq!(*sunday.points[0].start(), date(2026, 8, 30));
    assert_eq!(*monday.points[0].start(), date(2026, 8, 31));
}

#[tokio::test]
async fn an_unlogged_week_is_a_gap() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user(&store).await;
    logged(&store, &user, Mood::Good, "2026-09-02T12:00:00+00:00").await;
    logged(&store, &user, Mood::Good, "2026-10-28T12:00:00+00:00").await;
    let span = DateSpan::new(date(2026, 9, 1), date(2026, 10, 31)).unwrap();

    let trend = trend_of(&store, &user, span, WeekStart::Monday).await;

    assert!(trend.points.iter().any(|point| point.mean_mood().is_none()));
    assert!(trend.points[0].mean_mood().is_some());
}

#[tokio::test]
async fn a_thirty_day_span_is_drawn_in_days() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user(&store).await;
    let span = DateSpan::new(date(2026, 10, 1), date(2026, 10, 30)).unwrap();

    let trend = trend_of(&store, &user, span, WeekStart::Monday).await;

    assert_eq!(trend.granularity, Granularity::Day);
    assert_eq!(trend.points.len(), 30);
}

#[tokio::test]
async fn a_long_imported_history_is_not_refused() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user(&store).await;
    let span = DateSpan::new(date(2020, 1, 1), date(2026, 1, 1)).unwrap();

    let trend = trend_of(&store, &user, span, WeekStart::Monday).await;

    assert_eq!(trend.granularity, Granularity::Month);
}

#[tokio::test]
async fn a_year_back_from_today_is_still_drawn_in_weeks() {
    let store = Arc::new(InMemoryStore::new());
    let user = a_user(&store).await;
    let span = DateSpan::new(date(2025, 10, 14), date(2026, 10, 14)).unwrap();
    assert_eq!(span.days(), 366);

    let trend = trend_of(&store, &user, span, WeekStart::Monday).await;

    assert_eq!(trend.granularity, Granularity::Week);
}
