//! Request extractors for authenticated users.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;

use crate::auth::service::AuthService;
use crate::config::SESSION_COOKIE_NAME;
use crate::error::AppError;
use crate::state::AppState;
use crate::users::UserRow;

/// Authenticated session user (401 if missing/invalid).
#[derive(Debug, Clone)]
pub struct AuthUser(pub UserRow);

impl AuthUser {
    pub fn inner(&self) -> &UserRow {
        &self.0
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get(SESSION_COOKIE_NAME)
            .map(|c| c.value().to_owned())
            .ok_or(AppError::Unauthorized)?;

        let user = AuthService::resolve_user(state, &token).await?;
        Ok(AuthUser(user))
    }
}
