//! Authentication service: login, logout, session resolve, audit.

use chrono::{Duration as ChronoDuration, Utc};
use serde_json::json;
use uuid::Uuid;

use crate::auth::password::verify_password;
use crate::auth::session::{hash_client_hint, hash_session_token, mint_session_token};
use crate::error::AppError;
use crate::state::AppState;
use crate::users::{self, UserPublic, UserRow};

pub struct AuthService;

pub struct LoginOutcome {
    pub user: UserPublic,
    pub session_token: String,
}

impl AuthService {
    pub async fn login(
        state: &AppState,
        username_or_email: &str,
        password: &str,
        client_ip: Option<&str>,
        user_agent: Option<&str>,
        rate_key: &str,
    ) -> Result<LoginOutcome, AppError> {
        let identifier = username_or_email.trim();
        if identifier.is_empty() || password.is_empty() {
            return Err(AppError::Validation(
                "Username/email and password are required.".to_owned(),
            ));
        }

        if !state.login_limiter.check_allowed(rate_key) {
            tracing::warn!("login rate limit exceeded");
            return Err(AppError::RateLimited);
        }

        let user = users::find_by_username_or_email(&state.db, identifier)
            .await
            .map_err(|error| {
                tracing::error!(%error, "user lookup failed");
                AppError::Internal(anyhow::anyhow!("user lookup failed"))
            })?;

        let Some(user) = user else {
            state.login_limiter.record_failure(rate_key);
            Self::audit_anonymous(
                state,
                "login_failure",
                "Login failed: unknown identity",
                json!({ "reason": "unknown_identity" }),
            )
            .await?;
            // Uniform failure — do not reveal whether the user exists.
            return Err(AppError::InvalidCredentials);
        };

        if !user.is_active {
            state.login_limiter.record_failure(rate_key);
            Self::audit(
                state,
                Some(user.id),
                "user",
                user.id,
                "login_failure",
                "Login failed: inactive user",
                json!({ "reason": "inactive" }),
            )
            .await?;
            return Err(AppError::InvalidCredentials);
        }

        let password_ok = verify_password(password, &user.password_hash)?;
        if !password_ok {
            state.login_limiter.record_failure(rate_key);
            Self::audit(
                state,
                Some(user.id),
                "user",
                user.id,
                "login_failure",
                "Login failed: invalid credentials",
                json!({ "reason": "bad_password" }),
            )
            .await?;
            return Err(AppError::InvalidCredentials);
        }

        state.login_limiter.clear(rate_key);

        // Session fixation prevention: always mint a fresh session token.
        let session_token = mint_session_token();
        let token_hash = hash_session_token(&session_token);
        let session_id = Uuid::new_v4();
        let expires_at = Utc::now()
            + ChronoDuration::from_std(state.config.session_ttl)
                .unwrap_or_else(|_| ChronoDuration::hours(12));

        sqlx::query(
            r#"
            INSERT INTO sessions (
                id, user_id, token_hash, created_at, expires_at, revoked_at,
                last_seen_at, ip_hash, user_agent_hash
            ) VALUES ($1, $2, $3, now(), $4, NULL, now(), $5, $6)
            "#,
        )
        .bind(session_id)
        .bind(user.id)
        .bind(&token_hash)
        .bind(expires_at)
        .bind(client_ip.map(hash_client_hint))
        .bind(user_agent.map(hash_client_hint))
        .execute(&state.db)
        .await
        .map_err(|error| {
            tracing::error!(%error, "session create failed");
            AppError::Internal(anyhow::anyhow!("session create failed"))
        })?;

        users::touch_last_login(&state.db, user.id)
            .await
            .map_err(|error| {
                tracing::error!(%error, "last_login update failed");
                AppError::Internal(anyhow::anyhow!("last_login update failed"))
            })?;

        Self::audit(
            state,
            Some(user.id),
            "session",
            session_id,
            "login_success",
            "User logged in",
            json!({ "user_id": user.id }),
        )
        .await?;

        Ok(LoginOutcome {
            user: UserPublic::from(&user),
            session_token,
        })
    }

    pub async fn logout(state: &AppState, session_token: &str) -> Result<(), AppError> {
        let token_hash = hash_session_token(session_token);
        let row = sqlx::query_as::<_, (Uuid, Uuid)>(
            r#"
            UPDATE sessions
            SET revoked_at = now()
            WHERE token_hash = $1
              AND revoked_at IS NULL
              AND expires_at > now()
            RETURNING id, user_id
            "#,
        )
        .bind(&token_hash)
        .fetch_optional(&state.db)
        .await
        .map_err(|error| {
            tracing::error!(%error, "session revoke failed");
            AppError::Internal(anyhow::anyhow!("session revoke failed"))
        })?;

        if let Some((session_id, user_id)) = row {
            Self::audit(
                state,
                Some(user_id),
                "session",
                session_id,
                "logout",
                "User logged out",
                json!({}),
            )
            .await?;
        }

        Ok(())
    }

    pub async fn resolve_user(state: &AppState, session_token: &str) -> Result<UserRow, AppError> {
        let token_hash = hash_session_token(session_token);
        let row = sqlx::query_as::<_, (Uuid, Uuid)>(
            r#"
            SELECT s.id, s.user_id
            FROM sessions s
            WHERE s.token_hash = $1
              AND s.revoked_at IS NULL
              AND s.expires_at > now()
            LIMIT 1
            "#,
        )
        .bind(&token_hash)
        .fetch_optional(&state.db)
        .await
        .map_err(|error| {
            tracing::error!(%error, "session lookup failed");
            AppError::Internal(anyhow::anyhow!("session lookup failed"))
        })?;

        let Some((session_id, user_id)) = row else {
            return Err(AppError::Unauthorized);
        };

        let _ = sqlx::query(
            r#"
            UPDATE sessions SET last_seen_at = now() WHERE id = $1
            "#,
        )
        .bind(session_id)
        .execute(&state.db)
        .await;

        let user = users::find_by_id(&state.db, user_id)
            .await
            .map_err(|error| {
                tracing::error!(%error, "user load failed");
                AppError::Internal(anyhow::anyhow!("user load failed"))
            })?
            .ok_or(AppError::Unauthorized)?;

        if !user.is_active {
            return Err(AppError::Unauthorized);
        }

        Ok(user)
    }

    async fn audit(
        state: &AppState,
        actor_user_id: Option<Uuid>,
        entity_type: &str,
        entity_id: Uuid,
        action: &str,
        summary: &str,
        metadata: serde_json::Value,
    ) -> Result<(), AppError> {
        crate::audit::record(
            state,
            actor_user_id,
            None,
            entity_type,
            entity_id,
            action,
            summary,
            metadata,
        )
        .await
    }

    async fn audit_anonymous(
        state: &AppState,
        action: &str,
        summary: &str,
        metadata: serde_json::Value,
    ) -> Result<(), AppError> {
        Self::audit(state, None, "auth", Uuid::nil(), action, summary, metadata).await
    }
}
