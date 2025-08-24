// src/cdn_app_backend/src/roles/viewer_ops.rs

use crate::auth::checks;
use crate::services::file_service::FileService;
use crate::types::{DomainError, FileMetadata, User};

/// Viewer operations
pub struct ViewerOps<'a> {
    pub file_service: &'a FileService,
}

impl<'a> ViewerOps<'a> {
    /// Initialize ViewerOps
    pub fn new(file_service: &'a FileService) -> Self {
        Self { file_service }
    }

    /// Download a file
    pub fn download_file(&self, user: &User, file_id: &str) -> Result<Vec<u8>, DomainError> {
        checks::check_viewer(user)?;
        self.file_service.download_file(user, file_id)
    }

    /// List all active files
    pub fn list_active_files(&self, user: &User) -> Result<Vec<FileMetadata>, DomainError> {
        checks::check_viewer(user)?;
        Ok(self.file_service.list_files(None)
            .into_iter()
            .filter(|f| f.is_active)
            .collect())
    }
}
