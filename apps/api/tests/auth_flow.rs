//! Auth HTTP integration tests (requires DATABASE_URL + running Postgres).

use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use customer_ops_api::config::Config;
use customer_ops_api::http::build_router;
use customer_ops_api::seed;
use customer_ops_api::state::AppState;
use http_body_util::BodyExt;
use tower::ServiceExt;

fn test_config(database_url: &str) -> Config {
    Config {
        app_env: "test".to_owned(),
        api_host: "127.0.0.1".to_owned(),
        api_port: 8080,
        database_url: database_url.to_owned(),
        frontend_origin: "http://127.0.0.1:5173".to_owned(),
        session_secret: "test-secret-at-least-thirty-two-chars!!".to_owned(),
        session_ttl: Duration::from_secs(3600),
        login_rate_limit_max: 20,
        login_rate_limit_window: Duration::from_secs(900),
    }
}

async fn setup() -> Option<(axum::Router, Config)> {
    let database_url = std::env::var("DATABASE_URL").ok()?;
    if database_url.is_empty() {
        return None;
    }
    let config = test_config(&database_url);
    let state = AppState::connect(config.clone()).await.ok()?;
    seed::ensure_dev_users(&state).await.ok()?;
    Some((build_router(state), config))
}

fn origin_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ORIGIN,
        header::HeaderValue::from_static("http://127.0.0.1:5173"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers
}

#[tokio::test]
async fn me_unauthorized_without_cookie() {
    let Some((app, _)) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_logout_me_flow() {
    let Some((app, _)) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/login")
        .body(Body::from(
            r#"{"usernameOrEmail":"admin","password":"VisionRouteDemo26!"}"#,
        ))
        .unwrap();
    *req.headers_mut() = origin_headers();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let set_cookie = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|v| v.starts_with("vr_ops_session="))
        .expect("session cookie")
        .to_owned();
    let token_pair = set_cookie.split(';').next().unwrap();

    let me = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/me")
                .header(header::COOKIE, token_pair)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(me.status(), StatusCode::OK);
    let body = me.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["username"], "admin");
    assert_eq!(json["role"], "admin");

    let mut logout = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/logout")
        .header(header::COOKIE, token_pair)
        .body(Body::empty())
        .unwrap();
    *logout.headers_mut() = {
        let mut h = origin_headers();
        h.insert(
            header::COOKIE,
            header::HeaderValue::from_str(token_pair).unwrap(),
        );
        h
    };

    let logout_res = app.clone().oneshot(logout).await.unwrap();
    assert_eq!(logout_res.status(), StatusCode::OK);

    let me_after = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/me")
                .header(header::COOKIE, token_pair)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(me_after.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_rejects_bad_password_generically() {
    let Some((app, _)) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/login")
        .body(Body::from(
            r#"{"usernameOrEmail":"admin","password":"definitely-wrong-password"}"#,
        ))
        .unwrap();
    *req.headers_mut() = origin_headers();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"]["message"], "Invalid credentials.");
}

#[tokio::test]
async fn login_rejects_missing_origin() {
    let Some((app, _)) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"usernameOrEmail":"admin","password":"VisionRouteDemo26!"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
