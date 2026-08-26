use std::sync::Arc;

use domain::cycle::{CycleCalendar, CyclePosition};
use domain::entry::Date;
use domain::ports::{CycleStartQueryPort, UserPreferencesQueryPort, UserQueryPort};
use domain::user::UserId;

use crate::day::{timezone_for, today_in};
use crate::errors::ApplicationError;
use crate::user::preferences::preferences_of;

pub struct CycleView {
    pub tracking: bool,
    pub starts: Vec<Date>,
    pub today: Option<CyclePosition>,
    pub usual_length: i64,
}

pub struct Deps {
    pub query: Arc<dyn CycleStartQueryPort>,
    pub preferences: Arc<dyn UserPreferencesQueryPort>,
    pub users: Arc<dyn UserQueryPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<CycleView, ApplicationError> {
    let preferences = preferences_of(&user_id, &deps.preferences).await?;

    if !preferences.tracks_cycle() {
        return Ok(CycleView {
            tracking: false,
            starts: Vec::new(),
            today: None,
            usual_length: 0,
        });
    }

    let calendar = calendar_of(&user_id, &deps.query).await?;
    let timezone = timezone_for(&user_id, &deps.users).await?;
    let today = calendar.position_on(&today_in(&timezone));

    Ok(CycleView {
        tracking: true,
        starts: calendar.starts().to_vec(),
        today,
        usual_length: calendar.usual_length(),
    })
}

pub async fn calendar_of(
    user_id: &UserId,
    query: &Arc<dyn CycleStartQueryPort>,
) -> Result<CycleCalendar, ApplicationError> {
    Ok(CycleCalendar::new(query.find_by_user(user_id).await?))
}
