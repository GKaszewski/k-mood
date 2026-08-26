mod loops;

#[tokio::main]
async fn main() {
    bootstrap::setup_tracing();

    if let Err(error) = run().await {
        tracing::error!(%error, "the worker could not start");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = bootstrap::load()?;
    let context = bootstrap::build(config).await?;
    let worker = context.worker_config.clone();

    tracing::info!(
        poll_seconds = worker.poll_seconds,
        sweep_seconds = worker.sweep_seconds,
        most_attempts = worker.most_attempts,
        "k-mood worker started"
    );

    report_abandoned_work(&context).await;

    let queue = tokio::spawn(loops::work_the_queue(
        loops::QueueDependencies {
            queue: context.job_queue.clone(),
            backfill: context.recording_backlog.clone(),
            recordings: context.recordings.clone(),
            places: context.weather_backlog.clone(),
            weather: context.weather.clone(),
            weather_store: context.weather_store.clone(),
        },
        worker.clone(),
    ));

    let sweeper = tokio::spawn(loops::sweep_for_work(
        context.recording_backlog.clone(),
        context
            .weather
            .as_ref()
            .map(|_| context.weather_backlog.clone()),
        context.job_queue.clone(),
        worker.clone(),
    ));

    let sessions = tokio::spawn(loops::clear_expired_sessions(
        context.state.refresh_session_command.clone(),
        worker.clone(),
    ));

    let reminders = context.reminder_sender.clone().map(|sender| {
        tokio::spawn(loops::send_due_reminders(
            context.state.reminder_query.clone(),
            context.state.user_query.clone(),
            sender,
            worker,
        ))
    });

    if reminders.is_none() {
        tracing::info!("push notifications are not configured, so no reminders will be sent");
    }

    bootstrap::shutdown::on_signal().await;

    queue.abort();
    sweeper.abort();
    sessions.abort();
    if let Some(reminders) = reminders {
        reminders.abort();
    }

    tracing::info!("k-mood worker shut down");

    Ok(())
}

async fn report_abandoned_work(context: &bootstrap::Context) {
    match context.job_view.find_exhausted(50).await {
        Ok(abandoned) if !abandoned.is_empty() => {
            tracing::warn!(
                count = abandoned.len(),
                "jobs were given up on and are waiting to be looked at"
            );

            for job in abandoned {
                tracing::warn!(
                    job_id = %job.id(),
                    kind = job.kind().name(),
                    attempts = job.attempts(),
                    reason = job.last_error().unwrap_or("unknown"),
                    "abandoned job"
                );
            }
        }
        Ok(_) => {}
        Err(error) => tracing::error!(%error, "could not read abandoned jobs"),
    }
}
