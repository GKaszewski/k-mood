use domain::entry::{DateRange, EntrySelection, Pagination};
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
