use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::{CoveragePolicy, SubscriptionStatus};

#[derive(Debug, Clone, FromRow)]
pub struct SubscriptionRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub plan_id: Uuid,
    pub promo_id: Option<Uuid>,
    pub status: SubscriptionStatus,
    pub coverage_policy: CoveragePolicy,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub data_coverage_starts_at: Option<NaiveDate>,
    pub data_coverage_ends_at: Option<NaiveDate>,
    pub continue_shouldering: Option<bool>,
    pub renews_at: Option<DateTime<Utc>>,
    pub amount_cents: Option<i32>,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateSubscriptionInput {
    pub plan_id: Uuid,
    pub status: Option<SubscriptionStatus>,
    pub coverage_policy: Option<CoveragePolicy>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub data_coverage_starts_at: Option<NaiveDate>,
    pub data_coverage_ends_at: Option<NaiveDate>,
    pub amount_cents: Option<i32>,
    pub notes: Option<String>,
}

pub async fn create(
    pool: &sqlx::PgPool,
    account_id: Uuid,
    input: &CreateSubscriptionInput,
) -> Result<SubscriptionRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let status = input.status.unwrap_or(SubscriptionStatus::Active);
    let coverage_policy = input
        .coverage_policy
        .unwrap_or(CoveragePolicy::ShoulderedByUs);
    let starts_at = input.starts_at.unwrap_or_else(Utc::now);

    let plan_months: Option<i32> =
        sqlx::query_scalar("SELECT includes_data_months FROM plans WHERE id = $1")
            .bind(input.plan_id)
            .fetch_optional(pool)
            .await?;

    let data_start = input
        .data_coverage_starts_at
        .unwrap_or_else(|| starts_at.date_naive());
    let data_end = input.data_coverage_ends_at.or_else(|| {
        let months = plan_months.unwrap_or(12) as u32;
        Some(data_start + chrono::Months::new(months))
    });

    sqlx::query_as::<_, SubscriptionRow>(
        r#"
        INSERT INTO subscriptions (
            id, account_id, plan_id, status, coverage_policy, starts_at, ends_at,
            data_coverage_starts_at, data_coverage_ends_at, amount_cents, notes,
            created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now(), now())
        RETURNING id, account_id, plan_id, promo_id, status, coverage_policy, starts_at, ends_at,
                  data_coverage_starts_at, data_coverage_ends_at, continue_shouldering, renews_at,
                  amount_cents, currency, notes, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(account_id)
    .bind(input.plan_id)
    .bind(status)
    .bind(coverage_policy)
    .bind(starts_at)
    .bind(input.ends_at)
    .bind(data_start)
    .bind(data_end)
    .bind(input.amount_cents)
    .bind(&input.notes)
    .fetch_one(pool)
    .await
}

pub async fn get_active_for_account(
    pool: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<Option<SubscriptionRow>, sqlx::Error> {
    sqlx::query_as::<_, SubscriptionRow>(
        r#"
        SELECT id, account_id, plan_id, promo_id, status, coverage_policy, starts_at, ends_at,
               data_coverage_starts_at, data_coverage_ends_at, continue_shouldering, renews_at,
               amount_cents, currency, notes, created_at, updated_at
        FROM subscriptions
        WHERE account_id = $1 AND status IN ('trial', 'active')
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(account_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_expiring(
    pool: &sqlx::PgPool,
    within_days: i32,
) -> Result<Vec<SubscriptionRow>, sqlx::Error> {
    sqlx::query_as::<_, SubscriptionRow>(
        r#"
        SELECT id, account_id, plan_id, promo_id, status, coverage_policy, starts_at, ends_at,
               data_coverage_starts_at, data_coverage_ends_at, continue_shouldering, renews_at,
               amount_cents, currency, notes, created_at, updated_at
        FROM subscriptions
        WHERE status IN ('trial', 'active')
          AND data_coverage_ends_at IS NOT NULL
          AND data_coverage_ends_at <= (CURRENT_DATE + $1::int)
        ORDER BY data_coverage_ends_at ASC
        "#,
    )
    .bind(within_days)
    .fetch_all(pool)
    .await
}

pub async fn list_all(pool: &sqlx::PgPool) -> Result<Vec<SubscriptionRow>, sqlx::Error> {
    sqlx::query_as::<_, SubscriptionRow>(
        r#"
        SELECT id, account_id, plan_id, promo_id, status, coverage_policy, starts_at, ends_at,
               data_coverage_starts_at, data_coverage_ends_at, continue_shouldering, renews_at,
               amount_cents, currency, notes, created_at, updated_at
        FROM subscriptions
        ORDER BY starts_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}
