use std::sync::Arc;

use config::{AuthConfig, EntryConfig, PresetConfig, PushConfig, ServerConfig};
use domain::ports::{
    ActivityCommandPort, ActivityQueryPort, AuthServicePort, CascadeDeletePort, EventPublisherPort,
    ExportPort, ImportSourcePort, MediaStoragePort, MoodEntryCommandPort, MoodEntryQueryPort,
    PasswordHasherPort, PushSubscriptionCommandPort, PushSubscriptionQueryPort,
    RefreshSessionCommandPort, RefreshSessionQueryPort, ReminderCommandPort, ReminderQueryPort,
    ReminderSenderPort, UserCommandPort, UserQueryPort,
};

#[derive(Clone)]
pub struct AppState {
    pub entry_command: Arc<dyn MoodEntryCommandPort>,
    pub entry_query: Arc<dyn MoodEntryQueryPort>,
    pub activity_command: Arc<dyn ActivityCommandPort>,
    pub activity_query: Arc<dyn ActivityQueryPort>,
    pub user_command: Arc<dyn UserCommandPort>,
    pub user_query: Arc<dyn UserQueryPort>,
    pub reminder_command: Arc<dyn ReminderCommandPort>,
    pub reminder_query: Arc<dyn ReminderQueryPort>,
    pub refresh_session_command: Arc<dyn RefreshSessionCommandPort>,
    pub refresh_session_query: Arc<dyn RefreshSessionQueryPort>,
    pub cascade: Arc<dyn CascadeDeletePort>,
    pub auth_service: Arc<dyn AuthServicePort>,
    pub password_hasher: Arc<dyn PasswordHasherPort>,
    pub event_publisher: Arc<dyn EventPublisherPort>,
    pub media_storage: Arc<dyn MediaStoragePort>,
    pub export_port: Arc<dyn ExportPort>,
    pub import_source: Arc<dyn ImportSourcePort>,
    pub push_subscription_command: Arc<dyn PushSubscriptionCommandPort>,
    pub push_subscription_query: Arc<dyn PushSubscriptionQueryPort>,
    pub reminder_sender: Option<Arc<dyn ReminderSenderPort>>,
    pub server_config: ServerConfig,
    pub entry_config: EntryConfig,
    pub auth_config: AuthConfig,
    pub push_config: PushConfig,
    pub preset_config: PresetConfig,
}
