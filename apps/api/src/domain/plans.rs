use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PlanRow {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub currency: String,
    pub billing_cycle: String,
    pub device_limit: i32,
    pub included_sims: i32,
    pub includes_data_months: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_by_code<'e, E>(executor: E, code: &str) -> Result<Option<PlanRow>, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Postgres>,
{
    sqlx::query_as::<_, PlanRow>(
        r#"
        SELECT id, code, name, description, price_cents, currency, billing_cycle,
               device_limit, included_sims, includes_data_months, is_active, created_at, updated_at
        FROM plans
        WHERE lower(code) = lower($1) AND is_active = TRUE
        LIMIT 1
        "#,
    )
    .bind(code)
    .fetch_optional(executor)
    .await
}

pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<PlanRow>, sqlx::Error> {
    sqlx::query_as::<_, PlanRow>(
        r#"
        SELECT id, code, name, description, price_cents, currency, billing_cycle,
               device_limit, included_sims, includes_data_months, is_active, created_at, updated_at
        FROM plans
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn ensure_basic_plan(pool: &sqlx::PgPool) -> Result<PlanRow, sqlx::Error> {
    if let Some(plan) = find_by_code(pool, "BASIC").await? {
        return Ok(plan);
    }

    let id = Uuid::new_v4();
    sqlx::query_as::<_, PlanRow>(
        r#"
        INSERT INTO plans (
            id, code, name, description, price_cents, currency, billing_cycle,
            device_limit, included_sims, includes_data_months, is_active, created_at, updated_at
        ) VALUES (
            $1, 'BASIC', 'Basic Fleet', 'Default fleet tracking plan', 0, 'PHP', 'yearly',
            50, 50, 12, TRUE, now(), now()
        )
        RETURNING id, code, name, description, price_cents, currency, billing_cycle,
                  device_limit, included_sims, includes_data_months, is_active, created_at, updated_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
