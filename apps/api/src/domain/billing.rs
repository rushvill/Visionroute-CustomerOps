//! Customer invoices, payments received, and SIM data costs (VisionRoute spend).

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::{InvoiceStatus, PaymentMethod, SimCarrier};

#[derive(Debug, Clone, FromRow)]
pub struct InvoiceRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub number: String,
    pub description: String,
    pub amount_cents: i32,
    pub currency: String,
    pub status: InvoiceStatus,
    pub issued_at: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub paid_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct PaymentRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub amount_cents: i32,
    pub currency: String,
    pub method: PaymentMethod,
    pub paid_at: DateTime<Utc>,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct SimDataCostRow {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub sim_card_id: Option<Uuid>,
    pub amount_cents: i32,
    pub currency: String,
    pub carrier: Option<SimCarrier>,
    pub description: String,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub paid_at: NaiveDate,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateInvoiceInput {
    pub description: String,
    pub amount_cents: i32,
    pub currency: Option<String>,
    pub status: Option<InvoiceStatus>,
    pub issued_at: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreatePaymentInput {
    pub invoice_id: Option<Uuid>,
    pub amount_cents: i32,
    pub currency: Option<String>,
    pub method: Option<PaymentMethod>,
    pub paid_at: Option<DateTime<Utc>>,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateSimDataCostInput {
    pub account_id: Option<Uuid>,
    pub sim_card_id: Option<Uuid>,
    pub amount_cents: i32,
    pub currency: Option<String>,
    pub carrier: Option<SimCarrier>,
    pub description: String,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub paid_at: Option<NaiveDate>,
    pub notes: Option<String>,
}

pub async fn next_invoice_number(pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    let seq: i64 = sqlx::query_scalar("SELECT nextval('invoice_number_seq')")
        .fetch_one(pool)
        .await?;
    Ok(format!("INV-{seq:06}"))
}

pub async fn create_invoice(
    pool: &sqlx::PgPool,
    account_id: Uuid,
    input: &CreateInvoiceInput,
) -> Result<InvoiceRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let number = next_invoice_number(pool).await?;
    let currency = input
        .currency
        .clone()
        .unwrap_or_else(|| "PHP".to_owned());
    let status = input.status.unwrap_or(InvoiceStatus::Sent);
    let issued_at = input.issued_at.unwrap_or_else(|| Utc::now().date_naive());

    sqlx::query_as::<_, InvoiceRow>(
        r#"
        INSERT INTO customer_invoices (
            id, account_id, number, description, amount_cents, currency, status,
            issued_at, due_date, notes, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now(), now())
        RETURNING id, account_id, number, description, amount_cents, currency, status,
                  issued_at, due_date, paid_at, notes, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(account_id)
    .bind(&number)
    .bind(&input.description)
    .bind(input.amount_cents)
    .bind(&currency)
    .bind(status)
    .bind(issued_at)
    .bind(input.due_date)
    .bind(&input.notes)
    .fetch_one(pool)
    .await
}

pub async fn list_invoices(pool: &sqlx::PgPool) -> Result<Vec<InvoiceRow>, sqlx::Error> {
    sqlx::query_as::<_, InvoiceRow>(
        r#"
        SELECT id, account_id, number, description, amount_cents, currency, status,
               issued_at, due_date, paid_at, notes, created_at, updated_at
        FROM customer_invoices
        ORDER BY issued_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn list_invoices_by_account(
    pool: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<Vec<InvoiceRow>, sqlx::Error> {
    sqlx::query_as::<_, InvoiceRow>(
        r#"
        SELECT id, account_id, number, description, amount_cents, currency, status,
               issued_at, due_date, paid_at, notes, created_at, updated_at
        FROM customer_invoices
        WHERE account_id = $1
        ORDER BY issued_at DESC, created_at DESC
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
}

pub async fn get_invoice(
    pool: &sqlx::PgPool,
    id: Uuid,
) -> Result<Option<InvoiceRow>, sqlx::Error> {
    sqlx::query_as::<_, InvoiceRow>(
        r#"
        SELECT id, account_id, number, description, amount_cents, currency, status,
               issued_at, due_date, paid_at, notes, created_at, updated_at
        FROM customer_invoices
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_open_invoices(pool: &sqlx::PgPool) -> Result<Vec<InvoiceRow>, sqlx::Error> {
    sqlx::query_as::<_, InvoiceRow>(
        r#"
        SELECT id, account_id, number, description, amount_cents, currency, status,
               issued_at, due_date, paid_at, notes, created_at, updated_at
        FROM customer_invoices
        WHERE status IN ('sent', 'partial', 'overdue')
        ORDER BY due_date NULLS LAST, issued_at ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

async fn refresh_invoice_payment_status(
    pool: &sqlx::PgPool,
    invoice_id: Uuid,
) -> Result<(), sqlx::Error> {
    let invoice = get_invoice(pool, invoice_id).await?;
    let Some(invoice) = invoice else {
        return Ok(());
    };
    if matches!(invoice.status, InvoiceStatus::Cancelled | InvoiceStatus::Draft) {
        return Ok(());
    }

    let paid_sum: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::bigint
        FROM customer_payments
        WHERE invoice_id = $1
        "#,
    )
    .bind(invoice_id)
    .fetch_one(pool)
    .await?;

    let (status, paid_at) = if paid_sum <= 0 {
        (InvoiceStatus::Sent, None)
    } else if paid_sum >= i64::from(invoice.amount_cents) {
        (InvoiceStatus::Paid, Some(Utc::now()))
    } else {
        (InvoiceStatus::Partial, None)
    };

    sqlx::query(
        r#"
        UPDATE customer_invoices
        SET status = $2, paid_at = $3, updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(invoice_id)
    .bind(status)
    .bind(paid_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_payment(
    pool: &sqlx::PgPool,
    account_id: Uuid,
    input: &CreatePaymentInput,
) -> Result<PaymentRow, sqlx::Error> {
    if let Some(invoice_id) = input.invoice_id {
        let invoice = get_invoice(pool, invoice_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if invoice.account_id != account_id {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    let id = Uuid::new_v4();
    let currency = input
        .currency
        .clone()
        .unwrap_or_else(|| "PHP".to_owned());
    let method = input.method.unwrap_or(PaymentMethod::Cash);
    let paid_at = input.paid_at.unwrap_or_else(Utc::now);

    let row = sqlx::query_as::<_, PaymentRow>(
        r#"
        INSERT INTO customer_payments (
            id, account_id, invoice_id, amount_cents, currency, method,
            paid_at, reference, notes, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, now())
        RETURNING id, account_id, invoice_id, amount_cents, currency, method,
                  paid_at, reference, notes, created_at
        "#,
    )
    .bind(id)
    .bind(account_id)
    .bind(input.invoice_id)
    .bind(input.amount_cents)
    .bind(&currency)
    .bind(method)
    .bind(paid_at)
    .bind(&input.reference)
    .bind(&input.notes)
    .fetch_one(pool)
    .await?;

    if let Some(invoice_id) = input.invoice_id {
        refresh_invoice_payment_status(pool, invoice_id).await?;
    }

    Ok(row)
}

pub async fn list_payments(pool: &sqlx::PgPool) -> Result<Vec<PaymentRow>, sqlx::Error> {
    sqlx::query_as::<_, PaymentRow>(
        r#"
        SELECT id, account_id, invoice_id, amount_cents, currency, method,
               paid_at, reference, notes, created_at
        FROM customer_payments
        ORDER BY paid_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn list_payments_by_account(
    pool: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<Vec<PaymentRow>, sqlx::Error> {
    sqlx::query_as::<_, PaymentRow>(
        r#"
        SELECT id, account_id, invoice_id, amount_cents, currency, method,
               paid_at, reference, notes, created_at
        FROM customer_payments
        WHERE account_id = $1
        ORDER BY paid_at DESC, created_at DESC
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
}

pub async fn create_sim_data_cost(
    pool: &sqlx::PgPool,
    input: &CreateSimDataCostInput,
) -> Result<SimDataCostRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let currency = input
        .currency
        .clone()
        .unwrap_or_else(|| "PHP".to_owned());
    let paid_at = input.paid_at.unwrap_or_else(|| Utc::now().date_naive());

    sqlx::query_as::<_, SimDataCostRow>(
        r#"
        INSERT INTO sim_data_costs (
            id, account_id, sim_card_id, amount_cents, currency, carrier, description,
            period_start, period_end, paid_at, notes, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now())
        RETURNING id, account_id, sim_card_id, amount_cents, currency, carrier, description,
                  period_start, period_end, paid_at, notes, created_at
        "#,
    )
    .bind(id)
    .bind(input.account_id)
    .bind(input.sim_card_id)
    .bind(input.amount_cents)
    .bind(&currency)
    .bind(input.carrier)
    .bind(&input.description)
    .bind(input.period_start)
    .bind(input.period_end)
    .bind(paid_at)
    .bind(&input.notes)
    .fetch_one(pool)
    .await
}

pub async fn list_sim_data_costs(pool: &sqlx::PgPool) -> Result<Vec<SimDataCostRow>, sqlx::Error> {
    sqlx::query_as::<_, SimDataCostRow>(
        r#"
        SELECT id, account_id, sim_card_id, amount_cents, currency, carrier, description,
               period_start, period_end, paid_at, notes, created_at
        FROM sim_data_costs
        ORDER BY paid_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn totals_summary(pool: &sqlx::PgPool) -> Result<(i64, i64, i64), sqlx::Error> {
    let open_owed: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::bigint
        FROM customer_invoices
        WHERE status IN ('sent', 'partial', 'overdue')
        "#,
    )
    .fetch_one(pool)
    .await?;

    let payments_received: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::bigint
        FROM customer_payments
        "#,
    )
    .fetch_one(pool)
    .await?;

    let sim_costs: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::bigint
        FROM sim_data_costs
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok((open_owed, payments_received, sim_costs))
}
