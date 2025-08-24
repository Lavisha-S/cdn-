// auth/roles.rs
use crate::types::user::User;
use crate::errors::error::BackendError;

/// System roles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    Publisher,
    Viewer,
}

/// Actions that require permission checks
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    UploadFile,
    DownloadFile,
    DeleteFile,
    ViewMetadata,
    ManageUsers,
    AssignRole,
    RevokeRole,
}

/// Map actions to allowed roles
pub fn allowed_roles(action: &Action) -> Vec<Role> {
    match action {
        Action::UploadFile => vec![Role::Publisher, Role::Admin],
        Action::DownloadFile => vec![Role::Viewer, Role::Publisher, Role::Admin],
        Action::DeleteFile => vec![Role::Admin],
        Action::ViewMetadata => vec![Role::Viewer, Role::Publisher, Role::Admin],
        Action::ManageUsers => vec![Role::Admin],
        Action::AssignRole => vec![Role::Admin],
        Action::RevokeRole => vec![Role::Admin],
    }
}

/// Check if a user has permission for a given action
pub fn check_permission(user: &User, action: &Action) -> Result<(), BackendError> {
    let allowed = allowed_roles(action);
    if allowed.contains(&user.role) {
        Ok(())
    } else {
        Err(BackendError::Unauthorized(format!(
            "User {:?} with role {:?} cannot perform {:?}",
            user.id, user.role, action
        )))
    }
}

/// Convenience methods for User role checks
impl User {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn is_publisher(&self) -> bool {
        self.role == Role::Publisher
    }

    pub fn is_viewer(&self) -> bool {
        self.role == Role::Viewer
    }
}
