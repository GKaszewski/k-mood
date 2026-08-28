use std::collections::HashMap;
use std::sync::RwLock;

use super::InMemoryDimensionStore;
use crate::activity::{Activity, ActivityId};
use crate::api_token::ApiToken;
use crate::auth::RefreshSession;
use crate::entry::Date;
use crate::entry::{MoodEntry, MoodEntryId};
use crate::events::EventEnvelope;
use crate::job::Job;
use crate::metric::DailyMetric;
use crate::provider::ProviderConnection;
use crate::push::{PushSubscription, PushSubscriptionId};
use crate::rejection::RejectedMetric;
use crate::reminder::{Reminder, ReminderId};
use crate::song::RecordingId;
use crate::user::{User, UserId, UserPreferences};

pub struct InMemoryStore {
    pub(super) entries: RwLock<HashMap<MoodEntryId, MoodEntry>>,
    pub(super) entry_activities: RwLock<HashMap<MoodEntryId, Vec<ActivityId>>>,
    pub(super) activities: RwLock<HashMap<ActivityId, Activity>>,
    pub(super) users: RwLock<HashMap<UserId, User>>,
    pub(super) reminders: RwLock<HashMap<ReminderId, Reminder>>,
    pub refresh_sessions: RwLock<Vec<RefreshSession>>,
    pub(super) push_subscriptions: RwLock<HashMap<PushSubscriptionId, PushSubscription>>,
    pub(super) provider_connections: RwLock<Vec<ProviderConnection>>,
    pub(super) daily_metrics: RwLock<Vec<DailyMetric>>,
    pub(super) api_tokens: RwLock<Vec<ApiToken>>,
    pub(super) rejections: RwLock<Vec<RejectedMetric>>,
    pub(super) cycle_starts: RwLock<Vec<(UserId, Date)>>,
    pub(super) preferences: RwLock<Vec<UserPreferences>>,
    pub(super) jobs: RwLock<Vec<Job>>,
    pub(super) unidentified_songs: RwLock<Vec<(MoodEntryId, String, String)>>,
    pub(super) recordings: RwLock<Vec<(MoodEntryId, RecordingId)>>,
    pub(super) places_without_weather: RwLock<
        Vec<(
            MoodEntryId,
            crate::location::Coordinates,
            chrono::DateTime<chrono::FixedOffset>,
        )>,
    >,
    pub(super) observed_weather: RwLock<Vec<(MoodEntryId, crate::weather::Weather)>>,
    pub(super) dependent_dimensions: RwLock<Vec<std::sync::Arc<InMemoryDimensionStore>>>,
    pub(super) events: RwLock<Vec<EventEnvelope>>,
    pub(super) sent_reminders: RwLock<Vec<UserId>>,
    pub(super) media_owners: RwLock<HashMap<crate::attachment::MediaRef, UserId>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            entry_activities: RwLock::new(HashMap::new()),
            activities: RwLock::new(HashMap::new()),
            users: RwLock::new(HashMap::new()),
            reminders: RwLock::new(HashMap::new()),
            refresh_sessions: RwLock::new(Vec::new()),
            push_subscriptions: RwLock::new(HashMap::new()),
            provider_connections: RwLock::new(Vec::new()),
            daily_metrics: RwLock::new(Vec::new()),
            api_tokens: RwLock::new(Vec::new()),
            rejections: RwLock::new(Vec::new()),
            cycle_starts: RwLock::new(Vec::new()),
            preferences: RwLock::new(Vec::new()),
            jobs: RwLock::new(Vec::new()),
            unidentified_songs: RwLock::new(Vec::new()),
            recordings: RwLock::new(Vec::new()),
            places_without_weather: RwLock::new(Vec::new()),
            observed_weather: RwLock::new(Vec::new()),
            dependent_dimensions: RwLock::new(Vec::new()),
            events: RwLock::new(Vec::new()),
            sent_reminders: RwLock::new(Vec::new()),
            media_owners: RwLock::new(HashMap::new()),
        }
    }

    pub fn cascades_to(&self, dimensions: &std::sync::Arc<InMemoryDimensionStore>) {
        self.dependent_dimensions
            .write()
            .unwrap()
            .push(dimensions.clone());
    }

    pub(super) fn cascade_to_dimensions(&self, entry_ids: &[MoodEntryId]) {
        for store in self.dependent_dimensions.read().unwrap().iter() {
            for entry_id in entry_ids {
                store.forget(entry_id);
            }
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

impl InMemoryStore {
    pub fn assign_activities(&self, entry_id: &MoodEntryId, activities: Vec<ActivityId>) {
        self.entry_activities
            .write()
            .unwrap()
            .insert(entry_id.clone(), activities);
    }
}
