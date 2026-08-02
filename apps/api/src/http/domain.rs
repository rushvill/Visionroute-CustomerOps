//! Phase 4 domain HTTP handlers.

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::audit;
use crate::auth::csrf::assert_trusted_origin;
use crate::authz::{assert_account_access, require_permission, Permission};
use crate::domain::{
    self,
    types::{
        AccountStatus, DeviceStatus, InvoiceStatus, PaymentMethod, PrivacyRequestStatus,
        PrivacyRequestType, SimCarrier, SimStatus, SubscriptionStatus, TicketCategory,
        TicketPriority, TicketStatus,
    },
    AccountRow, ApproveSignupInput, AssignSimInput, CreateAccountInput, CreateDeviceInput,
    CreateInvoiceInput, CreatePaymentInput, CreateSignupInput, CreateSimDataCostInput,
    CreateSimInput, CreateSubscriptionInput, CreateTicketInput, DeviceRow, InvoiceRow, PaymentRow,
    CreatePrivacyRequestInput, PrivacyRequestRow, UpdatePrivacyRequestInput,
    RejectSignupInput, SignupRequestRow, SimDataCostRow, SimRow, SubscriptionRow, TicketRow,
    UpdateTicketInput,
};
use crate::error::AppError;
use crate::http::extractors::AuthUser;
use crate::state::AppState;

