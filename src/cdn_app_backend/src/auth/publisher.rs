// auth/publisher.rs

use crate::types::user::User;
use crate::auth::roles::Action;
use crate::auth::checks;
use crate::errors::error::BackendError;

/// Check if the user can upload a file
pub fn can_upload_file(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::UploadFile)
}

/// Check if the user can view file metadata
pub fn can_view_metadata(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::ViewMetadata)
}

/// Check if the user can download files (own or permitted files)
pub fn can_download_file(user: &User) -> Result<(), BackendError> {
    checks::enforce_permission(user, Action::DownloadFile)
}

/// Combined publisher actions for convenience
pub fn ensure_publisher_full_access(user: &User) -> Result<(), BackendError> {
    checks::enforce_multiple_actions(user, vec![
        Action::UploadFile,
        Action::DownloadFile,
        Action::ViewMetadata,
    ])
}

/// Optional: additional publisher-specific validations can be added here
/// e.g., limit upload size, quota checks, or specific folder access restrictions
