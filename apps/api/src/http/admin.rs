//! Admin-only handlers (permission-gated).

use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::authz::{assert_account_access, require_permission, Permission};
use crate::error::AppError;
use crate::http::extractors::AuthUser;
use crate::state::AppState;
use crate::users::{self, UserPublic};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventPublic {
    pub id: Uuid,
    pub actor_user_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: String,
    pub summary: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct AuditEventRow {
    id: Uuid,
    actor_user_id: Option<Uuid>,
    entity_type: String,
    entity_id: Uuid,
    action: String,
    summary: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AuditListQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    50
}

/// GET /api/v1/admin/users — UsersManage
pub async fn list_users(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<UserPublic>>, AppError> {
    require_permission(&state, &user, Permission::UsersManage).await?;

    let rows = users::list_users(&state.db).await.map_err(|error| {
        tracing::error!(%error, "list users failed");
        AppError::Internal(anyhow::anyhow!("list users failed"))
    })?;

    Ok(Json(rows.iter().map(UserPublic::from).collect()))
}

/// GET /api/v1/admin/audit-events — AuditRead
pub async fn list_audit_events(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Query(query): Query<AuditListQuery>,
) -> Result<Json<Vec<AuditEventPublic>>, AppError> {
    require_permission(&state, &user, Permission::AuditRead).await?;

    let limit = query.limit.clamp(1, 200);
    let rows = sqlx::query_as::<_, AuditEventRow>(
        r#"
        SELECT id, actor_user_id, entity_type, entity_id, action, summary, created_at
        FROM audit_events
        ORDER BY created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .map_err(|error| {
        tracing::error!(%error, "list audit events failed");
        AppError::Internal(anyhow::anyhow!("list audit events failed"))
    })?;

    Ok(Json(
        rows.into_iter()
            .map(|row| AuditEventPublic {
                id: row.id,
                actor_user_id: row.actor_user_id,
                entity_type: row.entity_type,
                entity_id: row.entity_id,
                action: row.action,
                summary: row.summary,
                created_at: row.created_at,
            })
            .collect(),
    ))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAccessProbe {
    pub account_id: Uuid,
    pub allowed: bool,
}

/// GET /api/v1/accounts/{account_id}/access — object-scope probe for Phase 3 tests / future CRM.
pub async fn account_access(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(account_id): Path<Uuid>,
) -> Result<Json<AccountAccessProbe>, AppError> {
    // Must be signed in; customers need AccountReadOwn, staff need AccountReadAny.
    if matches!(
        user.role,
        crate::users::UserRole::Admin | crate::users::UserRole::Operator
    ) {
        require_permission(&state, &user, Permission::AccountReadAny).await?;
    } else {
        require_permission(&state, &user, Permission::AccountReadOwn).await?;
    }

    assert_account_access(&state, &user, account_id).await?;

    Ok(Json(AccountAccessProbe {
        account_id,
        allowed: true,
    }))
}