// --- DTOs ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSignupBody {
    pub full_name: String,
    pub company_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub requested_username: Option<String>,
    pub estimated_devices: Option<i32>,
    pub message: Option<String>,
    pub preferred_contact: Option<String>,
    /// Must be true — records privacy notice acceptance.
    pub privacy_accepted: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupPublic {
    pub id: Uuid,
    pub status: crate::domain::types::SignupStatus,
    pub full_name: String,
    pub company_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub requested_username: Option<String>,
    pub estimated_devices: Option<i32>,
    pub message: Option<String>,
    pub preferred_contact: Option<String>,
    pub privacy_accepted_at: Option<DateTime<Utc>>,
    pub privacy_notice_version: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub converted_account_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<&SignupRequestRow> for SignupPublic {
    fn from(row: &SignupRequestRow) -> Self {
        Self {
            id: row.id,
            status: row.status,
            full_name: row.full_name.clone(),
            company_name: row.company_name.clone(),
            email: row.email.clone(),
            phone: row.phone.clone(),
            requested_username: row.requested_username.clone(),
            estimated_devices: row.estimated_devices,
            message: row.message.clone(),
            preferred_contact: row.preferred_contact.clone(),
            privacy_accepted_at: row.privacy_accepted_at,
            privacy_notice_version: row.privacy_notice_version.clone(),
            reviewed_at: row.reviewed_at,
            rejection_reason: row.rejection_reason.clone(),
            converted_account_id: row.converted_account_id,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveSignupBody {
    pub username: String,
    pub password: String,
    pub plan_code: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RejectSignupBody {
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveSignupResponse {
    pub signup: SignupPublic,
    pub account_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountBody {
    pub company_name: String,
    pub display_name: Option<String>,
    pub status: Option<AccountStatus>,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPublic {
    pub id: Uuid,
    pub account_code: String,
    pub company_name: String,
    pub display_name: Option<String>,
    pub status: AccountStatus,
    pub industry: Option<String>,
    pub billing_email: Option<String>,
    pub operations_email: Option<String>,
    pub phone: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: String,
    pub source: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<&AccountRow> for AccountPublic {
    fn from(row: &AccountRow) -> Self {
        Self {
            id: row.id,
            account_code: row.account_code.clone(),
            company_name: row.company_name.clone(),
            display_name: row.display_name.clone(),
            status: row.status,
            industry: row.industry.clone(),
            billing_email: row.billing_email.clone(),
            operations_email: row.operations_email.clone(),
            phone: row.phone.clone(),
            city: row.city.clone(),
            province: row.province.clone(),
            country: row.country.clone(),
            source: row.source.clone(),
            approved_at: row.approved_at,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeviceBody {
    pub name: String,
    pub plate_number: Option<String>,
    pub imei: Option<String>,
    pub provider: Option<String>,
    pub provider_device_id: Option<String>,
    pub status: Option<DeviceStatus>,
    pub install_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePublic {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub plate_number: Option<String>,
    pub imei: Option<String>,
    pub provider: String,
    pub status: DeviceStatus,
    pub install_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

impl From<&DeviceRow> for DevicePublic {
    fn from(row: &DeviceRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            name: row.name.clone(),
            plate_number: row.plate_number.clone(),
            imei: row.imei.clone(),
            provider: row.provider.clone(),
            status: row.status,
            install_date: row.install_date,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSimBody {
    pub carrier: Option<SimCarrier>,
    pub iccid: Option<String>,
    pub msisdn: Option<String>,
    pub sim_label: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost_cents: Option<i32>,
    pub data_plan_label: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimAdminPublic {
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
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimCustomerPublic {
    pub id: Uuid,
    pub carrier: SimCarrier,
    pub iccid: Option<String>,
    pub msisdn: Option<String>,
    pub sim_label: Option<String>,
    pub status: SimStatus,
    pub data_plan_label: Option<String>,
    pub account_id: Option<Uuid>,
    pub device_id: Option<Uuid>,
    pub activated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<&SimRow> for SimAdminPublic {
    fn from(row: &SimRow) -> Self {
        Self {
            id: row.id,
            carrier: row.carrier,
            iccid: row.iccid.clone(),
            msisdn: row.msisdn.clone(),
            sim_label: row.sim_label.clone(),
            status: row.status,
            purchase_date: row.purchase_date,
            purchase_cost_cents: row.purchase_cost_cents,
            data_plan_label: row.data_plan_label.clone(),
            account_id: row.account_id,
            device_id: row.device_id,
            activated_at: row.activated_at,
            created_at: row.created_at,
        }
    }
}

impl From<&SimRow> for SimCustomerPublic {
    fn from(row: &SimRow) -> Self {
        Self {
            id: row.id,
            carrier: row.carrier,
            iccid: row.iccid.clone(),
            msisdn: row.msisdn.clone(),
            sim_label: row.sim_label.clone(),
            status: row.status,
            data_plan_label: row.data_plan_label.clone(),
            account_id: row.account_id,
            device_id: row.device_id,
            activated_at: row.activated_at,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignSimBody {
    pub account_id: Uuid,
    pub device_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionBody {
    pub plan_id: Uuid,
    pub status: Option<SubscriptionStatus>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub data_coverage_starts_at: Option<NaiveDate>,
    pub data_coverage_ends_at: Option<NaiveDate>,
    pub amount_cents: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionAdminPublic {
    pub id: Uuid,
    pub account_id: Uuid,
    pub plan_id: Uuid,
    pub status: SubscriptionStatus,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub data_coverage_starts_at: Option<NaiveDate>,
    pub data_coverage_ends_at: Option<NaiveDate>,
    pub amount_cents: Option<i32>,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionCustomerPublic {
    pub id: Uuid,
    pub account_id: Uuid,
    pub plan_id: Uuid,
    pub plan_name: String,
    pub status: SubscriptionStatus,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub data_coverage_starts_at: Option<NaiveDate>,
    pub data_coverage_ends_at: Option<NaiveDate>,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

impl From<&SubscriptionRow> for SubscriptionAdminPublic {
    fn from(row: &SubscriptionRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            plan_id: row.plan_id,
            status: row.status,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
            data_coverage_starts_at: row.data_coverage_starts_at,
            data_coverage_ends_at: row.data_coverage_ends_at,
            amount_cents: row.amount_cents,
            currency: row.currency.clone(),
            notes: row.notes.clone(),
            created_at: row.created_at,
        }
    }
}

impl From<&SubscriptionRow> for SubscriptionCustomerPublic {
    fn from(row: &SubscriptionRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            plan_id: row.plan_id,
            plan_name: String::new(),
            status: row.status,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
            data_coverage_starts_at: row.data_coverage_starts_at,
            data_coverage_ends_at: row.data_coverage_ends_at,
            currency: row.currency.clone(),
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketBody {
    pub subject: String,
    pub description: Option<String>,
    pub category: Option<TicketCategory>,
    pub priority: Option<TicketPriority>,
    pub device_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketPublic {
    pub id: Uuid,
    pub account_id: Uuid,
    pub number: String,
    pub created_by_user_id: Uuid,
    pub assigned_to_user_id: Option<Uuid>,
    pub device_id: Option<Uuid>,
    pub subject: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub category: TicketCategory,
    pub created_at: DateTime<Utc>,
}

impl From<&TicketRow> for TicketPublic {
    fn from(row: &TicketRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            number: row.number.clone(),
            created_by_user_id: row.created_by_user_id,
            assigned_to_user_id: row.assigned_to_user_id,
            device_id: row.device_id,
            subject: row.subject.clone(),
            description: row.description.clone(),
            status: row.status,
            priority: row.priority,
            category: row.category,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchTicketBody {
    pub status: Option<TicketStatus>,
    pub priority: Option<TicketPriority>,
    pub assigned_to_user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct SimListQuery {
    pub status: Option<SimStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageExpiringQuery {
    #[serde(default = "default_within_days")]
    pub within_days: i32,
}

fn default_within_days() -> i32 {
    30
}

// --- Public signup ---

pub async fn create_signup_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateSignupBody>,
) -> Result<Json<SignupPublic>, AppError> {
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    validate_signup(&body)?;

    let rate_key = signup_rate_key(&headers, &body.email);
    if !state.login_limiter.check_allowed(&rate_key) {
        return Err(AppError::RateLimited);
    }

    let ip_hash = client_ip_from_headers(&headers).as_deref().map(hash_value);
    let ua_hash = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(hash_value);

    let row = domain::create_signup_request(
        &state.db,
        &CreateSignupInput {
            full_name: body.full_name.trim().to_owned(),
            company_name: body.company_name.trim().to_owned(),
            email: body.email.trim().to_ascii_lowercase(),
            phone: body.phone,
            requested_username: body.requested_username,
            estimated_devices: body.estimated_devices,
            message: body.message,
            preferred_contact: body.preferred_contact,
            ip_hash,
            user_agent_hash: ua_hash,
            privacy_accepted_at: Some(Utc::now()),
            privacy_notice_version: Some(domain::PRIVACY_NOTICE_VERSION.to_owned()),
        },
    )
    .await
    .map_err(|error| {
        tracing::error!(%error, "create signup failed");
        AppError::Internal(anyhow::anyhow!("create signup failed"))
    })?;

    let _ = audit::record(
        &state,
        None,
        None,
        "signup_request",
        row.id,
        "created",
        "Signup request submitted",
        json!({ "email": row.email, "company": row.company_name }),
    )
    .await;

    Ok(Json(SignupPublic::from(&row)))
}

// --- Admin signup ---

pub async fn admin_list_signups(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<SignupPublic>>, AppError> {
    require_permission(&state, &user, Permission::SignupReview).await?;
    let rows = domain::list_signups(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(SignupPublic::from).collect()))
}

pub async fn admin_approve_signup(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<ApproveSignupBody>,
) -> Result<Json<ApproveSignupResponse>, AppError> {
    require_permission(&state, &user, Permission::SignupReview).await?;

    if body.password.len() < 10 {
        return Err(AppError::Validation(
            "Password must be at least 10 characters.".to_owned(),
        ));
    }
    if body.username.trim().is_empty() {
        return Err(AppError::Validation("Username is required.".to_owned()));
    }

    let result = domain::approve_signup(
        &state.db,
        id,
        user.id,
        &ApproveSignupInput {
            username: body.username.trim().to_owned(),
            password: body.password,
            plan_code: body.plan_code,
        },
    )
    .await
    .map_err(map_approve_error)?;

    let _ = audit::record(
        &state,
        Some(user.id),
        Some(result.account_id),
        "signup_request",
        id,
        "approved",
        "Signup request approved",
        json!({
            "accountId": result.account_id,
            "userId": result.user_id,
        }),
    )
    .await;

    Ok(Json(ApproveSignupResponse {
        signup: SignupPublic::from(&result.signup),
        account_id: result.account_id,
        user_id: result.user_id,
    }))
}

pub async fn admin_reject_signup(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<RejectSignupBody>,
) -> Result<Json<SignupPublic>, AppError> {
    require_permission(&state, &user, Permission::SignupReview).await?;

    if body.reason.trim().is_empty() {
        return Err(AppError::Validation("Reason is required.".to_owned()));
    }

    let row = domain::reject_signup(
        &state.db,
        id,
        user.id,
        &RejectSignupInput {
            reason: body.reason.trim().to_owned(),
        },
    )
    .await
    .map_err(map_reject_error)?;

    let _ = audit::record(
        &state,
        Some(user.id),
        None,
        "signup_request",
        id,
        "rejected",
        "Signup request rejected",
        json!({ "reason": row.rejection_reason }),
    )
    .await;

    Ok(Json(SignupPublic::from(&row)))
}

// --- Admin accounts ---

pub async fn admin_list_accounts(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<AccountPublic>>, AppError> {
    require_permission(&state, &user, Permission::AccountReadAny).await?;
    let rows = domain::list_accounts(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(AccountPublic::from).collect()))
}

pub async fn admin_create_account(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(body): Json<CreateAccountBody>,
) -> Result<Json<AccountPublic>, AppError> {
    require_permission(&state, &user, Permission::AccountUpdateAny).await?;

    if body.company_name.trim().is_empty() {
        return Err(AppError::Validation("Company name is required.".to_owned()));
    }

    let row = domain::create_account(
        &state.db,
        &CreateAccountInput {
            id: None,
            company_name: body.company_name.trim().to_owned(),
            display_name: body.display_name,
            status: body.status.unwrap_or(AccountStatus::Active),
            industry: body.industry,
            tax_id: body.tax_id,
            billing_email: body.billing_email,
            operations_email: body.operations_email,
            phone: body.phone,
            address_line1: body.address_line1,
            address_line2: body.address_line2,
            city: body.city,
            province: body.province,
            postal_code: body.postal_code,
            country: body.country,
            notes: body.notes,
            source: body.source,
        },
        Some(user.id),
    )
    .await
    .map_err(db_err)?;

    Ok(Json(AccountPublic::from(&row)))
}

pub async fn admin_get_account(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountPublic>, AppError> {
    require_permission(&state, &user, Permission::AccountReadAny).await?;
    let row = domain::get_account(&state.db, id)
        .await
        .map_err(db_err)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(AccountPublic::from(&row)))
}

// --- Admin devices ---

pub async fn admin_create_device(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(account_id): Path<Uuid>,
    Json(body): Json<CreateDeviceBody>,
) -> Result<Json<DevicePublic>, AppError> {
    require_permission(&state, &user, Permission::DeviceManage).await?;
    assert_account_access(&state, &user, account_id).await?;

    if body.name.trim().is_empty() {
        return Err(AppError::Validation("Device name is required.".to_owned()));
    }

    let row = domain::create_device(
        &state.db,
        account_id,
        &CreateDeviceInput {
            name: body.name.trim().to_owned(),
            plate_number: body.plate_number,
            imei: body.imei,
            provider: body.provider,
            provider_device_id: body.provider_device_id,
            status: body.status,
            install_date: body.install_date,
            notes: body.notes,
        },
    )
    .await
    .map_err(db_err)?;

    Ok(Json(DevicePublic::from(&row)))
}

pub async fn admin_list_devices(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<DevicePublic>>, AppError> {
    require_permission(&state, &user, Permission::AccountReadAny).await?;
    assert_account_access(&state, &user, account_id).await?;

    let rows = domain::list_devices_by_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(DevicePublic::from).collect()))
}

// --- Admin SIMs ---

pub async fn admin_list_sims(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Query(query): Query<SimListQuery>,
) -> Result<Json<Vec<SimAdminPublic>>, AppError> {
    require_permission(&state, &user, Permission::SimInventoryRead).await?;
    let rows = domain::list_sims_all(&state.db, query.status)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(SimAdminPublic::from).collect()))
}

pub async fn admin_create_sim(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(body): Json<CreateSimBody>,
) -> Result<Json<SimAdminPublic>, AppError> {
    require_permission(&state, &user, Permission::SimAssign).await?;

    if body.iccid.as_ref().is_none_or(|s| s.trim().is_empty())
        && body.msisdn.as_ref().is_none_or(|s| s.trim().is_empty())
    {
        return Err(AppError::Validation(
            "ICCID or MSISDN is required.".to_owned(),
        ));
    }

    let row = domain::create_sim(
        &state.db,
        &CreateSimInput {
            carrier: body.carrier,
            iccid: body.iccid,
            msisdn: body.msisdn,
            sim_label: body.sim_label,
            purchase_date: body.purchase_date,
            purchase_cost_cents: body.purchase_cost_cents,
            data_plan_label: body.data_plan_label,
            notes: body.notes,
        },
    )
    .await
    .map_err(db_err)?;

    Ok(Json(SimAdminPublic::from(&row)))
}

pub async fn admin_assign_sim(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<AssignSimBody>,
) -> Result<Json<SimAdminPublic>, AppError> {
    require_permission(&state, &user, Permission::SimAssign).await?;

    domain::sims::get_by_id(&state.db, id)
        .await
        .map_err(db_err)?
        .ok_or(AppError::NotFound)?;

    if let Some(device_id) = body.device_id {
        let device = domain::devices::get_by_id(&state.db, device_id)
            .await
            .map_err(db_err)?
            .ok_or(AppError::NotFound)?;
        if device.account_id != body.account_id {
            return Err(AppError::Validation(
                "Device does not belong to the specified account.".to_owned(),
            ));
        }
    }

    let row = domain::assign_sim(
        &state.db,
        id,
        &AssignSimInput {
            account_id: body.account_id,
            device_id: body.device_id,
        },
    )
    .await
    .map_err(|error| {
        if matches!(error, sqlx::Error::RowNotFound) {
            AppError::NotFound
        } else {
            tracing::error!(%error, "sim assign failed");
            AppError::Internal(anyhow::anyhow!("sim assign failed"))
        }
    })?;

    let _ = audit::record(
        &state,
        Some(user.id),
        Some(body.account_id),
        "sim_card",
        id,
        "assigned",
        "SIM assigned to account",
        json!({
            "accountId": body.account_id,
            "deviceId": body.device_id,
        }),
    )
    .await;

    Ok(Json(SimAdminPublic::from(&row)))
}

// --- Admin subscriptions ---

pub async fn admin_create_subscription(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(account_id): Path<Uuid>,
    Json(body): Json<CreateSubscriptionBody>,
) -> Result<Json<SubscriptionAdminPublic>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    assert_account_access(&state, &user, account_id).await?;

    let row = domain::create_subscription(
        &state.db,
        account_id,
        &CreateSubscriptionInput {
            plan_id: body.plan_id,
            status: body.status,
            coverage_policy: None,
            starts_at: body.starts_at,
            ends_at: body.ends_at,
            data_coverage_starts_at: body.data_coverage_starts_at,
            data_coverage_ends_at: body.data_coverage_ends_at,
            amount_cents: body.amount_cents,
            notes: body.notes,
        },
    )
    .await
    .map_err(db_err)?;

    Ok(Json(SubscriptionAdminPublic::from(&row)))
}

pub async fn admin_coverage_expiring(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Query(query): Query<CoverageExpiringQuery>,
) -> Result<Json<Vec<SubscriptionAdminPublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    let days = query.within_days.clamp(1, 365);
    let rows = domain::list_expiring(&state.db, days)
        .await
        .map_err(db_err)?;
    Ok(Json(
        rows.iter().map(SubscriptionAdminPublic::from).collect(),
    ))
}

// --- Admin tickets ---

pub async fn admin_list_tickets(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<TicketPublic>>, AppError> {
    require_permission(&state, &user, Permission::TicketManageAll).await?;
    let rows = domain::list_tickets_all(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(TicketPublic::from).collect()))
}

pub async fn admin_patch_ticket(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchTicketBody>,
) -> Result<Json<TicketPublic>, AppError> {
    require_permission(&state, &user, Permission::TicketManageAll).await?;

    let row = domain::update_ticket(
        &state.db,
        id,
        &UpdateTicketInput {
            status: body.status,
            priority: body.priority,
            assigned_to_user_id: Some(body.assigned_to_user_id),
        },
    )
    .await
    .map_err(|error| {
        if matches!(error, sqlx::Error::RowNotFound) {
            AppError::NotFound
        } else {
            db_err(error)
        }
    })?;

    Ok(Json(TicketPublic::from(&row)))
}

// --- Customer /me ---

pub async fn me_account(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<AccountPublic>, AppError> {
    require_permission(&state, &user, Permission::AccountReadOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    let row = domain::get_account(&state.db, account_id)
        .await
        .map_err(db_err)?
        .ok_or(AppError::NotFound)?;
    Ok(Json(AccountPublic::from(&row)))
}

pub async fn me_devices(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<DevicePublic>>, AppError> {
    require_permission(&state, &user, Permission::DeviceReadOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;
    let rows = domain::list_devices_by_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(DevicePublic::from).collect()))
}

pub async fn me_sims(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<SimCustomerPublic>>, AppError> {
    require_permission(&state, &user, Permission::SimReadOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;
    let rows = domain::list_sims_by_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(SimCustomerPublic::from).collect()))
}

pub async fn me_subscription(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Option<SubscriptionCustomerPublic>>, AppError> {
    require_permission(&state, &user, Permission::SubscriptionReadOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;
    let row = domain::get_active_for_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    let Some(row) = row else {
        return Ok(Json(None));
    };
    let plan_name = domain::find_plan_by_id(&state.db, row.plan_id)
        .await
        .map_err(db_err)?
        .map(|p| p.name)
        .unwrap_or_else(|| "Plan".to_owned());
    let mut dto = SubscriptionCustomerPublic::from(&row);
    dto.plan_name = plan_name;
    Ok(Json(Some(dto)))
}

pub async fn me_list_tickets(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<TicketPublic>>, AppError> {
    require_permission(&state, &user, Permission::TicketOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;
    let rows = domain::list_tickets_by_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(TicketPublic::from).collect()))
}

pub async fn me_create_ticket(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    headers: HeaderMap,
    Json(body): Json<CreateTicketBody>,
) -> Result<Json<TicketPublic>, AppError> {
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;
    require_permission(&state, &user, Permission::TicketOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;

    if body.subject.trim().is_empty() {
        return Err(AppError::Validation("Subject is required.".to_owned()));
    }

    if let Some(device_id) = body.device_id {
        let device = domain::devices::get_by_id(&state.db, device_id)
            .await
            .map_err(db_err)?
            .ok_or(AppError::NotFound)?;
        if device.account_id != account_id {
            return Err(AppError::Forbidden);
        }
    }

    let row = domain::create_ticket(
        &state.db,
        account_id,
        user.id,
        &CreateTicketInput {
            subject: body.subject.trim().to_owned(),
            description: body.description,
            category: body.category,
            priority: body.priority,
            device_id: body.device_id,
            sim_card_id: None,
        },
    )
    .await
    .map_err(db_err)?;

    let _ = audit::record(
        &state,
        Some(user.id),
        Some(account_id),
        "ticket",
        row.id,
        "created",
        "Support ticket created",
        json!({ "number": row.number, "subject": row.subject }),
    )
    .await;

    Ok(Json(TicketPublic::from(&row)))
}

// --- helpers ---

fn validate_signup(body: &CreateSignupBody) -> Result<(), AppError> {
    if body.full_name.trim().is_empty() {
        return Err(AppError::Validation("Full name is required.".to_owned()));
    }
    if body.company_name.trim().is_empty() {
        return Err(AppError::Validation("Company name is required.".to_owned()));
    }
    if body.email.trim().is_empty() || !body.email.contains('@') {
        return Err(AppError::Validation("Valid email is required.".to_owned()));
    }
    if !body.privacy_accepted {
        return Err(AppError::Validation(
            "You must accept the privacy notice to sign up.".to_owned(),
        ));
    }
    Ok(())
}

fn signup_rate_key(headers: &HeaderMap, email: &str) -> String {
    let ip = client_ip_from_headers(headers).unwrap_or_else(|| "unknown".to_owned());
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    hasher.update(b"|signup|");
    hasher.update(email.trim().to_ascii_lowercase().as_bytes());
    hex::encode(hasher.finalize())
}

fn client_ip_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn hash_value(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    hex::encode(hasher.finalize())
}

fn db_err(error: sqlx::Error) -> AppError {
    tracing::error!(%error, "database error");
    AppError::Internal(anyhow::anyhow!("database error"))
}

fn map_approve_error(error: domain::signup::ApproveError) -> AppError {
    match error {
        domain::signup::ApproveError::NotFound => AppError::NotFound,
        domain::signup::ApproveError::InvalidState => {
            AppError::Validation("Signup request is not pending review.".to_owned())
        }
        domain::signup::ApproveError::UsernameTaken => {
            AppError::Validation("Username is already taken.".to_owned())
        }
        domain::signup::ApproveError::Password(e) => e,
        domain::signup::ApproveError::Db(e) => db_err(e),
    }
}

fn map_reject_error(error: domain::signup::RejectError) -> AppError {
    match error {
        domain::signup::RejectError::NotFound => AppError::NotFound,
        domain::signup::RejectError::InvalidState => {
            AppError::Validation("Signup request is not pending review.".to_owned())
        }
        domain::signup::RejectError::Db(e) => db_err(e),
    }
}

// --- Billing DTOs / handlers ---

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePublic {
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
}

impl From<&InvoiceRow> for InvoicePublic {
    fn from(row: &InvoiceRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            number: row.number.clone(),
            description: row.description.clone(),
            amount_cents: row.amount_cents,
            currency: row.currency.clone(),
            status: row.status,
            issued_at: row.issued_at,
            due_date: row.due_date,
            paid_at: row.paid_at,
            notes: row.notes.clone(),
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPublic {
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

impl From<&PaymentRow> for PaymentPublic {
    fn from(row: &PaymentRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            invoice_id: row.invoice_id,
            amount_cents: row.amount_cents,
            currency: row.currency.clone(),
            method: row.method,
            paid_at: row.paid_at,
            reference: row.reference.clone(),
            notes: row.notes.clone(),
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimDataCostPublic {
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

impl From<&SimDataCostRow> for SimDataCostPublic {
    fn from(row: &SimDataCostRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            sim_card_id: row.sim_card_id,
            amount_cents: row.amount_cents,
            currency: row.currency.clone(),
            carrier: row.carrier,
            description: row.description.clone(),
            period_start: row.period_start,
            period_end: row.period_end,
            paid_at: row.paid_at,
            notes: row.notes.clone(),
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingSummaryPublic {
    pub open_owed_cents: i64,
    pub payments_received_cents: i64,
    pub sim_data_cost_cents: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceBody {
    pub account_id: Uuid,
    pub description: String,
    pub amount_cents: i32,
    pub due_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentBody {
    pub account_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub amount_cents: i32,
    pub method: Option<PaymentMethod>,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSimDataCostBody {
    pub account_id: Option<Uuid>,
    pub sim_card_id: Option<Uuid>,
    pub amount_cents: i32,
    pub carrier: Option<SimCarrier>,
    pub description: String,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub paid_at: Option<NaiveDate>,
    pub notes: Option<String>,
}

pub async fn admin_billing_summary(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<BillingSummaryPublic>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    let (open_owed, payments, sim_costs) = domain::totals_summary(&state.db).await.map_err(db_err)?;
    Ok(Json(BillingSummaryPublic {
        open_owed_cents: open_owed,
        payments_received_cents: payments,
        sim_data_cost_cents: sim_costs,
    }))
}

pub async fn admin_list_invoices(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<InvoicePublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    let rows = domain::list_invoices(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(InvoicePublic::from).collect()))
}

pub async fn admin_create_invoice(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    headers: HeaderMap,
    Json(body): Json<CreateInvoiceBody>,
) -> Result<Json<InvoicePublic>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    if body.description.trim().is_empty() {
        return Err(AppError::Validation("Description is required.".to_owned()));
    }
    if body.amount_cents < 0 {
        return Err(AppError::Validation("Amount cannot be negative.".to_owned()));
    }

    let row = domain::create_invoice(
        &state.db,
        body.account_id,
        &CreateInvoiceInput {
            description: body.description.trim().to_owned(),
            amount_cents: body.amount_cents,
            currency: None,
            status: Some(InvoiceStatus::Sent),
            issued_at: None,
            due_date: body.due_date,
            notes: body.notes,
        },
    )
    .await
    .map_err(db_err)?;

    let _ = audit::record(
        &state,
        Some(user.id),
        Some(body.account_id),
        "invoice",
        row.id,
        "invoice_created",
        "Customer invoice created",
        json!({ "number": row.number, "amountCents": row.amount_cents }),
    )
    .await;

    Ok(Json(InvoicePublic::from(&row)))
}

pub async fn admin_list_payments(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<PaymentPublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    let rows = domain::list_payments(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(PaymentPublic::from).collect()))
}

pub async fn admin_create_payment(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    headers: HeaderMap,
    Json(body): Json<CreatePaymentBody>,
) -> Result<Json<PaymentPublic>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    if body.amount_cents <= 0 {
        return Err(AppError::Validation("Payment amount must be positive.".to_owned()));
    }

    let row = domain::create_payment(
        &state.db,
        body.account_id,
        &CreatePaymentInput {
            invoice_id: body.invoice_id,
            amount_cents: body.amount_cents,
            currency: None,
            method: body.method,
            paid_at: None,
            reference: body.reference,
            notes: body.notes,
        },
    )
    .await
    .map_err(|error| {
        if matches!(error, sqlx::Error::RowNotFound) {
            AppError::Validation("Invoice not found for this account.".to_owned())
        } else {
            db_err(error)
        }
    })?;

    let _ = audit::record(
        &state,
        Some(user.id),
        Some(body.account_id),
        "payment",
        row.id,
        "payment_recorded",
        "Customer payment recorded",
        json!({ "amountCents": row.amount_cents, "invoiceId": row.invoice_id }),
    )
    .await;

    Ok(Json(PaymentPublic::from(&row)))
}

pub async fn admin_list_sim_data_costs(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<SimDataCostPublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    let rows = domain::list_sim_data_costs(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(SimDataCostPublic::from).collect()))
}

pub async fn admin_create_sim_data_cost(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    headers: HeaderMap,
    Json(body): Json<CreateSimDataCostBody>,
) -> Result<Json<SimDataCostPublic>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    if body.description.trim().is_empty() {
        return Err(AppError::Validation("Description is required.".to_owned()));
    }
    if body.amount_cents < 0 {
        return Err(AppError::Validation("Amount cannot be negative.".to_owned()));
    }

    let row = domain::create_sim_data_cost(
        &state.db,
        &CreateSimDataCostInput {
            account_id: body.account_id,
            sim_card_id: body.sim_card_id,
            amount_cents: body.amount_cents,
            currency: None,
            carrier: body.carrier,
            description: body.description.trim().to_owned(),
            period_start: body.period_start,
            period_end: body.period_end,
            paid_at: body.paid_at,
            notes: body.notes,
        },
    )
    .await
    .map_err(db_err)?;

    let _ = audit::record(
        &state,
        Some(user.id),
        body.account_id,
        "sim_data_cost",
        row.id,
        "sim_data_cost_recorded",
        "SIM data cost recorded",
        json!({ "amountCents": row.amount_cents }),
    )
    .await;

    Ok(Json(SimDataCostPublic::from(&row)))
}

pub async fn admin_list_subscriptions(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<SubscriptionAdminPublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingManage).await?;
    let rows = domain::list_subscriptions(&state.db).await.map_err(db_err)?;
    Ok(Json(
        rows.iter().map(SubscriptionAdminPublic::from).collect(),
    ))
}

pub async fn me_invoices(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<InvoicePublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingReadOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;
    let rows = domain::list_invoices_by_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(InvoicePublic::from).collect()))
}

pub async fn me_payments(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<PaymentPublic>>, AppError> {
    require_permission(&state, &user, Permission::BillingReadOwn).await?;
    let account_id = user.account_id.ok_or(AppError::NotFound)?;
    assert_account_access(&state, &user, account_id).await?;
    let rows = domain::list_payments_by_account(&state.db, account_id)
        .await
        .map_err(db_err)?;
    Ok(Json(rows.iter().map(PaymentPublic::from).collect()))
}

// --- Privacy / DSAR ---

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyRequestPublic {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub requester_name: Option<String>,
    pub requester_email: String,
    pub request_type: PrivacyRequestType,
    pub details: Option<String>,
    pub status: PrivacyRequestStatus,
    pub handled_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<&PrivacyRequestRow> for PrivacyRequestPublic {
    fn from(row: &PrivacyRequestRow) -> Self {
        Self {
            id: row.id,
            account_id: row.account_id,
            requester_name: row.requester_name.clone(),
            requester_email: row.requester_email.clone(),
            request_type: row.request_type,
            details: row.details.clone(),
            status: row.status,
            handled_at: row.handled_at,
            resolution_notes: row.resolution_notes.clone(),
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePrivacyRequestBody {
    pub requester_name: Option<String>,
    pub requester_email: String,
    pub request_type: PrivacyRequestType,
    pub details: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchPrivacyRequestBody {
    pub status: PrivacyRequestStatus,
    pub resolution_notes: Option<String>,
}

pub async fn create_privacy_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreatePrivacyRequestBody>,
) -> Result<Json<PrivacyRequestPublic>, AppError> {
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    let email = body.requester_email.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(AppError::Validation("Valid email is required.".to_owned()));
    }

    let row = domain::create_privacy_request(
        &state.db,
        &CreatePrivacyRequestInput {
            account_id: None,
            requester_name: body
                .requester_name
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty()),
            requester_email: email,
            request_type: body.request_type,
            details: body.details,
        },
    )
    .await
    .map_err(db_err)?;

    let _ = audit::record(
        &state,
        None,
        None,
        "privacy_request",
        row.id,
        "privacy_request_created",
        "Privacy / data rights request received",
        json!({ "type": format!("{:?}", row.request_type) }),
    )
    .await;

    Ok(Json(PrivacyRequestPublic::from(&row)))
}

pub async fn me_create_privacy_request(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    headers: HeaderMap,
    Json(body): Json<CreatePrivacyRequestBody>,
) -> Result<Json<PrivacyRequestPublic>, AppError> {
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;
    let account_id = user.account_id;
    if let Some(aid) = account_id {
        assert_account_access(&state, &user, aid).await?;
    }

    let email = body.requester_email.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(AppError::Validation("Valid email is required.".to_owned()));
    }

    let row = domain::create_privacy_request(
        &state.db,
        &CreatePrivacyRequestInput {
            account_id,
            requester_name: body
                .requester_name
                .or(Some(user.full_name.clone()))
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty()),
            requester_email: email,
            request_type: body.request_type,
            details: body.details,
        },
    )
    .await
    .map_err(db_err)?;

    Ok(Json(PrivacyRequestPublic::from(&row)))
}

pub async fn admin_list_privacy_requests(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<Vec<PrivacyRequestPublic>>, AppError> {
    require_permission(&state, &user, Permission::PrivacyManage).await?;
    let rows = domain::list_privacy_requests(&state.db).await.map_err(db_err)?;
    Ok(Json(rows.iter().map(PrivacyRequestPublic::from).collect()))
}

pub async fn admin_patch_privacy_request(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchPrivacyRequestBody>,
) -> Result<Json<PrivacyRequestPublic>, AppError> {
    require_permission(&state, &user, Permission::PrivacyManage).await?;
    assert_trusted_origin(&headers, &state.config.frontend_origin)?;

    let row = domain::update_privacy_request(
        &state.db,
        id,
        user.id,
        &UpdatePrivacyRequestInput {
            status: body.status,
            resolution_notes: body.resolution_notes,
        },
    )
    .await
    .map_err(|error| {
        if matches!(error, sqlx::Error::RowNotFound) {
            AppError::NotFound
        } else {
            db_err(error)
        }
    })?;

    Ok(Json(PrivacyRequestPublic::from(&row)))
}
