use std::collections::HashMap;
use std::sync::RwLock;

use crate::activity::{Activity, ActivityId};
use crate::auth::RefreshSession;
use crate::entry::{MoodEntry, MoodEntryId};
use crate::events::EventEnvelope;
use crate::push::{PushSubscription, PushSubscriptionId};
use crate::reminder::{Reminder, ReminderId};
use crate::user::{User, UserId};

pub struct InMemoryStore {
    pub(super) entries: RwLock<HashMap<MoodEntryId, MoodEntry>>,
    pub(super) activities: RwLock<HashMap<ActivityId, Activity>>,
    pub(super) users: RwLock<HashMap<UserId, User>>,
    pub(super) reminders: RwLock<HashMap<ReminderId, Reminder>>,
    pub refresh_sessions: RwLock<Vec<RefreshSession>>,
    pub(super) push_subscriptions: RwLock<HashMap<PushSubscriptionId, PushSubscription>>,
    pub(super) events: RwLock<Vec<EventEnvelope>>,
    pub(super) sent_reminders: RwLock<Vec<UserId>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            activities: RwLock::new(HashMap::new()),
            users: RwLock::new(HashMap::new()),
            reminders: RwLock::new(HashMap::new()),
            refresh_sessions: RwLock::new(Vec::new()),
            push_subscriptions: RwLock::new(HashMap::new()),
            events: RwLock::new(Vec::new()),
            sent_reminders: RwLock::new(Vec::new()),
        }
    }

    pub fn published_events(&self) -> Vec<EventEnvelope> {
        self.events.read().unwrap().clone()
    }

    pub fn sent_reminders(&self) -> Vec<UserId> {
        self.sent_reminders.read().unwrap().clone()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.read().unwrap().len()
    }

    pub fn activity_count(&self) -> usize {
        self.activities.read().unwrap().len()
    }

    pub fn reminder_count(&self) -> usize {
        self.reminders.read().unwrap().len()
    }

    pub fn user_count(&self) -> usize {
        self.users.read().unwrap().len()
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}
