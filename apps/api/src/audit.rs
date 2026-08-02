//! Audit event helpers.

use serde_json::Value;
use uuid::Uuid;

use crate::error::AppError;
use crate::state::AppState;

#[allow(clippy::too_many_arguments)]
pub async fn record(
    state: &AppState,
    actor_user_id: Option<Uuid>,
    account_id: Option<Uuid>,
    entity_type: &str,
    entity_id: Uuid,
    action: &str,
    summary: &str,
    metadata: Value,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO audit_events (
            id, account_id, actor_user_id, entity_type, entity_id, action, summary, metadata, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, now())
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(account_id)
    .bind(actor_user_id)
    .bind(entity_type)
    .bind(entity_id)
    .bind(action)
    .bind(summary)
    .bind(metadata)
    .execute(&state.db)
    .await
    .map_err(|error| {
        tracing::error!(%error, "audit insert failed");
        AppError::Internal(anyhow::anyhow!("audit insert failed"))
    })?;
    Ok(())
}
