// src/cdn_app_backend/src/types/error.rs

use std::fmt;

/// Domain-specific errors for file operations, users, and validation
#[derive(Debug, Clone)]
pub enum DomainError {
    // User-related errors
    UserNotFound(String),
    UserAlreadyExists(String),
    InvalidRole(String),

    // File-related errors
    FileNotFound(String),
    FileAlreadyExists(String),
    InvalidFileMetadata(String),
    ChunkMissing(u64), // missing chunk number
    ChunkOutOfOrder(u64),

    // Validation errors
    InvalidInput(String),
    UnauthorizedAccess(String),

    // Permission errors
    Forbidden(String),

    // General domain error
    Other(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::UserNotFound(msg) => write!(f, "User not found: {}", msg),
            DomainError::UserAlreadyExists(msg) => write!(f, "User already exists: {}", msg),
            DomainError::InvalidRole(msg) => write!(f, "Invalid role: {}", msg),
            DomainError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            DomainError::FileAlreadyExists(msg) => write!(f, "File already exists: {}", msg),
            DomainError::InvalidFileMetadata(msg) => write!(f, "Invalid file metadata: {}", msg),
            DomainError::ChunkMissing(n) => write!(f, "Missing file chunk: {}", n),
            DomainError::ChunkOutOfOrder(n) => write!(f, "File chunk out of order: {}", n),
            DomainError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            DomainError::UnauthorizedAccess(msg) => write!(f, "Unauthorized access: {}", msg),
            DomainError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
