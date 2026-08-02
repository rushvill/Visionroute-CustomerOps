use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::{TicketCategory, TicketPriority, TicketStatus};

#[derive(Debug, Clone, FromRow)]
pub struct TicketRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub number: String,
    pub created_by_user_id: Uuid,
    pub assigned_to_user_id: Option<Uuid>,
    pub device_id: Option<Uuid>,
    pub sim_card_id: Option<Uuid>,
    pub subject: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub category: TicketCategory,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateTicketInput {
    pub subject: String,
    pub description: Option<String>,
    pub category: Option<TicketCategory>,
    pub priority: Option<TicketPriority>,
    pub device_id: Option<Uuid>,
    pub sim_card_id: Option<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTicketInput {
    pub status: Option<TicketStatus>,
    pub priority: Option<TicketPriority>,
    pub assigned_to_user_id: Option<Option<Uuid>>,
}

pub async fn next_ticket_number(pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    let seq: i64 = sqlx::query_scalar("SELECT nextval('ticket_number_seq')")
        .fetch_one(pool)
        .await?;
    Ok(format!("TKT-{seq:06}"))
}

pub async fn create(
    pool: &sqlx::PgPool,
    account_id: Uuid,
    created_by: Uuid,
    input: &CreateTicketInput,
) -> Result<TicketRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let number = next_ticket_number(pool).await?;
    let priority = input.priority.unwrap_or(TicketPriority::P2);
    let category = input.category.unwrap_or(TicketCategory::Other);

    sqlx::query_as::<_, TicketRow>(
        r#"
        INSERT INTO tickets (
            id, account_id, number, created_by_user_id, device_id, sim_card_id,
            subject, description, status, priority, category, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'open', $9, $10, now(), now())
        RETURNING id, account_id, number, created_by_user_id, assigned_to_user_id,
                  device_id, sim_card_id, subject, description, status, priority, category,
                  resolved_at, closed_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(account_id)
    .bind(&number)
    .bind(created_by)
    .bind(input.device_id)
    .bind(input.sim_card_id)
    .bind(&input.subject)
    .bind(&input.description)
    .bind(priority)
    .bind(category)
    .fetch_one(pool)
    .await
}

pub async fn list_by_account(
    pool: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<Vec<TicketRow>, sqlx::Error> {
    sqlx::query_as::<_, TicketRow>(
        r#"
        SELECT id, account_id, number, created_by_user_id, assigned_to_user_id,
               device_id, sim_card_id, subject, description, status, priority, category,
               resolved_at, closed_at, created_at, updated_at
        FROM tickets
        WHERE account_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
}

pub async fn list_all(pool: &sqlx::PgPool) -> Result<Vec<TicketRow>, sqlx::Error> {
    sqlx::query_as::<_, TicketRow>(
        r#"
        SELECT id, account_id, number, created_by_user_id, assigned_to_user_id,
               device_id, sim_card_id, subject, description, status, priority, category,
               resolved_at, closed_at, created_at, updated_at
        FROM tickets
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<TicketRow>, sqlx::Error> {
    sqlx::query_as::<_, TicketRow>(
        r#"
        SELECT id, account_id, number, created_by_user_id, assigned_to_user_id,
               device_id, sim_card_id, subject, description, status, priority, category,
               resolved_at, closed_at, created_at, updated_at
        FROM tickets
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn update(
    pool: &sqlx::PgPool,
    id: Uuid,
    input: &UpdateTicketInput,
) -> Result<TicketRow, sqlx::Error> {
    let existing = get_by_id(pool, id).await?.ok_or(sqlx::Error::RowNotFound)?;

    let status = input.status.unwrap_or(existing.status);
    let priority = input.priority.unwrap_or(existing.priority);
    let assigned = input
        .assigned_to_user_id
        .unwrap_or(existing.assigned_to_user_id);

    let resolved_at = match status {
        TicketStatus::Resolved | TicketStatus::Closed => Some(Utc::now()),
        _ => existing.resolved_at,
    };
    let closed_at = match status {
        TicketStatus::Closed => Some(Utc::now()),
        _ => existing.closed_at,
    };

    sqlx::query_as::<_, TicketRow>(
        r#"
        UPDATE tickets
        SET status = $2,
            priority = $3,
            assigned_to_user_id = $4,
            resolved_at = $5,
            closed_at = $6,
            updated_at = now()
        WHERE id = $1
        RETURNING id, account_id, number, created_by_user_id, assigned_to_user_id,
                  device_id, sim_card_id, subject, description, status, priority, category,
                  resolved_at, closed_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(status)
    .bind(priority)
    .bind(assigned)
    .bind(resolved_at)
    .bind(closed_at)
    .fetch_one(pool)
    .await
}
