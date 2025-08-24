
// src/cdn_app_backend/src/roles/mod.rs

/// Admin-specific operations (manage users, delete files)
pub mod admin_ops;

/// Publisher-specific operations (upload files, view own files)
pub mod publisher_ops;

/// Viewer-specific operations (download/view files)
pub mod viewer_ops;

/// Re-export for easy access
pub use admin_ops::*;
pub use publisher_ops::*;
pub use viewer_ops::*;
