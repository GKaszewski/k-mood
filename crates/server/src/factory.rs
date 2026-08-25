use std::sync::Arc;

use config::AppConfig;
use domain::ports::ReminderSenderPort;
use http_axum::state::AppState;

pub struct AppContext {
    pub state: AppState,
    pub reminder_sender: Option<Arc<dyn ReminderSenderPort>>,
}

pub async fn build(config: AppConfig) -> Result<AppContext, Box<dyn std::error::Error>> {
    let db_url = config
        .storage
        .database_url
        .clone()
        .unwrap_or_else(|| format!("sqlite://{}/k-mood.db", config.storage.data_dir));

    let pool = sqlite::create_pool(&db_url).await?;
    sqlite::run_migrations(&pool).await?;

    let user_command = Arc::new(sqlite::repositories::SqliteUserCommandRepository::new(
        pool.clone(),
    ));
    let user_query = Arc::new(sqlite::repositories::SqliteUserQueryRepository::new(
        pool.clone(),
    ));
    let activity_command = Arc::new(sqlite::repositories::SqliteActivityCommandRepository::new(
        pool.clone(),
    ));
    let activity_query = Arc::new(sqlite::repositories::SqliteActivityQueryRepository::new(
        pool.clone(),
    ));
    let entry_command = Arc::new(sqlite::repositories::SqliteEntryCommandRepository::new(
        pool.clone(),
    ));
    let entry_query = Arc::new(sqlite::repositories::SqliteEntryQueryRepository::new(
        pool.clone(),
    ));
    let reminder_command = Arc::new(sqlite::repositories::SqliteReminderCommandRepository::new(
        pool.clone(),
    ));
    let reminder_query = Arc::new(sqlite::repositories::SqliteReminderQueryRepository::new(
        pool.clone(),
    ));
    let refresh_session_command =
        Arc::new(sqlite::repositories::SqliteRefreshSessionCommandRepository::new(pool.clone()));
    let refresh_session_query =
        Arc::new(sqlite::repositories::SqliteRefreshSessionQueryRepository::new(pool.clone()));
    let cascade = Arc::new(sqlite::repositories::SqliteCascadeDeleteRepository::new(
        pool.clone(),
    ));
    let push_subscription_command =
        Arc::new(sqlite::repositories::SqlitePushSubscriptionCommandRepository::new(pool.clone()));
    let push_subscription_query =
        Arc::new(sqlite::repositories::SqlitePushSubscriptionQueryRepository::new(pool));

    let jwt_service = auth::JwtAuthService::new(&config.auth)?;
    let password_hasher = auth::Argon2PasswordHasher;

    let media_storage = storage::create_media_storage(&config.storage)?;

    let reminder_sender: Option<Arc<dyn ReminderSenderPort>> = if config.push.enabled {
        let sender =
            web_push_adapter::WebPushSender::new(&config.push, push_subscription_query.clone())?;
        Some(Arc::new(sender))
    } else {
        None
    };

    let (event_publisher, event_receiver) = event_publisher::create_event_channel();
    spawn_event_drain(event_receiver);

    let state = AppState {
        entry_command,
        entry_query,
        activity_command,
        activity_query,
        user_command,
        user_query,
        reminder_command,
        reminder_query,
        refresh_session_command,
        refresh_session_query,
        cascade,
        auth_service: Arc::new(jwt_service),
        password_hasher: Arc::new(password_hasher),
        event_publisher: Arc::new(event_publisher),
        media_storage: Arc::new(media_storage),
        export_port: Arc::new(exporter::JsonExportAdapter),
        import_source: Arc::new(importer::DaylioImportAdapter),
        push_subscription_command,
        push_subscription_query,
        reminder_sender: reminder_sender.clone(),
        server_config: config.server,
        entry_config: config.entry,
        auth_config: config.auth,
        push_config: config.push,
        preset_config: config.preset,
    };

    Ok(AppContext {
        state,
        reminder_sender,
    })
}

fn spawn_event_drain(mut receiver: event_publisher::EventReceiver) {
    tokio::spawn(async move {
        while let Some(envelope) = receiver.recv().await {
            tracing::debug!(
                event_id = %envelope.id(),
                event = ?envelope.event(),
                "domain event published"
            );
        }
        tracing::info!("event drain stopped");
    });
}
