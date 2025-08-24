// src/cdn_app_backend/src/file_store/mod.rs

/// File storage logic
pub mod storage;

/// Chunk splitting and reassembly
pub mod chunking;

/// File metadata management
pub mod metadata;

/// Re-export for easy access
pub use storage::*;
pub use chunking::*;
pub use metadata::*;
