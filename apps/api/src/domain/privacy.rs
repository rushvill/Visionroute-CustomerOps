//! Privacy / data-subject request handling (small-business DSAR inbox).

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::{PrivacyRequestStatus, PrivacyRequestType};

pub const PRIVACY_NOTICE_VERSION: &str = "2026-08-02";

#[derive(Debug, Clone, FromRow)]
pub struct PrivacyRequestRow {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub requester_name: Option<String>,
    pub requester_email: String,
    pub request_type: PrivacyRequestType,
    pub details: Option<String>,
    pub status: PrivacyRequestStatus,
    pub handled_by: Option<Uuid>,
    pub handled_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreatePrivacyRequestInput {
    pub account_id: Option<Uuid>,
    pub requester_name: Option<String>,
    pub requester_email: String,
    pub request_type: PrivacyRequestType,
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdatePrivacyRequestInput {
    pub status: PrivacyRequestStatus,
    pub resolution_notes: Option<String>,
}

pub async fn create_request(
    pool: &sqlx::PgPool,
    input: &CreatePrivacyRequestInput,
) -> Result<PrivacyRequestRow, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query_as::<_, PrivacyRequestRow>(
        r#"
        INSERT INTO privacy_requests (
            id, account_id, requester_name, requester_email, request_type, details,
            status, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, 'received', now(), now())
        RETURNING id, account_id, requester_name, requester_email, request_type, details,
                  status, handled_by, handled_at, resolution_notes, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(input.account_id)
    .bind(&input.requester_name)
    .bind(&input.requester_email)
    .bind(input.request_type)
    .bind(&input.details)
    .fetch_one(pool)
    .await
}

pub async fn list_requests(pool: &sqlx::PgPool) -> Result<Vec<PrivacyRequestRow>, sqlx::Error> {
    sqlx::query_as::<_, PrivacyRequestRow>(
        r#"
        SELECT id, account_id, requester_name, requester_email, request_type, details,
               status, handled_by, handled_at, resolution_notes, created_at, updated_at
        FROM privacy_requests
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn update_request(
    pool: &sqlx::PgPool,
    id: Uuid,
    handled_by: Uuid,
    input: &UpdatePrivacyRequestInput,
) -> Result<PrivacyRequestRow, sqlx::Error> {
    sqlx::query_as::<_, PrivacyRequestRow>(
        r#"
        UPDATE privacy_requests
        SET status = $2,
            resolution_notes = $3,
            handled_by = $4,
            handled_at = now(),
            updated_at = now()
        WHERE id = $1
        RETURNING id, account_id, requester_name, requester_email, request_type, details,
                  status, handled_by, handled_at, resolution_notes, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(input.status)
    .bind(&input.resolution_notes)
    .bind(handled_by)
    .fetch_one(pool)
    .await
}
