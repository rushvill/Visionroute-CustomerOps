//! Shared domain enums matching PostgreSQL types.

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "account_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum AccountStatus {
    Pending,
    Active,
    Suspended,
    Churned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "signup_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum SignupStatus {
    New,
    Reviewing,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "device_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum DeviceStatus {
    PendingInstall,
    Active,
    Inactive,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "sim_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum SimStatus {
    Inventory,
    Assigned,
    Active,
    Suspended,
    Exhausted,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "sim_carrier", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum SimCarrier {
    Smart,
    Globe,
    Tnt,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "subscription_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionStatus {
    Trial,
    Active,
    PastDue,
    Paused,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "coverage_policy", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum CoveragePolicy {
    ShoulderedByUs,
    CustomerPaid,
    Undecided,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum TicketStatus {
    Open,
    InProgress,
    WaitingCustomer,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "ticket_priority", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum TicketPriority {
    #[serde(rename = "p1")]
    P1,
    #[serde(rename = "p2")]
    P2,
    #[serde(rename = "p3")]
    P3,
    #[serde(rename = "p4")]
    P4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "ticket_category", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum TicketCategory {
    Device,
    SimData,
    Billing,
    Login,
    Install,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "invoice_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Partial,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_method", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    Gcash,
    Maya,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "privacy_request_type", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum PrivacyRequestType {
    Access,
    Rectification,
    Erasure,
    Restriction,
    Portability,
    Objection,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "privacy_request_status", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum PrivacyRequestStatus {
    Received,
    InProgress,
    Completed,
    Rejected,
}
