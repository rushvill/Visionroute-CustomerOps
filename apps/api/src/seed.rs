//! Development-only seed data.

use chrono::{Days, Months, Utc};
use tracing::info;
use uuid::Uuid;

use crate::auth::password::hash_password;
use crate::domain::{
    self,
    types::{
        AccountStatus, DeviceStatus, SimCarrier, SubscriptionStatus, TicketCategory, TicketPriority,
    },
    CreateAccountInput, CreateDeviceInput, CreateSimInput, CreateSubscriptionInput,
    CreateTicketInput,
};
use crate::state::AppState;
use crate::users::{self, UserRole};

const DEV_ADMIN_USERNAME: &str = "admin";
const DEV_ADMIN_EMAIL: &str = "admin@customerops.local";
const DEV_ADMIN_PASSWORD: &str = "VisionRouteDemo26!";
const DEV_ADMIN_NAME: &str = "Customer Ops Admin";

const DEV_CUSTOMER_USERNAME: &str = "customer";
const DEV_CUSTOMER_EMAIL: &str = "customer@customerops.local";
const DEV_CUSTOMER_PASSWORD: &str = "VisionRouteDemo26!";
const DEV_CUSTOMER_NAME: &str = "Demo Customer";

/// Stable demo account id for the seeded customer.
pub const DEV_CUSTOMER_ACCOUNT_ID: Uuid =
    Uuid::from_u128(0x1111_2222_3333_4444_5555_6666_7777_8888);

/// Creates the first platform admin when the user table is empty.
///
/// Set `BOOTSTRAP_ADMIN_PASSWORD` (and optionally username/email/name) on first boot,
/// then remove the password env var after you can log in.
pub async fn ensure_bootstrap_admin(state: &AppState) -> anyhow::Result<()> {
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*)::bigint FROM users")
        .fetch_one(&state.db)
        .await?;
    if user_count > 0 {
        return Ok(());
    }

    let password = match std::env::var("BOOTSTRAP_ADMIN_PASSWORD") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            tracing::warn!(
                "users table is empty and BOOTSTRAP_ADMIN_PASSWORD is unset; no admin created"
            );
            return Ok(());
        }
    };

    if password.len() < 12 {
        anyhow::bail!("BOOTSTRAP_ADMIN_PASSWORD must be at least 12 characters");
    }

    let username =
        std::env::var("BOOTSTRAP_ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_owned());
    let email = std::env::var("BOOTSTRAP_ADMIN_EMAIL")
        .unwrap_or_else(|_| "admin@visionroute.local".to_owned());
    let full_name = std::env::var("BOOTSTRAP_ADMIN_NAME")
        .unwrap_or_else(|_| "VisionRoute Admin".to_owned());

    ensure_user(
        state,
        &username,
        &email,
        &password,
        &full_name,
        UserRole::Admin,
        None,
    )
    .await?;

    info!(%username, "bootstrapped first platform admin");
    Ok(())
}

/// Seeds platform admin, demo account, and related domain fixtures in development/test.
pub async fn ensure_dev_users(state: &AppState) -> anyhow::Result<()> {
    if !state.config.is_development() {
        return Ok(());
    }

    ensure_user(
        state,
        DEV_ADMIN_USERNAME,
        DEV_ADMIN_EMAIL,
        DEV_ADMIN_PASSWORD,
        DEV_ADMIN_NAME,
        UserRole::Admin,
        None,
    )
    .await?;

    let plan = domain::ensure_basic_plan(&state.db).await?;

    let account = if domain::get_account(&state.db, DEV_CUSTOMER_ACCOUNT_ID)
        .await?
        .is_some()
    {
        domain::get_account(&state.db, DEV_CUSTOMER_ACCOUNT_ID)
            .await?
            .expect("account exists")
    } else {
        domain::create_account(
            &state.db,
            &CreateAccountInput {
                id: Some(DEV_CUSTOMER_ACCOUNT_ID),
                company_name: "Demo Fleet Co".to_owned(),
                display_name: Some("Demo Fleet Co".to_owned()),
                status: AccountStatus::Active,
                industry: None,
                tax_id: None,
                billing_email: Some(DEV_CUSTOMER_EMAIL.to_owned()),
                operations_email: None,
                phone: None,
                address_line1: None,
                address_line2: None,
                city: None,
                province: None,
                postal_code: None,
                country: None,
                notes: None,
                source: Some("seed".to_owned()),
            },
            None,
        )
        .await?
    };

    ensure_user(
        state,
        DEV_CUSTOMER_USERNAME,
        DEV_CUSTOMER_EMAIL,
        DEV_CUSTOMER_PASSWORD,
        DEV_CUSTOMER_NAME,
        UserRole::Customer,
        Some(account.id),
    )
    .await?;

    seed_domain_fixtures(state, account.id, plan.id).await?;

    Ok(())
}

