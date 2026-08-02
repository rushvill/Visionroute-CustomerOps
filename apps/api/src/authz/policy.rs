//! Role → permission matrix (see docs/security/permissions-matrix.md).

use serde_json::json;

use crate::audit;
use crate::error::AppError;
use crate::state::AppState;
use crate::users::{UserRole, UserRow};

/// Capability keys aligned with the permissions matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    SignupCreate,
    SignupReview,
    AccountReadOwn,
    AccountReadAny,
    AccountUpdateAny,
    DeviceReadOwn,
    DeviceManage,
    SimInventoryRead,
    SimReadOwn,
    SimAssign,
    SubscriptionReadOwn,
    BillingReadOwn,
    BillingManage,
    TicketOwn,
    TicketManageAll,
    TicketInternalNotes,
    UsersManage,
    AuditRead,
    PrivacyManage,
}

impl Permission {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignupCreate => "signup.create",
            Self::SignupReview => "signup.review",
            Self::AccountReadOwn => "account.read_own",
            Self::AccountReadAny => "account.read_any",
            Self::AccountUpdateAny => "account.update_any",
            Self::DeviceReadOwn => "device.read_own",
            Self::DeviceManage => "device.manage",
            Self::SimInventoryRead => "sim.inventory_read",
            Self::SimReadOwn => "sim.read_own",
            Self::SimAssign => "sim.assign",
            Self::SubscriptionReadOwn => "subscription.read_own",
            Self::BillingReadOwn => "billing.read_own",
            Self::BillingManage => "billing.manage",
            Self::TicketOwn => "ticket.own",
            Self::TicketManageAll => "ticket.manage_all",
            Self::TicketInternalNotes => "ticket.internal_notes",
            Self::UsersManage => "users.manage",
            Self::AuditRead => "audit.read",
            Self::PrivacyManage => "privacy.manage",
        }
    }
}

/// Deny-by-default role grants.
pub fn allows(role: UserRole, permission: Permission) -> bool {
    match role {
        UserRole::Admin => true,
        UserRole::Operator => !matches!(permission, Permission::UsersManage),
        UserRole::Customer => matches!(
            permission,
            Permission::AccountReadOwn
                | Permission::DeviceReadOwn
                | Permission::SimReadOwn
                | Permission::SubscriptionReadOwn
                | Permission::BillingReadOwn
                | Permission::TicketOwn
        ),
        UserRole::Viewer => matches!(
            permission,
            Permission::AccountReadOwn
                | Permission::DeviceReadOwn
                | Permission::SimReadOwn
                | Permission::SubscriptionReadOwn
                | Permission::BillingReadOwn
        ),
    }
}

/// Enforce a permission; on deny, write an audit event and return Forbidden.
pub async fn require_permission(
    state: &AppState,
    user: &UserRow,
    permission: Permission,
) -> Result<(), AppError> {
    if allows(user.role, permission) {
        return Ok(());
    }

    let _ = audit::record(
        state,
        Some(user.id),
        user.account_id,
        "authz",
        user.id,
        "permission_denied",
        "Authorization denied",
        json!({
            "permission": permission.as_str(),
            "role": user.role.as_str(),
        }),
    )
    .await;

    tracing::warn!(
        user_id = %user.id,
        role = user.role.as_str(),
        permission = permission.as_str(),
        "permission denied"
    );

    Err(AppError::Forbidden)
}

/// Anonymous signup create is allowed without a session (used when signup lands in Phase 4).
#[allow(dead_code)]
pub fn anonymous_allows(permission: Permission) -> bool {
    matches!(permission, Permission::SignupCreate)
}

#[cfg(test)]
mod tests {
    use super::{allows, Permission};
    use crate::users::UserRole;

    #[test]
    fn admin_can_manage_users() {
        assert!(allows(UserRole::Admin, Permission::UsersManage));
        assert!(allows(UserRole::Admin, Permission::AuditRead));
    }

    #[test]
    fn customer_cannot_manage_users_or_audit() {
        assert!(!allows(UserRole::Customer, Permission::UsersManage));
        assert!(!allows(UserRole::Customer, Permission::AuditRead));
        assert!(!allows(UserRole::Customer, Permission::SimInventoryRead));
        assert!(allows(UserRole::Customer, Permission::TicketOwn));
    }

    #[test]
    fn operator_cannot_manage_users_but_can_audit() {
        assert!(!allows(UserRole::Operator, Permission::UsersManage));
        assert!(allows(UserRole::Operator, Permission::AuditRead));
        assert!(allows(UserRole::Operator, Permission::SimAssign));
    }

    #[test]
    fn viewer_is_read_only_own() {
        assert!(allows(UserRole::Viewer, Permission::AccountReadOwn));
        assert!(!allows(UserRole::Viewer, Permission::TicketOwn));
        assert!(!allows(UserRole::Viewer, Permission::DeviceManage));
    }
}
