//! Phase 4 core CRM domain services.

mod accounts;
mod billing;
pub mod devices;
mod plans;
pub mod privacy;
pub mod signup;
pub mod sims;
mod subscriptions;
mod tickets;
pub mod types;

pub use accounts::{
    create as create_account, get_by_id as get_account, list as list_accounts, next_account_code,
    AccountRow, CreateAccountInput,
};
pub use billing::{
    create_invoice, create_payment, create_sim_data_cost, list_invoices, list_invoices_by_account,
    list_open_invoices, list_payments, list_payments_by_account, list_sim_data_costs, totals_summary,
    CreateInvoiceInput, CreatePaymentInput, CreateSimDataCostInput, InvoiceRow, PaymentRow,
    SimDataCostRow,
};
pub use devices::{
    create as create_device, list_by_account as list_devices_by_account, CreateDeviceInput,
    DeviceRow,
};
pub use plans::{ensure_basic_plan, find_by_code, find_by_id as find_plan_by_id, PlanRow};
pub use privacy::{
    create_request as create_privacy_request, list_requests as list_privacy_requests,
    update_request as update_privacy_request, CreatePrivacyRequestInput, PrivacyRequestRow,
    UpdatePrivacyRequestInput, PRIVACY_NOTICE_VERSION,
};
pub use signup::{
    approve as approve_signup, create_request as create_signup_request, get_by_id as get_signup,
    list as list_signups, reject as reject_signup, ApproveSignupInput, CreateSignupInput,
    RejectSignupInput, SignupRequestRow,
};
pub use sims::{
    assign as assign_sim, create_inventory as create_sim, list_all as list_sims_all,
    list_by_account as list_sims_by_account, AssignSimInput, CreateSimInput, SimRow,
};
pub use subscriptions::{
    create as create_subscription, get_active_for_account, list_all as list_subscriptions,
    list_expiring, CreateSubscriptionInput, SubscriptionRow,
};
pub use tickets::{
    create as create_ticket, get_by_id as get_ticket, list_all as list_tickets_all,
    list_by_account as list_tickets_by_account, next_ticket_number, update as update_ticket,
    CreateTicketInput, TicketRow, UpdateTicketInput,
};
pub use types::{
    InvoiceStatus, PaymentMethod, PrivacyRequestStatus, PrivacyRequestType, SimCarrier,
};
