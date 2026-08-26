use super::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPreferences {
    user_id: UserId,
    tracks_cycle: bool,
}

impl UserPreferences {
    pub fn off_by_default(user_id: UserId) -> Self {
        Self {
            user_id,
            tracks_cycle: false,
        }
    }

    pub fn from_persistence(user_id: UserId, tracks_cycle: bool) -> Self {
        Self {
            user_id,
            tracks_cycle,
        }
    }
}

impl UserPreferences {
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn tracks_cycle(&self) -> bool {
        self.tracks_cycle
    }

    pub fn track_cycle(&mut self, tracks_cycle: bool) {
        self.tracks_cycle = tracks_cycle;
    }
}
