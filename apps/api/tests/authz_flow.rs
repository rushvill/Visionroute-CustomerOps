//! Authorization integration tests (requires DATABASE_URL).

use std::time::Duration;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use customer_ops_api::config::Config;
use customer_ops_api::http::build_router;
use customer_ops_api::seed::{self, DEV_CUSTOMER_ACCOUNT_ID};
use customer_ops_api::state::AppState;
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

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
async fn admin_can_list_users_customer_cannot() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let admin_cookie = login_cookie(&app, "admin", "VisionRouteDemo26!").await;
    let admin_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/users")
                .header(header::COOKIE, &admin_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(admin_res.status(), StatusCode::OK);

    let customer_cookie = login_cookie(&app, "customer", "VisionRouteDemo26!").await;
    let customer_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/users")
                .header(header::COOKIE, &customer_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(customer_res.status(), StatusCode::FORBIDDEN);

    let body = customer_res.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"]["code"], "FORBIDDEN");
}

#[tokio::test]
async fn customer_denied_other_account_scope() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let cookie = login_cookie(&app, "customer", "VisionRouteDemo26!").await;
    let other = Uuid::new_v4();

    let denied = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/accounts/{other}/access"))
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(denied.status(), StatusCode::FORBIDDEN);

    let allowed = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/accounts/{DEV_CUSTOMER_ACCOUNT_ID}/access"))
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(allowed.status(), StatusCode::OK);
}

#[tokio::test]
async fn anonymous_cannot_hit_admin() {
    let Some(app) = setup().await else {
        eprintln!("skipping: DATABASE_URL not set");
        return;
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/admin/audit-events")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
