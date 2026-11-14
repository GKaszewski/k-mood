use domain::entry::{DateRange, DateSpan, EntrySelection, Pagination};
use domain::trend::WeekStart;
use domain::user::UserId;

#[derive(Debug)]
pub struct ListEntriesQuery {
    pub selection: EntrySelection,
    pub page: Pagination,
}

#[derive(Debug)]
pub struct MoodStatsQuery {
    pub user_id: UserId,
    pub range: Option<DateRange>,
}

#[derive(Debug)]
pub struct MoodTrendQuery {
    pub user_id: UserId,
    pub span: DateSpan,
    pub week_start: WeekStart,
}
