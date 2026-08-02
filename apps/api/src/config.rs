use std::net::IpAddr;
use std::time::Duration;

use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_env: String,
    pub api_host: String,
    pub api_port: u16,
    pub database_url: String,
    pub frontend_origin: String,
    pub session_secret: String,
    pub session_ttl: Duration,
    pub login_rate_limit_max: u32,
    pub login_rate_limit_window: Duration,
}

pub const SESSION_COOKIE_NAME: &str = "vr_ops_session";

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();

        let session_ttl_hours: u64 = std::env::var("SESSION_TTL_HOURS")
            .unwrap_or_else(|_| "12".to_owned())
            .parse()
            .map_err(|_| anyhow::anyhow!("SESSION_TTL_HOURS must be a positive integer"))?;

        let login_rate_limit_max: u32 = std::env::var("LOGIN_RATE_LIMIT_MAX")
            .unwrap_or_else(|_| "10".to_owned())
            .parse()
            .map_err(|_| anyhow::anyhow!("LOGIN_RATE_LIMIT_MAX must be a positive integer"))?;

        let login_rate_limit_window_secs: u64 = std::env::var("LOGIN_RATE_LIMIT_WINDOW_SECS")
            .unwrap_or_else(|_| "900".to_owned())
            .parse()
            .map_err(|_| {
                anyhow::anyhow!("LOGIN_RATE_LIMIT_WINDOW_SECS must be a positive integer")
            })?;

        // Render and many PaaS hosts inject `PORT`; prefer explicit `API_PORT` when set.
        let api_port = std::env::var("API_PORT")
            .or_else(|_| std::env::var("PORT"))
            .unwrap_or_else(|_| "8080".to_owned())
            .parse()
            .map_err(|_| anyhow::anyhow!("API_PORT/PORT must be a valid u16"))?;

        Ok(Self {
            app_env: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_owned()),
            api_host: std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            api_port,
            database_url: std::env::var("DATABASE_URL").unwrap_or_default(),
            frontend_origin: std::env::var("FRONTEND_ORIGIN")
                .unwrap_or_else(|_| "http://127.0.0.1:5173".to_owned()),
            session_secret: std::env::var("SESSION_SECRET").unwrap_or_default(),
            session_ttl: Duration::from_secs(session_ttl_hours.saturating_mul(3600).max(3600)),
            login_rate_limit_max: login_rate_limit_max.max(1),
            login_rate_limit_window: Duration::from_secs(login_rate_limit_window_secs.max(60)),
        })
    }

    pub fn is_production(&self) -> bool {
        matches!(
            self.app_env.to_ascii_lowercase().as_str(),
            "production" | "prod" | "staging"
        )
    }

    pub fn is_development(&self) -> bool {
        matches!(
            self.app_env.to_ascii_lowercase().as_str(),
            "development" | "dev" | "test"
        )
    }

    pub fn cookie_secure(&self) -> bool {
        self.is_production()
    }

    pub fn validate_for_runtime(&self) -> anyhow::Result<()> {
        if self.database_url.trim().is_empty() {
            anyhow::bail!("DATABASE_URL is required");
        }
        if !self.database_url.starts_with("postgres://")
            && !self.database_url.starts_with("postgresql://")
        {
            anyhow::bail!("DATABASE_URL must be a PostgreSQL URL");
        }

        if self.is_production() {
            if self.session_secret.trim().is_empty()
                || self.session_secret.contains("dev-only")
                || self.session_secret.len() < 32
            {
                anyhow::bail!(
                    "SESSION_SECRET must be a strong non-placeholder value in {}",
                    self.app_env
                );
            }
            if self.frontend_origin.starts_with("http://localhost")
                || self.frontend_origin.starts_with("http://127.0.0.1")
            {
                anyhow::bail!("FRONTEND_ORIGIN must not be localhost in {}", self.app_env);
            }
            if !self.frontend_origin.starts_with("https://") {
                anyhow::bail!("FRONTEND_ORIGIN must be https:// in {}", self.app_env);
            }
        }

        Ok(())
    }

    pub fn bind_ip(&self) -> IpAddr {
        self.api_host
            .parse()
            .unwrap_or_else(|_| IpAddr::from([127, 0, 0, 1]))
    }
}

#[derive(Serialize)]
pub struct PublicConfigView {
    pub app_env: String,
    pub api_port: u16,
}

impl From<&Config> for PublicConfigView {
    fn from(value: &Config) -> Self {
        Self {
            app_env: value.app_env.clone(),
            api_port: value.api_port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::time::Duration;

    fn base_config() -> Config {
        Config {
            app_env: "development".to_owned(),
            api_host: "127.0.0.1".to_owned(),
            api_port: 8080,
            database_url: "postgres://customerops:customerops@127.0.0.1:5432/customerops"
                .to_owned(),
            frontend_origin: "http://127.0.0.1:5173".to_owned(),
            session_secret: "dev-only-change-me-before-production-use-32b".to_owned(),
            session_ttl: Duration::from_secs(12 * 3600),
            login_rate_limit_max: 10,
            login_rate_limit_window: Duration::from_secs(900),
        }
    }

    #[test]
    fn development_accepts_dev_secret() {
        let config = base_config();
        assert!(config.validate_for_runtime().is_ok());
    }

    #[test]
    fn production_rejects_placeholder_secret() {
        let mut config = base_config();
        config.app_env = "production".to_owned();
        config.frontend_origin = "https://ops.example.com".to_owned();
        let err = config
            .validate_for_runtime()
            .expect_err("placeholder secret");
        assert!(err.to_string().contains("SESSION_SECRET"));
    }

    #[test]
    fn production_requires_https_frontend_origin() {
        let mut config = base_config();
        config.app_env = "production".to_owned();
        config.session_secret = "a-strong-production-session-secret-32b".to_owned();
        config.frontend_origin = "http://ops.example.com".to_owned();
        let err = config
            .validate_for_runtime()
            .expect_err("http frontend origin");
        assert!(err.to_string().contains("https://"));
    }

    #[test]
    fn requires_postgres_url() {
        let mut config = base_config();
        config.database_url = "file:./dev.db".to_owned();
        assert!(config.validate_for_runtime().is_err());
    }
}
