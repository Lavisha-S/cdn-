// src/cdn_app_backend/src/lib.rs

/// Top-level re-exports for the CDN backend


pub mod auth;
pub mod config;
pub mod errors;
pub mod file_store;
pub mod ic;
pub mod roles;
pub mod services;
pub mod types;
pub mod utils;


/// Re-export all modules for easy access
pub use auth::*;
pub use types::*;
pub use file_store::*;
pub use roles::*;
pub use services::*;
pub use utils::*;
pub use errors::*;

use crate::types::user::Role;
use crate::file_store::storage::FileStorage;
use crate::roles::{admin_ops::AdminOps, publisher_ops::PublisherOps, viewer_ops::ViewerOps};
use crate::services::file_service::FileService;
use crate::types::User;
use uuid::Uuid;

pub fn run_demo() {
	println!("Starting CDN backend MVP...");

	// Initialize core services
	let storage = FileStorage::new();
	let file_service = FileService::new(storage);

	// Initialize role operations
	let admin_ops = AdminOps::new(&file_service);
	let publisher_ops = PublisherOps::new(&file_service);
	let viewer_ops = ViewerOps::new(&file_service);

	// Sample users
	let admin_user = User {
		id: Uuid::new_v4(),
		username: "superadmin".to_string(),
		role: Role::Admin,
		email: Some("admin@example.com".to_string()),
		is_active: true,
	};

	let publisher_user = User {
		id: Uuid::new_v4(),
		username: "publisher1".to_string(),
		role: Role::Publisher,
		email: Some("publisher1@example.com".to_string()),
		is_active: true,
	};

	let viewer_user = User {
		id: Uuid::new_v4(),
		username: "viewer1".to_string(),
		role: Role::Viewer,
		email: Some("viewer1@example.com".to_string()),
		is_active: true,
	};

	// === Demo workflow ===
	// Publisher uploads a file
	let file_data = b"Hello, this is a test file for CDN MVP.".to_vec();
	let uploaded_file = publisher_ops
		.upload_file(&publisher_user, "test_file.txt".to_string(), file_data)
		.expect("Publisher should be able to upload file");

	println!("File uploaded: {:?}", uploaded_file);

	// Viewer downloads the file
	let downloaded_data = viewer_ops
		.download_file(&viewer_user, &uploaded_file.id)
		.expect("Viewer should be able to download active file");

	println!(
		"Viewer downloaded file content: {}",
		String::from_utf8(downloaded_data).unwrap()
	);

	// Admin deletes the file
	admin_ops
		.delete_file(&admin_user, &uploaded_file.id)
		.expect("Admin should be able to delete file");

	println!("File deleted by admin.");
}
