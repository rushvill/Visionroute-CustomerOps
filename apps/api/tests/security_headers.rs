//! Security headers smoke test.

use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use customer_ops_api::config::Config;
use customer_ops_api::http::build_router;
use customer_ops_api::seed;
use customer_ops_api::state::AppState;
use tower::ServiceExt;

async fn setup() -> Option<axum::Router> {
    let database_url = std::env::var("DATABASE_URL").ok()?;
    if database_url.is_empty() {
        return None;
    }
    let config = Config {
        app_env: "test".to_owned(),
        api_host: "127.0.0.1".to_owned(),
        api_port: 8080,
        database_url,
        frontend_origin: "http://127.0.0.1:5173".to_owned(),
        session_secret: "test-secret-at-least-thirty-two-chars!!".to_owned(),
        session_ttl: Duration::from_secs(3600),
        login_rate_limit_max: 50,
        login_rate_limit_window: Duration::from_secs(900),
    };
    let state = AppState::connect(config).await.ok()?;
    seed::ensure_dev_users(&state).await.ok()?;
    Some(build_router(state))
}

#[tokio::test]
async fn health_sets_security_headers() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(header::X_CONTENT_TYPE_OPTIONS)
            .and_then(|v| v.to_str().ok()),
        Some("nosniff")
    );
    assert!(response
        .headers()
        .get(header::CONTENT_SECURITY_POLICY)
        .is_some());
    assert_eq!(
        response
            .headers()
            .get(header::HeaderName::from_static("x-frame-options"))
            .and_then(|v| v.to_str().ok()),
        Some("DENY")
    );
    // HSTS only in production-like secure cookie mode
    assert!(response
        .headers()
        .get(header::STRICT_TRANSPORT_SECURITY)
        .is_none());
}
