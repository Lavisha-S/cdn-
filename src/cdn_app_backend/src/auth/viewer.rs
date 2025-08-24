// auth/viewer.rs

use crate::types::user::User;
use crate::auth::roles::Action;
use crate::auth::checks;
use crate::errors::error::BackendError;

/// Check if the user can view/download files
pub fn can_download_file(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::DownloadFile)
}

/// Check if the user can view file metadata
pub fn can_view_metadata(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::ViewMetadata)
}

/// Combined viewer actions for convenience
pub fn ensure_viewer_full_access(user: &User) -> Result<(), BackendError> {
    checks::enforce_multiple_actions(user, vec![
        Action::DownloadFile,
        Action::ViewMetadata,
    ])
}

/// Optional: additional viewer-specific validations can be added here
/// e.g., access limits, time-limited downloads, or folder-based restrictions
