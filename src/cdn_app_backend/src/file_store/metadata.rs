// src/cdn_app_backend/src/file_store/metadata.rs

use crate::types::FileMetadata;
use crate::errors::BackendError;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    /// In-memory metadata storage (file_id -> FileMetadata)
    static ref METADATA_STORAGE: Mutex<HashMap<String, FileMetadata>> = Mutex::new(HashMap::new());
}

/// Add metadata to storage
///
/// # Arguments
/// * `metadata` - FileMetadata object
///
/// # Returns
/// Ok(()) if successful or BackendError
pub fn add_metadata(metadata: FileMetadata) -> Result<(), BackendError> {
    let mut storage = METADATA_STORAGE.lock().unwrap();
    if storage.contains_key(&metadata.id) {
        return Err(BackendError::DuplicateFile);
    }
    storage.insert(metadata.id.clone(), metadata);
    Ok(())
}

/// Retrieve metadata by file ID
pub fn get_metadata(file_id: &str) -> Result<FileMetadata, BackendError> {
    let storage = METADATA_STORAGE.lock().unwrap();
    match storage.get(file_id) {
        Some(metadata) => Ok(metadata.clone()),
        None => Err(BackendError::FileNotFound),
    }
}

/// Delete metadata by file ID
pub fn delete_metadata(file_id: &str) -> Result<(), BackendError> {
    let mut storage = METADATA_STORAGE.lock().unwrap();
    if storage.remove(file_id).is_some() {
        Ok(())
    } else {
        Err(BackendError::FileNotFound)
    }
}

/// List all files uploaded by a specific user
pub fn list_user_files(username: &str) -> Vec<FileMetadata> {
    let storage = METADATA_STORAGE.lock().unwrap();
    storage
        .values()
        .filter(|meta| meta.uploader == username)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FileMetadata;

    fn sample_metadata(id: &str, uploader: &str) -> FileMetadata {
        FileMetadata {
            id: id.to_string(),
            filename: "test.txt".to_string(),
            uploader: uploader.to_string(),
            uploaded_at: 1234567890,
            file_hash: "abc123".to_string(),
            size: 1024,
        }
    }

    #[test]
    fn test_add_and_get_metadata() {
        let meta = sample_metadata("file1", "user1");
        add_metadata(meta.clone()).unwrap();
        let retrieved = get_metadata("file1").unwrap();
        assert_eq!(retrieved.id, "file1");
    }

    #[test]
    fn test_delete_metadata() {
        let meta = sample_metadata("file2", "user2");
        add_metadata(meta.clone()).unwrap();
        delete_metadata("file2").unwrap();
        assert!(get_metadata("file2").is_err());
    }

    #[test]
    fn test_list_user_files() {
        let meta1 = sample_metadata("file3", "user3");
        let meta2 = sample_metadata("file4", "user3");
        add_metadata(meta1.clone()).unwrap();
        add_metadata(meta2.clone()).unwrap();
        let list = list_user_files("user3");
        assert_eq!(list.len(), 2);
    }
}
