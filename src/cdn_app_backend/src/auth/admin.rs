// auth/admin.rs

use crate::types::user::User;
use crate::auth::roles::{Action};
use crate::auth::checks;
use crate::errors::error::BackendError;

/// Check if the user is allowed to manage users (assign/revoke roles)
pub fn can_manage_users(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::ManageUsers)
}

/// Check if the user can assign roles
pub fn can_assign_role(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::AssignRole)
}

/// Check if the user can revoke roles
pub fn can_revoke_role(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::RevokeRole)
}

/// Check if the user can delete any file in the system
pub fn can_delete_file(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::DeleteFile)
}

/// Combined check: admin full access for management operations
pub fn ensure_admin_full_access(user: &User) -> Result<(), BackendError> {
    // Enforce multiple admin actions at once
    checks::enforce_multiple_actions(user, vec![
        Action::DeleteFile,
        Action::ManageUsers,
        Action::AssignRole,
        Action::RevokeRole,
    ])
}

/// Optional: additional admin-only validations can be added here
