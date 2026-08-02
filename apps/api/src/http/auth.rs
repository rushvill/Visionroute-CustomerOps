use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::auth::csrf::assert_trusted_origin;
use crate::auth::service::AuthService;
use crate::auth::session::SessionCookie;
use crate::config::SESSION_COOKIE_NAME;
use crate::error::AppError;
use crate::http::extractors::AuthUser;
use crate::state::AppState;
use crate::users::UserPublic;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<UserPublic>), AppError> {
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    let client_ip = client_ip_from_headers(&headers);
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let rate_key = rate_limit_key(
        client_ip.as_deref().unwrap_or("unknown"),
        &body.username_or_email,
    );

    let outcome = AuthService::login(
        &state,
        &body.username_or_email,
        &body.password,
        client_ip.as_deref(),
        user_agent.as_deref(),
        &rate_key,
    )
    .await?;

    let cookie = SessionCookie::build(&outcome.session_token, &state.config);
    Ok((jar.add(cookie), Json(outcome.user)))
}

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
) -> Result<CookieJar, AppError> {
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    if let Some(token) = jar.get(SESSION_COOKIE_NAME).map(|c| c.value().to_owned()) {
        AuthService::logout(&state, &token).await?;
    }

    Ok(jar.add(SessionCookie::clear(&state.config)))
}

pub async fn me(AuthUser(user): AuthUser) -> Json<UserPublic> {
    Json(UserPublic::from(&user))
}

fn client_ip_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn rate_limit_key(ip: &str, identifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    hasher.update(b"|");
    hasher.update(identifier.trim().to_ascii_lowercase().as_bytes());
    hex::encode(hasher.finalize())
}
