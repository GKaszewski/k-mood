mod config_loader;
mod factory;

#[tokio::main]
async fn main() {
    setup_tracing();

    if let Err(e) = run().await {
        tracing::error!(error = %e, "application failed");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_loader::load()?;

    tracing::info!(
        host = %config.server.host,
        port = %config.server.port,
        "starting k-mood server"
    );

    let context = factory::build(config.clone()).await?;

    if let Some(sender) = context.reminder_sender {
        spawn_reminder_scheduler(
            context.state.reminder_query.clone(),
            context.state.user_query.clone(),
            sender,
        );
        tracing::info!("push notifications enabled, reminder scheduler started");
    }

    spawn_session_cleanup(context.state.refresh_session_command.clone());

    let router = http_axum::router::build_router(context.state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!(addr = %addr, "listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("server shut down gracefully");
    Ok(())
}

fn spawn_reminder_scheduler(
    reminder_query: std::sync::Arc<dyn domain::ports::ReminderQueryPort>,
    user_query: std::sync::Arc<dyn domain::ports::UserQueryPort>,
    sender: std::sync::Arc<dyn domain::ports::ReminderSenderPort>,
) {
    use application::reminder::use_cases::process_due_reminders;

    tokio::spawn(async move {
        let deps = process_due_reminders::Deps {
            reminder_query,
            user_query,
            sender,
        };

        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            match process_due_reminders::execute(&deps).await {
                Ok(sent) => {
                    if sent > 0 {
                        tracing::info!(sent, "reminders sent");
                    }
                }
                Err(e) => tracing::error!(error = %e, "reminder processing failed"),
            }
        }
    });
}

fn spawn_session_cleanup(
    refresh_session_command: std::sync::Arc<dyn domain::ports::RefreshSessionCommandPort>,
) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
        loop {
            interval.tick().await;
            match refresh_session_command.delete_expired().await {
                Ok(deleted) => {
                    if deleted > 0 {
                        tracing::info!(deleted, "expired refresh sessions cleaned up");
                    }
                }
                Err(e) => tracing::error!(error = %e, "refresh session cleanup failed"),
            }
        }
    });
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("received Ctrl+C"),
        _ = terminate => tracing::info!("received SIGTERM"),
    }
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")),
        )
        .init();
}
