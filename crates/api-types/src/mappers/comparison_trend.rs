use domain::trend::{Granularity, TrendPoint};

use crate::responses::ComparedTrendResponse;

use super::trend::granularity_name;

impl ComparedTrendResponse {
    pub(super) fn of(granularity: Granularity, a: &[TrendPoint], b: &[TrendPoint]) -> Self {
        Self {
            granularity: granularity_name(granularity).to_string(),
            a: a.iter().map(Into::into).collect(),
            b: b.iter().map(Into::into).collect(),
        }
    }
}
