use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::{SimCarrier, SimStatus};

#[derive(Debug, Clone, FromRow)]
pub struct SimRow {
    pub id: Uuid,
    pub carrier: SimCarrier,
    pub iccid: Option<String>,
    pub msisdn: Option<String>,
    pub sim_label: Option<String>,
    pub status: SimStatus,
    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost_cents: Option<i32>,
    pub data_plan_label: Option<String>,
    pub account_id: Option<Uuid>,
    pub device_id: Option<Uuid>,
    pub activated_at: Option<DateTime<Utc>>,
    pub last_status_check_at: Option<DateTime<Utc>>,
    pub data_exhausted_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateSimInput {
    pub carrier: Option<SimCarrier>,
    pub iccid: Option<String>,
    pub msisdn: Option<String>,
    pub sim_label: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost_cents: Option<i32>,
    pub data_plan_label: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AssignSimInput {
    pub account_id: Uuid,
    pub device_id: Option<Uuid>,
}

pub async fn create_inventory(
    pool: &sqlx::PgPool,
    input: &CreateSimInput,
) -> Result<SimRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let carrier = input.carrier.unwrap_or(SimCarrier::Smart);

    sqlx::query_as::<_, SimRow>(
        r#"
        INSERT INTO sim_cards (
            id, carrier, iccid, msisdn, sim_label, status, purchase_date,
            purchase_cost_cents, data_plan_label, notes, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, 'inventory', $6, $7, $8, $9, now(), now())
        RETURNING id, carrier, iccid, msisdn, sim_label, status, purchase_date,
                  purchase_cost_cents, data_plan_label, account_id, device_id,
                  activated_at, last_status_check_at, data_exhausted_at, notes,
                  created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(carrier)
    .bind(&input.iccid)
    .bind(&input.msisdn)
    .bind(&input.sim_label)
    .bind(input.purchase_date)
    .bind(input.purchase_cost_cents)
    .bind(&input.data_plan_label)
    .bind(&input.notes)
    .fetch_one(pool)
    .await
}

pub async fn list_all(
    pool: &sqlx::PgPool,
    status: Option<SimStatus>,
) -> Result<Vec<SimRow>, sqlx::Error> {
    if let Some(status) = status {
        sqlx::query_as::<_, SimRow>(
            r#"
            SELECT id, carrier, iccid, msisdn, sim_label, status, purchase_date,
                   purchase_cost_cents, data_plan_label, account_id, device_id,
                   activated_at, last_status_check_at, data_exhausted_at, notes,
                   created_at, updated_at
            FROM sim_cards
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, SimRow>(
            r#"
            SELECT id, carrier, iccid, msisdn, sim_label, status, purchase_date,
                   purchase_cost_cents, data_plan_label, account_id, device_id,
                   activated_at, last_status_check_at, data_exhausted_at, notes,
                   created_at, updated_at
            FROM sim_cards
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
    }
}

pub async fn list_by_account(
    pool: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<Vec<SimRow>, sqlx::Error> {
    sqlx::query_as::<_, SimRow>(
        r#"
        SELECT id, carrier, iccid, msisdn, sim_label, status, purchase_date,
               purchase_cost_cents, data_plan_label, account_id, device_id,
               activated_at, last_status_check_at, data_exhausted_at, notes,
               created_at, updated_at
        FROM sim_cards
        WHERE account_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<SimRow>, sqlx::Error> {
    sqlx::query_as::<_, SimRow>(
        r#"
        SELECT id, carrier, iccid, msisdn, sim_label, status, purchase_date,
               purchase_cost_cents, data_plan_label, account_id, device_id,
               activated_at, last_status_check_at, data_exhausted_at, notes,
               created_at, updated_at
        FROM sim_cards
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn assign(
    pool: &sqlx::PgPool,
    sim_id: Uuid,
    input: &AssignSimInput,
) -> Result<SimRow, sqlx::Error> {
    let new_status = if input.device_id.is_some() {
        SimStatus::Active
    } else {
        SimStatus::Assigned
    };

    sqlx::query_as::<_, SimRow>(
        r#"
        UPDATE sim_cards
        SET account_id = $2,
            device_id = $3,
            status = $4,
            activated_at = CASE WHEN $4 = 'active'::sim_status THEN COALESCE(activated_at, now()) ELSE activated_at END,
            updated_at = now()
        WHERE id = $1 AND status IN ('inventory', 'assigned')
        RETURNING id, carrier, iccid, msisdn, sim_label, status, purchase_date,
                  purchase_cost_cents, data_plan_label, account_id, device_id,
                  activated_at, last_status_check_at, data_exhausted_at, notes,
                  created_at, updated_at
        "#,
    )
    .bind(sim_id)
    .bind(input.account_id)
    .bind(input.device_id)
    .bind(new_status)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)
}
