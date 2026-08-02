use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::types::DeviceStatus;

#[derive(Debug, Clone, FromRow)]
pub struct DeviceRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub plate_number: Option<String>,
    pub imei: Option<String>,
    pub provider: String,
    pub provider_device_id: Option<String>,
    pub provider_account_ref: Option<String>,
    pub status: DeviceStatus,
    pub install_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateDeviceInput {
    pub name: String,
    pub plate_number: Option<String>,
    pub imei: Option<String>,
    pub provider: Option<String>,
    pub provider_device_id: Option<String>,
    pub status: Option<DeviceStatus>,
    pub install_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

pub async fn create(
    pool: &sqlx::PgPool,
    account_id: Uuid,
    input: &CreateDeviceInput,
) -> Result<DeviceRow, sqlx::Error> {
    let id = Uuid::new_v4();
    let provider = input.provider.as_deref().unwrap_or("tracksolid");
    let status = input.status.unwrap_or(DeviceStatus::PendingInstall);

    sqlx::query_as::<_, DeviceRow>(
        r#"
        INSERT INTO devices (
            id, account_id, name, plate_number, imei, provider, provider_device_id,
            status, install_date, notes, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, now(), now())
        RETURNING id, account_id, name, plate_number, imei, provider, provider_device_id,
                  provider_account_ref, status, install_date, notes, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(account_id)
    .bind(&input.name)
    .bind(&input.plate_number)
    .bind(&input.imei)
    .bind(provider)
    .bind(&input.provider_device_id)
    .bind(status)
    .bind(input.install_date)
    .bind(&input.notes)
    .fetch_one(pool)
    .await
}

pub async fn list_by_account(
    pool: &sqlx::PgPool,
    account_id: Uuid,
) -> Result<Vec<DeviceRow>, sqlx::Error> {
    sqlx::query_as::<_, DeviceRow>(
        r#"
        SELECT id, account_id, name, plate_number, imei, provider, provider_device_id,
               provider_account_ref, status, install_date, notes, created_at, updated_at
        FROM devices
        WHERE account_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(account_id)
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<DeviceRow>, sqlx::Error> {
    sqlx::query_as::<_, DeviceRow>(
        r#"
        SELECT id, account_id, name, plate_number, imei, provider, provider_device_id,
               provider_account_ref, status, install_date, notes, created_at, updated_at
        FROM devices
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
