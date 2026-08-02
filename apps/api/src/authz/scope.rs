//! Object-level / tenant scope checks.

use serde_json::json;
use uuid::Uuid;

use crate::audit;
use crate::error::AppError;
use crate::state::AppState;
use crate::users::{UserRole, UserRow};

/// Staff (admin/operator) may access any account; customers/viewers only their own.
pub fn can_access_account(user: &UserRow, account_id: Uuid) -> bool {
    match user.role {
        UserRole::Admin | UserRole::Operator => true,
        UserRole::Customer | UserRole::Viewer => user.account_id == Some(account_id),
    }
}

/// Enforce account scope; on deny, audit and return Forbidden (not NotFound — caller may map).
pub async fn assert_account_access(
    state: &AppState,
    user: &UserRow,
    account_id: Uuid,
) -> Result<(), AppError> {
    if can_access_account(user, account_id) {
        return Ok(());
    }

    let _ = audit::record(
        state,
        Some(user.id),
        user.account_id,
        "account",
        account_id,
        "scope_denied",
        "Account scope denied",
        json!({
            "requested_account_id": account_id,
            "role": user.role.as_str(),
        }),
    )
    .await;

    tracing::warn!(
        user_id = %user.id,
        %account_id,
        "account scope denied"
    );

    Err(AppError::Forbidden)
}

#[cfg(test)]
mod tests {
    use super::can_access_account;
    use crate::users::{UserRole, UserRow};
    use chrono::Utc;
    use uuid::Uuid;

    fn user(role: UserRole, account_id: Option<Uuid>) -> UserRow {
        UserRow {
            id: Uuid::new_v4(),
            account_id,
            username: "u".into(),
            email: "u@example.com".into(),
            password_hash: "x".into(),
            full_name: "U".into(),
            phone: None,
            role,
            is_active: true,
            last_login_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn customer_only_own_account() {
        let own = Uuid::new_v4();
        let other = Uuid::new_v4();
        let customer = user(UserRole::Customer, Some(own));
        assert!(can_access_account(&customer, own));
        assert!(!can_access_account(&customer, other));
    }

    #[test]
    fn admin_any_account() {
        let admin = user(UserRole::Admin, None);
        assert!(can_access_account(&admin, Uuid::new_v4()));
    }
}
