use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::accounts::{self, CreateAccountInput};
use super::plans;
use super::subscriptions::{self, CreateSubscriptionInput};
use super::types::{AccountStatus, CoveragePolicy, SignupStatus, SubscriptionStatus};
use crate::auth::password::hash_password;
use crate::users::{self, UserRole};

#[derive(Debug, Clone, FromRow)]
pub struct SignupRequestRow {
    pub id: Uuid,
    pub status: SignupStatus,
    pub full_name: String,
    pub company_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub requested_username: Option<String>,
    pub estimated_devices: Option<i32>,
    pub message: Option<String>,
    pub preferred_contact: Option<String>,
    pub ip_hash: Option<String>,
    pub user_agent_hash: Option<String>,
    pub privacy_accepted_at: Option<DateTime<Utc>>,
    pub privacy_notice_version: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub converted_account_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateSignupInput {
    pub full_name: String,
    pub company_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub requested_username: Option<String>,
    pub estimated_devices: Option<i32>,
    pub message: Option<String>,
    pub preferred_contact: Option<String>,
    pub ip_hash: Option<String>,
    pub user_agent_hash: Option<String>,
    pub privacy_accepted_at: Option<DateTime<Utc>>,
    pub privacy_notice_version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApproveSignupInput {
    pub username: String,
    pub password: String,
    pub plan_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RejectSignupInput {
    pub reason: String,
}

pub struct ApproveSignupResult {
    pub signup: SignupRequestRow,
    pub account_id: Uuid,
    pub user_id: Uuid,
}

pub async fn create_request(
    pool: &sqlx::PgPool,
    input: &CreateSignupInput,
) -> Result<SignupRequestRow, sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query_as::<_, SignupRequestRow>(
        r#"
        INSERT INTO signup_requests (
            id, status, full_name, company_name, email, phone, requested_username,
            estimated_devices, message, preferred_contact, ip_hash, user_agent_hash,
            privacy_accepted_at, privacy_notice_version,
            created_at, updated_at
        ) VALUES (
            $1, 'new', $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, now(), now()
        )
        RETURNING id, status, full_name, company_name, email, phone, requested_username,
                  estimated_devices, message, preferred_contact, ip_hash, user_agent_hash,
                  privacy_accepted_at, privacy_notice_version,
                  reviewed_by, reviewed_at, rejection_reason, converted_account_id,
                  created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&input.full_name)
    .bind(&input.company_name)
    .bind(&input.email)
    .bind(&input.phone)
    .bind(&input.requested_username)
    .bind(input.estimated_devices)
    .bind(&input.message)
    .bind(&input.preferred_contact)
    .bind(&input.ip_hash)
    .bind(&input.user_agent_hash)
    .bind(input.privacy_accepted_at)
    .bind(&input.privacy_notice_version)
    .fetch_one(pool)
    .await
}

pub async fn list(pool: &sqlx::PgPool) -> Result<Vec<SignupRequestRow>, sqlx::Error> {
    sqlx::query_as::<_, SignupRequestRow>(
        r#"
        SELECT id, status, full_name, company_name, email, phone, requested_username,
               estimated_devices, message, preferred_contact, ip_hash, user_agent_hash,
               privacy_accepted_at, privacy_notice_version,
               reviewed_by, reviewed_at, rejection_reason, converted_account_id,
               created_at, updated_at
        FROM signup_requests
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(
    pool: &sqlx::PgPool,
    id: Uuid,
) -> Result<Option<SignupRequestRow>, sqlx::Error> {
    sqlx::query_as::<_, SignupRequestRow>(
        r#"
        SELECT id, status, full_name, company_name, email, phone, requested_username,
               estimated_devices, message, preferred_contact, ip_hash, user_agent_hash,
               privacy_accepted_at, privacy_notice_version,
               reviewed_by, reviewed_at, rejection_reason, converted_account_id,
               created_at, updated_at
        FROM signup_requests
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn approve(
    pool: &sqlx::PgPool,
    signup_id: Uuid,
    reviewer_id: Uuid,
    input: &ApproveSignupInput,
) -> Result<ApproveSignupResult, ApproveError> {
    let signup = get_by_id(pool, signup_id)
        .await
        .map_err(ApproveError::Db)?
        .ok_or(ApproveError::NotFound)?;

    if signup.status != SignupStatus::New && signup.status != SignupStatus::Reviewing {
        return Err(ApproveError::InvalidState);
    }

    if users::find_by_username(pool, &input.username)
        .await
        .map_err(ApproveError::Db)?
        .is_some()
    {
        return Err(ApproveError::UsernameTaken);
    }

    let password_hash = hash_password(&input.password).map_err(ApproveError::Password)?;

    let mut tx = pool.begin().await.map_err(ApproveError::Db)?;

    let account = accounts::create(
        &mut *tx,
        &CreateAccountInput {
            id: None,
            company_name: signup.company_name.clone(),
            display_name: Some(signup.company_name.clone()),
            status: AccountStatus::Active,
            industry: None,
            tax_id: None,
            billing_email: Some(signup.email.clone()),
            operations_email: None,
            phone: signup.phone.clone(),
            address_line1: None,
            address_line2: None,
            city: None,
            province: None,
            postal_code: None,
            country: None,
            notes: signup.message.clone(),
            source: Some("signup".to_owned()),
        },
        Some(reviewer_id),
    )
    .await
    .map_err(ApproveError::Db)?;

    let user_id = Uuid::new_v4();
    users::insert_user(
        &mut *tx,
        user_id,
        &input.username,
        &signup.email,
        &password_hash,
        &signup.full_name,
        UserRole::Customer,
        Some(account.id),
    )
    .await
    .map_err(ApproveError::Db)?;

    let plan_code = input.plan_code.as_deref().unwrap_or("BASIC").to_owned();
    let plan = plans::find_by_code(&mut *tx, &plan_code)
        .await
        .map_err(ApproveError::Db)?;

    let updated = sqlx::query_as::<_, SignupRequestRow>(
        r#"
        UPDATE signup_requests
        SET status = 'approved',
            reviewed_by = $2,
            reviewed_at = now(),
            converted_account_id = $3,
            updated_at = now()
        WHERE id = $1
        RETURNING id, status, full_name, company_name, email, phone, requested_username,
                  estimated_devices, message, preferred_contact, ip_hash, user_agent_hash,
                  privacy_accepted_at, privacy_notice_version,
                  reviewed_by, reviewed_at, rejection_reason, converted_account_id,
                  created_at, updated_at
        "#,
    )
    .bind(signup_id)
    .bind(reviewer_id)
    .bind(account.id)
    .fetch_one(&mut *tx)
    .await
    .map_err(ApproveError::Db)?;

    tx.commit().await.map_err(ApproveError::Db)?;

    if let Some(plan) = plan {
        let starts = Utc::now();
        let months = plan.includes_data_months.unwrap_or(12) as u32;
        let data_end = Utc::now().date_naive() + chrono::Months::new(months);

        subscriptions::create(
            pool,
            account.id,
            &CreateSubscriptionInput {
                plan_id: plan.id,
                status: Some(SubscriptionStatus::Active),
                coverage_policy: Some(CoveragePolicy::ShoulderedByUs),
                starts_at: Some(starts),
                ends_at: None,
                data_coverage_starts_at: Some(Utc::now().date_naive()),
                data_coverage_ends_at: Some(data_end),
                amount_cents: Some(plan.price_cents),
                notes: None,
            },
        )
        .await
        .map_err(ApproveError::Db)?;
    }

    Ok(ApproveSignupResult {
        signup: updated,
        account_id: account.id,
        user_id,
    })
}

pub async fn reject(
    pool: &sqlx::PgPool,
    signup_id: Uuid,
    reviewer_id: Uuid,
    input: &RejectSignupInput,
) -> Result<SignupRequestRow, RejectError> {
    let signup = get_by_id(pool, signup_id)
        .await
        .map_err(RejectError::Db)?
        .ok_or(RejectError::NotFound)?;

    if signup.status != SignupStatus::New && signup.status != SignupStatus::Reviewing {
        return Err(RejectError::InvalidState);
    }

    sqlx::query_as::<_, SignupRequestRow>(
        r#"
        UPDATE signup_requests
        SET status = 'rejected',
            reviewed_by = $2,
            reviewed_at = now(),
            rejection_reason = $3,
            updated_at = now()
        WHERE id = $1
        RETURNING id, status, full_name, company_name, email, phone, requested_username,
                  estimated_devices, message, preferred_contact, ip_hash, user_agent_hash,
                  privacy_accepted_at, privacy_notice_version,
                  reviewed_by, reviewed_at, rejection_reason, converted_account_id,
                  created_at, updated_at
        "#,
    )
    .bind(signup_id)
    .bind(reviewer_id)
    .bind(&input.reason)
    .fetch_one(pool)
    .await
    .map_err(RejectError::Db)
}

#[derive(Debug, thiserror::Error)]
pub enum ApproveError {
    #[error("not found")]
    NotFound,
    #[error("invalid state")]
    InvalidState,
    #[error("username taken")]
    UsernameTaken,
    #[error("password hash failed")]
    Password(#[source] crate::error::AppError),
    #[error("database error")]
    Db(#[source] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RejectError {
    #[error("not found")]
    NotFound,
    #[error("invalid state")]
    InvalidState,
    #[error("database error")]
    Db(#[source] sqlx::Error),
}
