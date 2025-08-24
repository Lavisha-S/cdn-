// src/cdn_app_backend/src/file_store/storage.rs

use crate::types::{FileMetadata, User};
use crate::utils::{hashing, time, validation};
use crate::errors::BackendError;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    /// In-memory storage for demonstration (file_id -> content)
    static ref FILE_STORAGE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

/// Store a file with validations and generate metadata
///
/// # Arguments
/// * `filename` - Name of the file
/// * `content` - File content bytes
/// * `uploader` - User uploading the file
/// * `max_size` - Maximum allowed file size in bytes
///
/// # Returns
/// `FileMetadata` on success or `BackendError` on failure
pub fn store_file(
    filename: &str,
    content: &[u8],
    uploader: &User,
    max_size: usize,
) -> Result<FileMetadata, BackendError> {
    // Validate filename
    validation::validate_filename(filename)
        .map_err(|e| BackendError::ValidationError(e))?;

    // Validate file size
    validation::validate_file_size(content.len(), max_size)
        .map_err(|e| BackendError::ValidationError(e))?;

    if content.is_empty() {
        return Err(BackendError::ValidationError(
            "File content cannot be empty".into(),
        ));
    }

    // Generate file ID
    let timestamp = time::current_timestamp();
    let file_id = hashing::generate_file_id(content, timestamp);

    // Check for duplicate
    let mut storage = FILE_STORAGE.lock().unwrap();
    if storage.contains_key(&file_id) {
        return Err(BackendError::DuplicateFile);
    }

    // Compute file hash for integrity
    let file_hash = hashing::hash_bytes(content);

    // Store the file content
    storage.insert(file_id.clone(), content.to_vec());

    // Create metadata
    let metadata = FileMetadata {
        id: file_id,
        filename: filename.to_string(),
        uploader: uploader.username.clone(),
        uploaded_at: timestamp,
        file_hash,
        size: content.len() as u64,
    };

    Ok(metadata)
}

/// Retrieve file content by file ID
pub fn get_file(file_id: &str) -> Result<Vec<u8>, BackendError> {
    let storage = FILE_STORAGE.lock().unwrap();
    match storage.get(file_id) {
        Some(content) => Ok(content.clone()),
        None => Err(BackendError::FileNotFound),
    }
}

/// Delete a file by ID (Admin only)
pub fn delete_file(file_id: &str) -> Result<(), BackendError> {
    let mut storage = FILE_STORAGE.lock().unwrap();
    if storage.remove(file_id).is_some() {
        Ok(())
    } else {
        Err(BackendError::FileNotFound)
    }
}