async fn seed_domain_fixtures(
    state: &AppState,
    account_id: Uuid,
    plan_id: Uuid,
) -> anyhow::Result<()> {
    let devices = domain::list_devices_by_account(&state.db, account_id).await?;
    let device = if let Some(existing) = devices.into_iter().next() {
        existing
    } else {
        domain::create_device(
            &state.db,
            account_id,
            &CreateDeviceInput {
                name: "Demo Truck 01".to_owned(),
                plate_number: Some("ABC-1234".to_owned()),
                imei: Some("860000000000001".to_owned()),
                provider: None,
                provider_device_id: None,
                status: Some(DeviceStatus::Active),
                install_date: None,
                notes: None,
            },
        )
        .await?
    };

    let sims = domain::list_sims_by_account(&state.db, account_id).await?;
    if sims.is_empty() {
        let sim = domain::create_sim(
            &state.db,
            &CreateSimInput {
                carrier: Some(SimCarrier::Smart),
                iccid: Some("89014103211118510720".to_owned()),
                msisdn: Some("639171234567".to_owned()),
                sim_label: Some("Demo SIM".to_owned()),
                purchase_date: None,
                purchase_cost_cents: Some(50000),
                data_plan_label: Some("12GB/year".to_owned()),
                notes: None,
            },
        )
        .await?;

        domain::assign_sim(
            &state.db,
            sim.id,
            &domain::AssignSimInput {
                account_id,
                device_id: Some(device.id),
            },
        )
        .await?;
    }

    if domain::get_active_for_account(&state.db, account_id)
        .await?
        .is_none()
    {
        let starts = Utc::now();
        let data_start = starts.date_naive();
        let data_end = data_start + Months::new(12);

        domain::create_subscription(
            &state.db,
            account_id,
            &CreateSubscriptionInput {
                plan_id,
                status: Some(SubscriptionStatus::Active),
                coverage_policy: None,
                starts_at: Some(starts),
                ends_at: None,
                data_coverage_starts_at: Some(data_start),
                data_coverage_ends_at: Some(data_end),
                amount_cents: Some(0),
                notes: Some("Seeded demo subscription".to_owned()),
            },
        )
        .await?;
    }

    let tickets = domain::list_tickets_by_account(&state.db, account_id).await?;
    if tickets.is_empty() {
        let customer = users::find_by_username(&state.db, DEV_CUSTOMER_USERNAME)
            .await?
            .expect("customer user seeded");

        domain::create_ticket(
            &state.db,
            account_id,
            customer.id,
            &CreateTicketInput {
                subject: "Demo device check".to_owned(),
                description: Some("Seeded open ticket for development.".to_owned()),
                category: Some(TicketCategory::Device),
                priority: Some(TicketPriority::P3),
                device_id: Some(device.id),
                sim_card_id: None,
            },
        )
        .await?;
    }

    let invoices = domain::list_invoices_by_account(&state.db, account_id).await?;
    if invoices.is_empty() {
        let invoice = domain::create_invoice(
            &state.db,
            account_id,
            &domain::CreateInvoiceInput {
                description: "Annual VisionRoute service".to_owned(),
                amount_cents: 4_500_00,
                currency: Some("PHP".to_owned()),
                status: Some(domain::InvoiceStatus::Sent),
                issued_at: None,
                due_date: Some(Utc::now().date_naive() + Days::new(14)),
                notes: Some("Seeded demo invoice".to_owned()),
            },
        )
        .await?;

        domain::create_payment(
            &state.db,
            account_id,
            &domain::CreatePaymentInput {
                invoice_id: Some(invoice.id),
                amount_cents: 2_000_00,
                currency: Some("PHP".to_owned()),
                method: Some(domain::PaymentMethod::Gcash),
                paid_at: None,
                reference: Some("GCASH-DEMO".to_owned()),
                notes: Some("Partial demo payment".to_owned()),
            },
        )
        .await?;
    }

    let sim_costs = domain::list_sim_data_costs(&state.db).await?;
    if sim_costs.is_empty() {
        let sims = domain::list_sims_by_account(&state.db, account_id).await?;
        let sim_id = sims.first().map(|s| s.id);
        domain::create_sim_data_cost(
            &state.db,
            &domain::CreateSimDataCostInput {
                account_id: Some(account_id),
                sim_card_id: sim_id,
                amount_cents: 1_299_00,
                currency: Some("PHP".to_owned()),
                carrier: Some(SimCarrier::Smart),
                description: "Smart 12GB yearly data load".to_owned(),
                period_start: Some(Utc::now().date_naive()),
                period_end: Some(Utc::now().date_naive() + chrono::Months::new(12)),
                paid_at: None,
                notes: Some("Seeded SIM data cost".to_owned()),
            },
        )
        .await?;
    }

    info!(%account_id, "seeded development domain fixtures");
    Ok(())
}

async fn ensure_user(
    state: &AppState,
    username: &str,
    email: &str,
    password: &str,
    full_name: &str,
    role: UserRole,
    account_id: Option<Uuid>,
) -> anyhow::Result<()> {
    if let Some(existing) = users::find_by_username(&state.db, username).await? {
        if account_id.is_some() && existing.account_id != account_id {
            sqlx::query(
                r#"
                UPDATE users
                SET account_id = $2, updated_at = now()
                WHERE id = $1
                "#,
            )
            .bind(existing.id)
            .bind(account_id)
            .execute(&state.db)
            .await?;
            info!(username, "updated development user account link");
        }
        return Ok(());
    }

    let password_hash = hash_password(password)
        .map_err(|error| anyhow::anyhow!("seed password hash failed: {error}"))?;

    users::insert_user(
        &state.db,
        Uuid::new_v4(),
        username,
        email,
        &password_hash,
        full_name,
        role,
        account_id,
    )
    .await?;

    info!(username, role = role.as_str(), "seeded development user");
    Ok(())
}

/// Back-compat alias.
pub async fn ensure_dev_admin(state: &AppState) -> anyhow::Result<()> {
    ensure_dev_users(state).await
}
