// src/cdn_app_backend/src/roles/publisher_ops.rs

use crate::auth::checks;
use crate::services::file_service::FileService;
use crate::types::{DomainError, FileMetadata, User};

/// Publisher operations
pub struct PublisherOps<'a> {
    pub file_service: &'a FileService,
}

impl<'a> PublisherOps<'a> {
    /// Initialize PublisherOps
    pub fn new(file_service: &'a FileService) -> Self {
        Self { file_service }
    }

    /// Upload a file
    pub fn upload_file(
        &self,
        user: &User,
        filename: String,
        data: Vec<u8>,
    ) -> Result<FileMetadata, DomainError> {
        checks::check_publisher(user)?;
        self.file_service.upload_file(user, filename, data)
    }

    /// List files owned by the publisher
    pub fn list_own_files(&self, user: &User) -> Result<Vec<FileMetadata>, DomainError> {
        checks::check_publisher(user)?;
        Ok(self.file_service.list_files(Some(&user.id.to_string())))
    }

    /// Deactivate own file
    pub fn deactivate_own_file(
        &self,
        user: &User,
        file_meta: &mut FileMetadata,
    ) -> Result<(), DomainError> {
        checks::check_publisher(user)?;
        if file_meta.owner_id != user.id.to_string() {
            return Err(DomainError::Forbidden(format!(
                "User {} cannot deactivate file {}",
                user.username, file_meta.id
            )));
        }
        file_meta.is_active = false;
        Ok(())
    }

    /// Activate own file
    pub fn activate_own_file(
        &self,
        user: &User,
        file_meta: &mut FileMetadata,
    ) -> Result<(), DomainError> {
        checks::check_publisher(user)?;
        if file_meta.owner_id != user.id.to_string() {
            return Err(DomainError::Forbidden(format!(
                "User {} cannot activate file {}",
                user.username, file_meta.id
            )));
        }
        file_meta.is_active = true;
        Ok(())
    }
}
