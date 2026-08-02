//! Authorization: permissions, roles, and account scope.
//! Deny by default. Frontend is UX only — backend enforces.

mod policy;
mod scope;

pub use policy::{allows, require_permission, Permission};
pub use scope::{assert_account_access, can_access_account};
