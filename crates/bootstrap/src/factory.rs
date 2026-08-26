use std::sync::Arc;

use config::AppConfig;
use domain::ports::ReminderSenderPort;
use http_axum::state::AppState;

pub struct Context {
    pub state: AppState,
    pub reminder_sender: Option<Arc<dyn ReminderSenderPort>>,
    pub job_queue: Arc<dyn domain::ports::JobQueueCommandPort>,
    pub job_view: Arc<dyn domain::ports::JobQueueQueryPort>,
    pub recording_backlog: Arc<dyn domain::ports::RecordingBackfillQueryPort>,
    pub recordings: Arc<dyn domain::ports::RecordingLookupPort>,
    pub weather_backlog: Arc<dyn domain::ports::WeatherBacklogQueryPort>,
    pub weather_store: Arc<dyn domain::ports::EntryDimensionPort>,
    pub weather: Option<Arc<dyn domain::ports::WeatherLookupPort>>,
    pub worker_config: config::WorkerConfig,
}

pub async fn build(config: AppConfig) -> Result<Context, Box<dyn std::error::Error>> {
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
    let weather_store = Arc::new(sqlite::repositories::SqliteWeatherDimensionRepository::new(
        pool.clone(),
    ));

    let dimensions: Vec<Arc<dyn domain::ports::EntryDimensionPort>> = vec![
        Arc::new(sqlite::repositories::SqliteContentDimensionRepository::new(
            pool.clone(),
        )),
        Arc::new(sqlite::repositories::SqliteRelationDimensionRepository::activities(pool.clone())),
        Arc::new(sqlite::repositories::SqliteRelationDimensionRepository::photos(pool.clone())),
        Arc::new(
            sqlite::repositories::SqliteRelationDimensionRepository::voice_memos(pool.clone()),
        ),
        Arc::new(sqlite::repositories::SqliteLocationDimensionRepository::new(pool.clone())),
        Arc::new(sqlite::repositories::SqliteSongDimensionRepository::new(
            pool.clone(),
        )),
        weather_store.clone(),
    ];
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
    let job_queue = Arc::new(sqlite::repositories::SqliteJobQueueRepository::new(
        pool.clone(),
    ));
    let recording_backlog =
        Arc::new(sqlite::repositories::SqliteRecordingBackfillRepository::new(pool.clone()));
    let weather_backlog = Arc::new(sqlite::repositories::SqliteWeatherBacklogRepository::new(
        pool.clone(),
    ));
    let cycle_repository = Arc::new(sqlite::repositories::SqliteCycleStartRepository::new(
        pool.clone(),
    ));
    let preferences_repository = Arc::new(
        sqlite::repositories::SqliteUserPreferencesRepository::new(pool.clone()),
    );
    let api_token_command = Arc::new(sqlite::repositories::SqliteApiTokenCommandRepository::new(
        pool.clone(),
    ));
    let api_token_query = Arc::new(sqlite::repositories::SqliteApiTokenQueryRepository::new(
        pool.clone(),
    ));
    let daily_metric_command =
        Arc::new(sqlite::repositories::SqliteDailyMetricCommandRepository::new(pool.clone()));
    let rejections = Arc::new(sqlite::repositories::SqliteRejectionRepository::new(
        pool.clone(),
        config.import.rejections_kept,
    ));
    let daily_metric_query = Arc::new(sqlite::repositories::SqliteDailyMetricQueryRepository::new(
        pool.clone(),
        rejections.clone(),
    ));
    let push_subscription_command =
        Arc::new(sqlite::repositories::SqlitePushSubscriptionCommandRepository::new(pool.clone()));
    let push_subscription_query =
        Arc::new(sqlite::repositories::SqlitePushSubscriptionQueryRepository::new(pool.clone()));

    let jwt_service = auth::JwtAuthService::new(&config.auth)?;
    let password_hasher = auth::Argon2PasswordHasher;

    let media_storage = storage::create_media_storage(&config.storage)?;

    let reminder_sender: Option<Arc<dyn ReminderSenderPort>> = if config.push.enabled {
        let sender = web_push_adapter::WebPushSender::new(
            &config.push,
            push_subscription_query.clone(),
            push_subscription_command.clone(),
        )?;
        Some(Arc::new(sender))
    } else {
        None
    };

    let provider_connection_command = Arc::new(
        sqlite::repositories::SqliteProviderConnectionCommandRepository::new(pool.clone()),
    );
    let provider_connection_query =
        Arc::new(sqlite::repositories::SqliteProviderConnectionQueryRepository::new(pool.clone()));

    let credential_cipher = match config.provider.encryption_key.as_deref() {
        Some(key) if !key.is_empty() => {
            let cipher = crypto::ChaChaCredentialCipher::new(key)?;
            tracing::info!("provider connections enabled");
            Some(Arc::new(cipher) as Arc<dyn domain::provider::CredentialCipher>)
        }
        _ => {
            tracing::info!(
                "no credential encryption key configured, provider connections are unavailable"
            );
            None
        }
    };

    let music_http = reqwest::Client::builder()
        .user_agent(music::musicbrainz::USER_AGENT)
        .build()?;

    let now_playing: Option<Arc<dyn domain::ports::NowPlayingPort>> = Some(Arc::new(
        music::SubsonicNowPlayingAdapter::new(music_http.clone()),
    ));
    let recording_lookup: Arc<dyn domain::ports::RecordingLookupPort> =
        Arc::new(music::MusicBrainzRecordingLookup::new(music_http));

    let weather_lookup: Option<Arc<dyn domain::ports::WeatherLookupPort>> = if config
        .worker
        .look_up_weather
    {
        Some(Arc::new(weather::OpenMeteoWeatherLookup::new(
            reqwest::Client::new(),
        )))
    } else {
        tracing::info!("weather lookups are switched off, so no coordinates leave this machine");
        None
    };

    let (event_publisher, event_receiver) = event_publisher::create_event_channel();
    spawn_event_drain(event_receiver);

    let state = AppState {
        entry_command,
        entry_query,
        dimensions,
        weather_store: weather_store.clone(),
        provider_connection_command,
        provider_connection_query,
        credential_cipher,
        now_playing,
        recording_lookup: recording_lookup.clone(),
        activity_command,
        activity_query,
        user_command,
        user_query,
        reminder_command,
        reminder_query,
        refresh_session_command,
        refresh_session_query,
        cascade,
        daily_metric_command,
        daily_metric_query,
        cycle_command: cycle_repository.clone(),
        cycle_query: cycle_repository,
        preferences_command: preferences_repository.clone(),
        preferences_query: preferences_repository,
        rejection_command: rejections.clone(),
        rejection_query: rejections,
        api_token_command,
        api_token_query,
        api_token_secrets: Arc::new(auth::ApiTokenSecret),
        auth_service: Arc::new(jwt_service),
        password_hasher: Arc::new(password_hasher),
        event_publisher: Arc::new(event_publisher),
        media_storage: Arc::new(media_storage),
        backup_writer: Arc::new(exporter::ZipBackupWriter),
        backup_reader: Arc::new(importer::KmoodBackupAdapter),
        extract_writer: Arc::new(exporter::MarkdownExtractWriter),
        import_source: Arc::new(importer::DaylioImportAdapter),
        push_subscription_command,
        push_subscription_query,
        reminder_sender: reminder_sender.clone(),
        server_config: config.server,
        entry_config: config.entry,
        analysis_config: config.analysis,
        import_config: config.import,
        auth_config: config.auth,
        push_config: config.push,
        preset_config: config.preset,
    };

    Ok(Context {
        state,
        reminder_sender,
        job_queue: job_queue.clone(),
        job_view: job_queue,
        recording_backlog,
        recordings: recording_lookup,
        weather_backlog,
        weather_store: weather_store.clone(),
        weather: weather_lookup,
        worker_config: config.worker,
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
