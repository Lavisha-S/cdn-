// src/cdn_app_backend/src/tests/integration_tests.rs

use crate::roles::{admin_ops::AdminOps, publisher_ops::PublisherOps, viewer_ops::ViewerOps};
use crate::services::file_service::FileService;
use crate::file_store::storage::FileStorage;
use crate::types::{User, Role};
use crate::types::DomainError;

#[test]
fn test_publisher_upload_and_list() {
    let storage = FileStorage::new();
    let file_service = FileService::new(storage);
    let publisher_ops = PublisherOps::new(&file_service);

    let publisher = User {
        id: "pub-1".to_string(),
        username: "publisher1".to_string(),
        role: Role::Publisher,
    };

    let file_data = b"Publisher file content".to_vec();
    let file_meta = publisher_ops
        .upload_file(&publisher, "pub_file.txt".to_string(), file_data.clone())
        .expect("Publisher upload should succeed");

    assert_eq!(file_meta.filename, "pub_file.txt");

    let files = publisher_ops.list_own_files(&publisher).unwrap();
    assert_eq!(files.len(), 1);
}

#[test]
fn test_viewer_download_active_file() {
    let storage = FileStorage::new();
    let file_service = FileService::new(storage);
    let publisher_ops = PublisherOps::new(&file_service);
    let viewer_ops = ViewerOps::new(&file_service);

    let publisher = User {
        id: "pub-2".to_string(),
        username: "publisher2".to_string(),
        role: Role::Publisher,
    };

    let viewer = User {
        id: "viewer-1".to_string(),
        username: "viewer1".to_string(),
        role: Role::Viewer,
    };

    let data = b"Test download file".to_vec();
    let file_meta = publisher_ops
        .upload_file(&publisher, "download_file.txt".to_string(), data.clone())
        .unwrap();

    let downloaded_data = viewer_ops
        .download_file(&viewer, &file_meta.id)
        .unwrap();
    assert_eq!(downloaded_data, data);
}

#[test]
fn test_admin_delete_file() {
    let storage = FileStorage::new();
    let file_service = FileService::new(storage);
    let publisher_ops = PublisherOps::new(&file_service);
    let admin_ops = AdminOps::new(&file_service);
    let viewer_ops = ViewerOps::new(&file_service);

    let admin = User {
        id: "admin-1".to_string(),
        username: "admin1".to_string(),
        role: Role::Admin,
    };

    let publisher = User {
        id: "pub-3".to_string(),
        username: "publisher3".to_string(),
        role: Role::Publisher,
    };

    let viewer = User {
        id: "viewer-2".to_string(),
        username: "viewer2".to_string(),
        role: Role::Viewer,
    };

    let data = b"File to delete".to_vec();
    let file_meta = publisher_ops
        .upload_file(&publisher, "delete_file.txt".to_string(), data.clone())
        .unwrap();

    // Admin deletes
    admin_ops.delete_file(&admin, &file_meta.id).unwrap();

    // Viewer cannot download deleted file
    let result = viewer_ops.download_file(&viewer, &file_meta.id);
    assert!(matches!(result, Err(DomainError::FileNotFound(_))));
}

#[test]
fn test_unauthorized_access() {
    let storage = FileStorage::new();
    let file_service = FileService::new(storage);
    let publisher_ops = PublisherOps::new(&file_service);
    let viewer_ops = ViewerOps::new(&file_service);

    let viewer = User {
        id: "viewer-3".to_string(),
        username: "viewer3".to_string(),
        role: Role::Viewer,
    };

    let data = b"Unauthorized upload".to_vec();
    let upload_result = publisher_ops.upload_file(&viewer, "fail.txt".to_string(), data);
    assert!(matches!(upload_result, Err(DomainError::Unauthorized(_))));
}
