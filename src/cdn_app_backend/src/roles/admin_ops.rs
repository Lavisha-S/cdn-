// src/cdn_app_backend/src/roles/admin_ops.rs

use crate::auth::checks;
use crate::services::file_service::FileService;
use crate::types::{DomainError, FileMetadata, User};

/// Admin operations
pub struct AdminOps<'a> {
    pub file_service: &'a FileService,
}

impl<'a> AdminOps<'a> {
    /// Initialize AdminOps
    pub fn new(file_service: &'a FileService) -> Self {
        Self { file_service }
    }

    /// Delete a file
    pub fn delete_file(&self, user: &User, file_id: &str) -> Result<(), DomainError> {
        checks::check_admin(user)?;
        self.file_service.delete_file(user, file_id)
    }

    /// List all files (admin can see all)
    pub fn list_all_files(&self, user: &User) -> Result<Vec<FileMetadata>, DomainError> {
        checks::check_admin(user)?;
        Ok(self.file_service.list_files(None))
    }

    /// Deactivate a file
    pub fn deactivate_file(&self, user: &User, file_meta: &mut FileMetadata) -> Result<(), DomainError> {
        checks::check_admin(user)?;
        file_meta.is_active = false;
        Ok(())
    }

    /// Activate a file
    pub fn activate_file(&self, user: &User, file_meta: &mut FileMetadata) -> Result<(), DomainError> {
        checks::check_admin(user)?;
        file_meta.is_active = true;
        Ok(())
    }

    /// Manage users - placeholder for user management
    pub fn manage_user(&self, user: &User, target_user_id: &str, action: &str) -> Result<(), DomainError> {
        checks::check_admin(user)?;
        // TODO: Implement user management (suspend, role change, delete)
        println!("Admin {} performs '{}' on user {}", user.username, action, target_user_id);
        Ok(())
    }
}
