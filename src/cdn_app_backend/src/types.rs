use serde_bytes::ByteBuf;
// src/cdn_app_backend/types.rs

use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::fmt;

/// User roles in the system
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Admin,    // Full access: manage users, upload, delete, download
    Uploader, // Can upload files
    Viewer,   // Can only list and download files
}

/// Permissions mapped to roles
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    ManageUsers,  // Add/remove roles
    UploadFile,   // Upload files
    DeleteFile,   // Delete files
    DownloadFile, // Download files
    ListFiles,    // View available files
}

/// Metadata about a stored file
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub uploaded_by: String, // Principal as text
    pub uploaded_at: u64,    // Timestamp (nanoseconds since epoch)
}

/// Unified response type for API calls
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum ApiResponse<T> {
    Success(T),
    Error(String),
}

/// Utility to format roles nicely
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Uploader => write!(f, "Uploader"),
            Role::Viewer => write!(f, "Viewer"),
        }
    }
}
