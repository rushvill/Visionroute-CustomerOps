//! Shared application state for request handlers.

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

use crate::auth::rate_limit::LoginRateLimiter;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    pub login_limiter: LoginRateLimiter,
}

impl AppState {
    pub async fn connect(config: Config) -> anyhow::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&config.database_url)
            .await?;

        sqlx::migrate!("./migrations").run(&db).await?;
        info!("database migrations applied");

        Ok(Self {
            login_limiter: LoginRateLimiter::new(
                config.login_rate_limit_max,
                config.login_rate_limit_window,
            ),
            config,
            db,
        })
    }
}
