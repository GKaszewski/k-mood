#[tokio::main]
async fn main() {
    bootstrap::setup_tracing();

    if let Err(error) = run().await {
        tracing::error!(%error, "the server could not start");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = bootstrap::load()?;

    tracing::info!(
        host = %config.server.host,
        port = %config.server.port,
        "starting k-mood server"
    );

    let context = bootstrap::build(config.clone()).await?;
    let router = http_axum::router::build_router(context.state);

    let address = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!(
        address = %address,
        "listening; background work runs in the k-mood-worker process"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(bootstrap::shutdown::on_signal())
        .await?;

    tracing::info!("k-mood server shut down gracefully");

    Ok(())
}
