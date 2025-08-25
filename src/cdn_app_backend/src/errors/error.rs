// src/cdn_app_backend/src/errors/error.rs

use std::fmt;

/// Backend / Canister-level errors
#[derive(Debug, Clone)]
pub enum BackendError {
    // Network / runtime errors
    NetworkError(String),
    StorageUnavailable(String),
    UnexpectedFailure(String),

    // Domain-wrapped errors
    DomainError(String),

    // Permission / auth errors
    Unauthorized(String),
    Forbidden(String),

    // Validation/data errors
    InvalidData(String),
    ValidationError(String),
    DuplicateFile,
    FileNotFound,

    // General backend error
    Other(String),
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            BackendError::StorageUnavailable(msg) => write!(f, "Storage unavailable: {}", msg),
            BackendError::UnexpectedFailure(msg) => write!(f, "Unexpected failure: {}", msg),
            BackendError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            BackendError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            BackendError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            BackendError::Other(msg) => write!(f, "Backend error: {}", msg),
        }
    }
}

impl std::error::Error for BackendError {}

/// Helper conversions
impl From<String> for BackendError {
    fn from(msg: String) -> Self {
        BackendError::Other(msg)
    }
}

impl From<&str> for BackendError {
    fn from(msg: &str) -> Self {
        BackendError::Other(msg.to_string())
    }
}
