use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::AccountStatus;

#[derive(Debug, Clone, FromRow)]
pub struct AccountRow {
    pub id: Uuid,
    pub account_code: String,
    pub company_name: String,
    pub display_name: Option<String>,
    pub status: AccountStatus,
    pub industry: Option<String>,
    pub tax_id: Option<String>,
    pub billing_email: Option<String>,
    pub operations_email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub notes: Option<String>,
    pub tracksolid_account_ref: Option<String>,
    pub source: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateAccountInput {
    pub id: Option<Uuid>,
    pub company_name: String,
    pub display_name: Option<String>,
    pub status: AccountStatus,
    pub industry: Option<String>,
    pub tax_id: Option<String>,
    pub billing_email: Option<String>,
    pub operations_email: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub notes: Option<String>,
    pub source: Option<String>,
}

pub async fn next_account_code(pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    let seq: i64 = sqlx::query_scalar("SELECT nextval('account_code_seq')")
        .fetch_one(pool)
        .await?;
    Ok(format!("VR-{seq:05}"))
}

pub async fn create<'e, E>(
    executor: E,
    input: &CreateAccountInput,
    created_by: Option<Uuid>,
) -> Result<AccountRow, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    let id = input.id.unwrap_or_else(Uuid::new_v4);
    let country = input.country.as_deref().unwrap_or("PH");

    sqlx::query_as::<_, AccountRow>(
        r#"
        INSERT INTO accounts (
            id, account_code, company_name, display_name, status, industry, tax_id,
            billing_email, operations_email, phone, address_line1, address_line2,
            city, province, postal_code, country, notes, source, approved_at, approved_by,
            created_by, updated_by, created_at, updated_at
        ) VALUES (
            $1,
            'VR-' || lpad(nextval('account_code_seq')::text, 5, '0'),
            $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17,
            CASE WHEN $4 = 'active'::account_status THEN now() ELSE NULL END,
            CASE WHEN $4 = 'active'::account_status THEN $18 ELSE NULL END,
            $18, $18, now(), now()
        )
        RETURNING id, account_code, company_name, display_name, status, industry, tax_id,
                  billing_email, operations_email, phone, address_line1, address_line2,
                  city, province, postal_code, country, notes, tracksolid_account_ref, source,
                  approved_at, approved_by, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&input.company_name)
    .bind(&input.display_name)
    .bind(input.status)
    .bind(&input.industry)
    .bind(&input.tax_id)
    .bind(&input.billing_email)
    .bind(&input.operations_email)
    .bind(&input.phone)
    .bind(&input.address_line1)
    .bind(&input.address_line2)
    .bind(&input.city)
    .bind(&input.province)
    .bind(&input.postal_code)
    .bind(country)
    .bind(&input.notes)
    .bind(&input.source)
    .bind(created_by)
    .fetch_one(executor)
    .await
}

pub async fn list(pool: &sqlx::PgPool) -> Result<Vec<AccountRow>, sqlx::Error> {
    sqlx::query_as::<_, AccountRow>(
        r#"
        SELECT id, account_code, company_name, display_name, status, industry, tax_id,
               billing_email, operations_email, phone, address_line1, address_line2,
               city, province, postal_code, country, notes, tracksolid_account_ref, source,
               approved_at, approved_by, created_at, updated_at
        FROM accounts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<AccountRow>, sqlx::Error> {
    sqlx::query_as::<_, AccountRow>(
        r#"
        SELECT id, account_code, company_name, display_name, status, industry, tax_id,
               billing_email, operations_email, phone, address_line1, address_line2,
               city, province, postal_code, country, notes, tracksolid_account_ref, source,
               approved_at, approved_by, created_at, updated_at
        FROM accounts
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
