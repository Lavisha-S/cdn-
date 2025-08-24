// src/cdn_app_backend/src/auth/mod.rs

/// Role definitions and permissions
pub mod roles;

/// Central permission checks
pub mod checks;

/// Admin-specific auth helpers
pub mod admin;

/// Publisher-specific auth helpers
pub mod publisher;

/// Viewer-specific auth helpers
pub mod viewer;

/// Re-export for easy access
pub use roles::*;
pub use checks::*;
pub use admin::*;
pub use publisher::*;
pub use viewer::*;
