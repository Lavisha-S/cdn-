// src/cdn_app_backend/src/main.rs

mod auth;
mod config;
mod errors;
mod file_store;
mod roles;
mod services;
mod types;
mod utils;

use crate::auth::roles::Role;
use crate::file_store::storage::FileStorage;
use crate::roles::{admin_ops::AdminOps, publisher_ops::PublisherOps, viewer_ops::ViewerOps};
use crate::services::file_service::FileService;
use crate::types::User;

fn main() {
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
        id: "admin-1".to_string(),
        username: "superadmin".to_string(),
        role: Role::Admin,
    };

    let publisher_user = User {
        id: "pub-1".to_string(),
        username: "publisher1".to_string(),
        role: Role::Publisher,
    };

    let viewer_user = User {
        id: "viewer-1".to_string(),
        username: "viewer1".to_string(),
        role: Role::Viewer,
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
