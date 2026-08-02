//! Phase 4 domain integration tests (requires DATABASE_URL).

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
        login_rate_limit_max: 50,
        login_rate_limit_window: Duration::from_secs(900),
    }
}

async fn setup() -> Option<axum::Router> {
    let database_url = std::env::var("DATABASE_URL").ok()?;
    if database_url.is_empty() {
        return None;
    }
    let config = test_config(&database_url);
    let state = AppState::connect(config).await.ok()?;
    seed::ensure_dev_users(&state).await.ok()?;
    Some(build_router(state))
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

async fn login_cookie(app: &axum::Router, username: &str, password: &str) -> String {
    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/auth/login")
        .body(Body::from(format!(
            r#"{{"usernameOrEmail":"{username}","password":"{password}"}}"#
        )))
        .unwrap();
    *req.headers_mut() = origin_headers();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK, "login as {username}");
    response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|v| v.starts_with("vr_ops_session="))
        .expect("session cookie")
        .split(';')
        .next()
        .unwrap()
        .to_owned()
}

#[tokio::test]
async fn public_signup_create() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let email = format!("signup-{}@example.com", uuid::Uuid::new_v4());
    let body =
        format!(r#"{{"fullName":"Jane Doe","companyName":"Acme Logistics","email":"{email}","privacyAccepted":true}}"#);

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/signup-requests")
        .body(Body::from(body))
        .unwrap();
    *req.headers_mut() = origin_headers();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["status"], "new");
    assert_eq!(json["email"], email);
}

#[tokio::test]
async fn admin_approve_signup_creates_account_user() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let email = format!("approve-{}@example.com", uuid::Uuid::new_v4());
    let signup_body =
        format!(r#"{{"fullName":"Approve Me","companyName":"Approve Co","email":"{email}","privacyAccepted":true}}"#);

    let mut signup_req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/signup-requests")
        .body(Body::from(signup_body))
        .unwrap();
    *signup_req.headers_mut() = origin_headers();

    let signup_res = app.clone().oneshot(signup_req).await.unwrap();
    assert_eq!(signup_res.status(), StatusCode::OK);
    let signup_bytes = signup_res.into_body().collect().await.unwrap().to_bytes();
    let signup_json: serde_json::Value = serde_json::from_slice(&signup_bytes).unwrap();
    let signup_id = signup_json["id"].as_str().expect("signup id");

    let admin_cookie = login_cookie(&app, "admin", "VisionRouteDemo26!").await;
    let username = format!("user{}", &signup_id[..8]);
    let approve_body = format!(r#"{{"username":"{username}","password":"LongPassword1!"}}"#);

    let approve_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/admin/signup-requests/{signup_id}/approve"))
                .header(header::COOKIE, &admin_cookie)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(approve_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(approve_res.status(), StatusCode::OK);
    let approve_bytes = approve_res.into_body().collect().await.unwrap().to_bytes();
    let approve_json: serde_json::Value = serde_json::from_slice(&approve_bytes).unwrap();
    assert!(approve_json["accountId"].is_string());
    assert!(approve_json["userId"].is_string());
    assert_eq!(approve_json["signup"]["status"], "approved");
}

#[tokio::test]
async fn customer_can_list_own_devices() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let cookie = login_cookie(&app, "customer", "VisionRouteDemo26!").await;
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/me/devices")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.is_array());
    assert!(!json.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn customer_cannot_list_admin_users() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let cookie = login_cookie(&app, "customer", "VisionRouteDemo26!").await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/users")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn customer_ticket_create() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let cookie = login_cookie(&app, "customer", "VisionRouteDemo26!").await;

    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/me/tickets")
        .header(header::COOKIE, &cookie)
        .body(Body::from(
            r#"{"subject":"GPS offline","description":"Device not reporting","priority":"p3"}"#,
        ))
        .unwrap();
    *req.headers_mut() = {
        let mut h = origin_headers();
        h.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&cookie).unwrap(),
        );
        h
    };

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["subject"], "GPS offline");
    assert_eq!(json["status"], "open");
    assert!(json["number"].as_str().unwrap().starts_with("TKT-"));
}
