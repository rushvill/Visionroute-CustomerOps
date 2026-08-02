mod admin;
mod auth;
pub mod domain;
pub mod extractors;
mod health;
mod security_headers;

use axum::http::{HeaderValue, Method};
use axum::middleware;
use axum::routing::{get, patch, post};
use axum::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::state::AppState;

/// JSON API body cap (256 KiB) — enough for CRM forms, not file uploads.
const MAX_JSON_BODY_BYTES: usize = 256 * 1024;

pub fn build_router(state: AppState) -> Router {
    let cors = cors_layer(&state.config);

    Router::new()
        .route("/health", get(health::live))
        .route("/ready", get(health::ready))
        .route("/api/v1/health", get(health::api_health))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/logout", post(auth::logout))
        .route("/api/v1/me", get(auth::me))
        .route("/api/v1/admin/users", get(admin::list_users))
        .route("/api/v1/admin/audit-events", get(admin::list_audit_events))
        .route(
            "/api/v1/accounts/{account_id}/access",
            get(admin::account_access),
        )
        .route(
            "/api/v1/signup-requests",
            post(domain::create_signup_request),
        )
        .route(
            "/api/v1/admin/signup-requests",
            get(domain::admin_list_signups),
        )
        .route(
            "/api/v1/admin/signup-requests/{id}/approve",
            post(domain::admin_approve_signup),
        )
        .route(
            "/api/v1/admin/signup-requests/{id}/reject",
            post(domain::admin_reject_signup),
        )
        .route("/api/v1/admin/accounts", get(domain::admin_list_accounts))
        .route("/api/v1/admin/accounts", post(domain::admin_create_account))
        .route(
            "/api/v1/admin/accounts/{id}",
            get(domain::admin_get_account),
        )
        .route(
            "/api/v1/admin/accounts/{account_id}/devices",
            post(domain::admin_create_device),
        )
        .route(
            "/api/v1/admin/accounts/{account_id}/devices",
            get(domain::admin_list_devices),
        )
        .route("/api/v1/admin/sims", get(domain::admin_list_sims))
        .route("/api/v1/admin/sims", post(domain::admin_create_sim))
        .route(
            "/api/v1/admin/sims/{id}/assign",
            post(domain::admin_assign_sim),
        )
        .route(
            "/api/v1/admin/accounts/{account_id}/subscriptions",
            post(domain::admin_create_subscription),
        )
        .route(
            "/api/v1/admin/coverage-expiring",
            get(domain::admin_coverage_expiring),
        )
        .route(
            "/api/v1/admin/subscriptions",
            get(domain::admin_list_subscriptions),
        )
        .route(
            "/api/v1/admin/billing/summary",
            get(domain::admin_billing_summary),
        )
        .route(
            "/api/v1/admin/invoices",
            get(domain::admin_list_invoices).post(domain::admin_create_invoice),
        )
        .route(
            "/api/v1/admin/payments",
            get(domain::admin_list_payments).post(domain::admin_create_payment),
        )
        .route(
            "/api/v1/admin/sim-data-costs",
            get(domain::admin_list_sim_data_costs).post(domain::admin_create_sim_data_cost),
        )
        .route("/api/v1/admin/tickets", get(domain::admin_list_tickets))
        .route(
            "/api/v1/admin/tickets/{id}",
            patch(domain::admin_patch_ticket),
        )
        .route(
            "/api/v1/privacy-requests",
            post(domain::create_privacy_request),
        )
        .route(
            "/api/v1/me/privacy-requests",
            post(domain::me_create_privacy_request),
        )
        .route(
            "/api/v1/admin/privacy-requests",
            get(domain::admin_list_privacy_requests),
        )
        .route(
            "/api/v1/admin/privacy-requests/{id}",
            patch(domain::admin_patch_privacy_request),
        )
        .route("/api/v1/me/account", get(domain::me_account))
        .route("/api/v1/me/devices", get(domain::me_devices))
        .route("/api/v1/me/sims", get(domain::me_sims))
        .route("/api/v1/me/subscription", get(domain::me_subscription))
        .route("/api/v1/me/invoices", get(domain::me_invoices))
        .route("/api/v1/me/payments", get(domain::me_payments))
        .route("/api/v1/me/tickets", get(domain::me_list_tickets))
        .route("/api/v1/me/tickets", post(domain::me_create_ticket))
        .layer(RequestBodyLimitLayer::new(MAX_JSON_BODY_BYTES))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            security_headers::security_headers_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

fn cors_layer(config: &Config) -> CorsLayer {
    let origin = config.frontend_origin.clone();
    CorsLayer::new()
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
            axum::http::header::COOKIE,
            axum::http::header::ORIGIN,
        ])
        .allow_origin(AllowOrigin::predicate(move |value: &HeaderValue, _| {
            value
                .to_str()
                .map(|candidate| candidate == origin)
                .unwrap_or(false)
        }))
}
