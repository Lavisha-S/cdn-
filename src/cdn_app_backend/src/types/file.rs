// types/file.rs

use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use crate::types::user::Role;
use crate::errors::error::BackendError;

/// Metadata for a file stored in the CDN
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct FileMetadata {
    pub id: String,               // Unique file identifier (UUID or hash)
    pub owner: Principal,         // Owner of the file (uploader)
    pub owner_id: String,         // Owner's user id (for logic)
    pub filename: String,         // Original file name
    pub size: u64,                // File size in bytes
    pub mime_type: String,        // MIME type (e.g., application/pdf)
    pub uploaded_at: u64,         // Timestamp of upload
    pub roles_allowed: Vec<Role>, // Roles allowed to access this file
    pub chunk_count: u32,         // Number of chunks if file is split
    pub is_active: bool,          // File active status
    pub file_hash: Option<String>,// Optional file hash
}

/// Represents a file chunk
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub struct FileChunk {
    pub file_id: String,     // Associated file ID
    pub index: u32,          // Chunk index (0..n)
    pub data: Vec<u8>,       // Raw chunk data
}

/// Methods for FileMetadata
impl FileMetadata {
    /// Create new file metadata
    pub fn new(
        id: String,
        owner: Principal,
        filename: String,
        size: u64,
        mime_type: String,
        uploaded_at: u64,
        roles_allowed: Vec<Role>,
        chunk_count: u32,
    ) -> Self {
        Self {
            id,
            owner,
            owner_id: String::new(),
            filename,
            size,
            mime_type,
            uploaded_at,
            roles_allowed,
            chunk_count,
            is_active: true,
            file_hash: None,
        }
    }

    /// Check if a user has access to this file
    pub fn can_access(&self, user_role: &Role, user_id: &Principal) -> bool {
        // Owner always has access
        if &self.owner == user_id {
            return true;
        }
        // Admin always has access
        if *user_role == Role::Admin {
            return true;
        }
        // Check role permissions
        self.roles_allowed.contains(user_role)
    }

    /// Validate file metadata
    pub fn validate(&self) -> Result<(), BackendError> {
        if self.filename.trim().is_empty() {
            return Err(BackendError::InvalidData("Filename cannot be empty".into()));
        }
        if self.size == 0 {
            return Err(BackendError::InvalidData("File size cannot be zero".into()));
        }
        if self.chunk_count == 0 {
            return Err(BackendError::InvalidData("Chunk count must be at least 1".into()));
        }
        Ok(())
    }
}
