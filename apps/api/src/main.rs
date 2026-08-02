//! VisionRoute Customer Ops API — Phase 2 authentication.

#![forbid(unsafe_code)]

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;

use customer_ops_api::config::Config;
use customer_ops_api::http::build_router;
use customer_ops_api::seed;
use customer_ops_api::state::AppState;
use customer_ops_api::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init();

    let config = Config::from_env()?;
    config.validate_for_runtime()?;

    let state = AppState::connect(config.clone()).await?;
    seed::ensure_bootstrap_admin(&state).await?;
    seed::ensure_dev_users(&state).await?;

    let app = build_router(state);
    let addr = SocketAddr::from((config.bind_ip(), config.api_port));
    let listener = TcpListener::bind(addr).await?;

    info!(
        %addr,
        env = %config.app_env,
        "customer-ops API listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if tokio::signal::ctrl_c().await.is_err() {
            tracing::error!("failed to install Ctrl+C handler");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(error) => {
                tracing::error!(%error, "failed to install SIGTERM handler");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received");
}
