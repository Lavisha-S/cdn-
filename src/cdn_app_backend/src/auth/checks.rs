// src/cdn_app_backend/src/auth/checks.rs

use crate::types::user::Role;
use crate::types::User;
use crate::errors::BackendError;

/// Checks if the user has the required role.
/// Returns Ok(()) if allowed, Err otherwise.
pub fn require_role(user: &User, required_role: Role) -> Result<(), BackendError> {
    match required_role {
        Role::Admin => {
            if user.role == Role::Admin {
                Ok(())
            } else {
                Err(BackendError::Unauthorized("Admin privileges required".into()))
            }
        }
        Role::Publisher => {
            if user.role == Role::Publisher || user.role == Role::Admin {
                // Admin can act as Publisher too
                Ok(())
            } else {
                Err(BackendError::Unauthorized("Publisher privileges required".into()))
            }
        }
        Role::Viewer => {
            // All roles can act as Viewer
            if user.role == Role::Viewer || user.role == Role::Publisher || user.role == Role::Admin {
                Ok(())
            } else {
                Err(BackendError::Unauthorized("Viewer privileges required".into()))
            }
        }
    }
}

/// Helper to check if user is Admin
pub fn is_admin(user: &User) -> bool {
    user.role == Role::Admin
}

/// Helper to check if user is Publisher
pub fn is_publisher(user: &User) -> bool {
    matches!(user.role, Role::Publisher | Role::Admin)
}

/// Helper to check if user is Viewer
pub fn is_viewer(user: &User) -> bool {
    matches!(user.role, Role::Viewer | Role::Publisher | Role::Admin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::user::Role;
    use crate::types::User;

    fn make_user(role: Role) -> User {
        User {
            username: "test_user".into(),
            role,
        }
    }

    #[test]
    fn test_admin_check() {
        let admin = make_user(Role::Admin);
        let pub_user = make_user(Role::Publisher);
        assert!(require_role(&admin, Role::Admin).is_ok());
        assert!(require_role(&pub_user, Role::Admin).is_err());
    }

    #[test]
    fn test_publisher_check() {
        let admin = make_user(Role::Admin);
        let pub_user = make_user(Role::Publisher);
        let viewer = make_user(Role::Viewer);

        assert!(require_role(&admin, Role::Publisher).is_ok());
        assert!(require_role(&pub_user, Role::Publisher).is_ok());
        assert!(require_role(&viewer, Role::Publisher).is_err());
    }

    #[test]
    fn test_viewer_check() {
        let admin = make_user(Role::Admin);
        let pub_user = make_user(Role::Publisher);
        let viewer = make_user(Role::Viewer);

        assert!(require_role(&admin, Role::Viewer).is_ok());
        assert!(require_role(&pub_user, Role::Viewer).is_ok());
        assert!(require_role(&viewer, Role::Viewer).is_ok());
    }
}
