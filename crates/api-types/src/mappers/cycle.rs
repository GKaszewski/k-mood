use application::cycle::use_cases::read_cycle::CycleView;
use domain::cycle::CyclePosition;
use domain::user::UserPreferences;

use crate::responses::{CyclePositionResponse, CycleViewResponse, PreferencesResponse};

impl From<CycleView> for CycleViewResponse {
    fn from(view: CycleView) -> Self {
        Self {
            tracking: view.tracking,
            starts: view.starts.iter().map(|date| date.to_string()).collect(),
            today: view.today.map(Into::into),
            usual_length: view.usual_length,
        }
    }
}

impl From<CyclePosition> for CyclePositionResponse {
    fn from(position: CyclePosition) -> Self {
        Self {
            day: position.day().value(),
            progress: position.progress(),
        }
    }
}

impl From<UserPreferences> for PreferencesResponse {
    fn from(preferences: UserPreferences) -> Self {
        Self {
            tracks_cycle: preferences.tracks_cycle(),
        }
    }
}
