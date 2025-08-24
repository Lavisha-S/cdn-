// tests/edge_cases.rs

use ic_cdk::export::Principal;
use crate::types::{User, Role, FileMetadata, BackendError};
use crate::services::file_service::FileService;
use crate::file_store::storage::StorageManager;

#[test]
fn test_duplicate_file_upload() {
    let mut storage = StorageManager::new();
    let mut users = vec![User {
        id: Principal::anonymous(),
        role: Role::Publisher,
        name: "Publisher1".into(),
    }];
    let mut service = FileService::new(&mut storage, &mut users);

    let publisher_id = users[0].id;
    let publisher_role = Role::Publisher;

    let file_metadata = FileMetadata {
        id: "file1".into(),
        name: "Duplicate.txt".into(),
        size: 5,
        owner: publisher_id,
    };
    let file_data = b"12345".to_vec();

    assert!(service.upload_file(&publisher_role, &publisher_id, file_metadata.clone(), &file_data).is_ok());

    // Attempt duplicate upload → should fail
    let result = service.upload_file(&publisher_role, &publisher_id, file_metadata.clone(), &file_data);
    assert!(matches!(result, Err(BackendError::FileAlreadyExists(_))));
}

#[test]
fn test_invalid_file_metadata() {
    let mut storage = StorageManager::new();
    let mut users = vec![User {
        id: Principal::anonymous(),
        role: Role::Publisher,
        name: "Publisher1".into(),
    }];
    let mut service = FileService::new(&mut storage, &mut users);

    let publisher_id = users[0].id;
    let publisher_role = Role::Publisher;

    // File with empty name
    let invalid_metadata = FileMetadata {
        id: "file2".into(),
        name: "".into(),
        size: 10,
        owner: publisher_id,
    };
    let file_data = b"abcdefghij".to_vec();

    let result = service.upload_file(&publisher_role, &publisher_id, invalid_metadata, &file_data);
    assert!(matches!(result, Err(BackendError::InvalidMetadata(_))));
}

#[test]
fn test_non_existent_file_download() {
    let mut storage = StorageManager::new();
    let mut users = vec![User {
        id: Principal::anonymous(),
        role: Role::Viewer,
        name: "Viewer1".into(),
    }];
    let mut service = FileService::new(&mut storage, &mut users);

    let viewer_id = users[0].id;
    let viewer_role = Role::Viewer;

    let result = service.download_file(&viewer_role, &viewer_id, "nonexistent_file");
    assert!(matches!(result, Err(BackendError::FileNotFound(_))));
}

#[test]
fn test_non_existent_user_operation() {
    let mut storage = StorageManager::new();
    let mut users = vec![];
    let mut service = FileService::new(&mut storage, &mut users);

    let admin_role = Role::Admin;
    let fake_user_id = Principal::anonymous();

    // Admin trying to remove a user that does not exist
    let result = service.remove_user(&admin_role, &fake_user_id);
    assert!(matches!(result, Err(BackendError::UserNotFound(_))));
}

#[test]
fn test_file_chunking_edge_case() {
    let mut storage = StorageManager::new();
    let mut users = vec![User {
        id: Principal::anonymous(),
        role: Role::Publisher,
        name: "Publisher1".into(),
    }];
    let mut service = FileService::new(&mut storage, &mut users);

    let publisher_id = users[0].id;
    let publisher_role = Role::Publisher;

    // File exactly at the chunk boundary
    let file_size = storage.chunk_size() * 3; // 3 full chunks
    let file_metadata = FileMetadata {
        id: "chunk_boundary".into(),
        name: "ChunkBoundary.txt".into(),
        size: file_size as u64,
        owner: publisher_id,
    };
    let file_data = vec![0u8; file_size];

    assert!(service.upload_file(&publisher_role, &publisher_id, file_metadata.clone(), &file_data).is_ok());

    // Verify chunks stored correctly
    let chunks = storage.get_file_chunks(&file_metadata.id).unwrap();
    assert_eq!(chunks.len(), 3);
}
