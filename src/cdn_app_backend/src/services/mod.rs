// src/cdn_app_backend/src/services/mod.rs

/// File service: handles high-level file operations
pub mod file_service;

/// Re-export for easy access
pub use file_service::*;
