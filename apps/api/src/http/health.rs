use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct LiveResponse {
    pub ok: bool,
    pub service: &'static str,
}

#[derive(Serialize)]
pub struct ReadyResponse {
    pub ok: bool,
    pub database: DatabaseStatus,
}

#[derive(Serialize)]
pub struct DatabaseStatus {
    pub ok: bool,
    pub connected: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiHealthResponse {
    pub ok: bool,
    pub service: &'static str,
    pub version: &'static str,
    pub app_env: String,
}

/// Liveness — process is up (no dependency checks).
pub async fn live() -> Json<LiveResponse> {
    Json(LiveResponse {
        ok: true,
        service: "customer-ops-api",
    })
}

/// Readiness — PostgreSQL reachable via shared pool.
pub async fn ready(State(state): State<AppState>) -> Result<Json<ReadyResponse>, AppError> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map_err(|error| {
            tracing::warn!(%error, "database ping failed");
            AppError::Unavailable
        })?;

    Ok(Json(ReadyResponse {
        ok: true,
        database: DatabaseStatus {
            ok: true,
            connected: true,
        },
    }))
}

pub async fn api_health(State(state): State<AppState>) -> Json<ApiHealthResponse> {
    Json(ApiHealthResponse {
        ok: true,
        service: "customer-ops-api",
        version: env!("CARGO_PKG_VERSION"),
        app_env: state.config.app_env.clone(),
    })
}
