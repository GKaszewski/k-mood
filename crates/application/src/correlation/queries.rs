use domain::entry::DateSpan;
use domain::user::UserId;

#[derive(Debug)]
pub struct CorrelationQuery {
    pub user_id: UserId,
    pub span: DateSpan,
    pub minimum_sample_size: usize,
    pub false_discovery_rate: f64,
}
