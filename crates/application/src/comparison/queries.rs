use domain::comparison::{ComparisonRules, MaximumSpan};
use domain::entry::DateSpan;
use domain::user::UserId;

#[derive(Debug)]
pub struct ComparisonQuery {
    pub user_id: UserId,
    pub a: DateSpan,
    pub b: DateSpan,
    pub maximum_span: MaximumSpan,
    pub rules: ComparisonRules,
    pub top_music_count: usize,
}
