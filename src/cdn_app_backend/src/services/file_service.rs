// src/cdn_app_backend/src/services/file_service.rs

use crate::auth::checks;
use crate::types::user::Role;
use crate::file_store::{chunking, metadata, storage};
use crate::types::{User, FileMetadata};
use crate::errors::BackendError;

/// Maximum chunk size (1 MB)
const MAX_CHUNK_SIZE: usize = 1_048_576;

/// Upload a file (Publisher only)
pub fn upload_file(
    filename: &str,
    content: &[u8],
    user: &User,
    max_size: usize,
) -> Result<String, BackendError> {
    // Auth check
    checks::require_role(user, Role::Publisher)?;

    // Validate file size
    if content.is_empty() {
        return Err(BackendError::ValidationError("File cannot be empty".into()));
    }
    if content.len() > max_size {
        return Err(BackendError::ValidationError("File exceeds max size".into()));
    }

    // Chunk & hash
    let chunks = chunking::split_into_chunks(content, MAX_CHUNK_SIZE)?;
    let hashes = chunking::hash_chunks(&chunks);
    let reassembled = chunking::reassemble_chunks(&chunks)?;
    if reassembled != content {
        return Err(BackendError::ValidationError(
            "Chunk reassembly mismatch".into(),
        ));
    }

    // Store file & metadata
    let file_metadata = storage::store_file(filename, content, user, max_size)?;
    metadata::add_metadata(file_metadata.clone())?;

    Ok(file_metadata.id)
}

/// Download a file (Viewer or higher)
pub fn download_file(file_id: &str, user: &User) -> Result<Vec<u8>, BackendError> {
    checks::require_role(user, Role::Viewer)?;
    storage::get_file(file_id)
}

/// Get metadata (Viewer or higher)
pub fn get_file_metadata(file_id: &str, user: &User) -> Result<FileMetadata, BackendError> {
    checks::require_role(user, Role::Viewer)?;
    metadata::get_metadata(file_id)
}

/// Delete a file (Admin only)
pub fn delete_file(file_id: &str, user: &User) -> Result<(), BackendError> {
    checks::require_role(user, Role::Admin)?;

    storage::delete_file(file_id)?;
    metadata::delete_metadata(file_id)?;
    Ok(())
}

/// List files uploaded by a user (Viewer or higher)
pub fn list_user_files(username: &str, user: &User) -> Result<Vec<FileMetadata>, BackendError> {
    checks::require_role(user, Role::Viewer)?;
    Ok(metadata::list_user_files(username))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::User;
    use crate::types::user::Role;

    fn make_user(role: Role) -> User {
        User {
            username: "test_user".into(),
            role,
        }
    }

    #[test]
    fn test_upload_download() {
        let publisher = make_user(Role::Publisher);
        let viewer = make_user(Role::Viewer);

        let content = b"Hello CDN".to_vec();
        let file_id = upload_file("hello.txt", &content, &publisher, 1024).unwrap();
        let downloaded = download_file(&file_id, &viewer).unwrap();

        assert_eq!(content, downloaded);
    }

    #[test]
    fn test_unauthorized_upload() {
        let viewer = make_user(Role::Viewer);
        let result = upload_file("fail.txt", b"data", &viewer, 1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_file() {
        let admin = make_user(Role::Admin);
        let publisher = make_user(Role::Publisher);

        let content = b"Delete me".to_vec();
        let file_id = upload_file("delete.txt", &content, &publisher, 1024).unwrap();

        delete_file(&file_id, &admin).unwrap();
        assert!(download_file(&file_id, &publisher).is_err());
    }

    #[test]
    fn test_list_files() {
        let publisher = make_user(Role::Publisher);
        let viewer = make_user(Role::Viewer);

        upload_file("file1.txt", b"Data1", &publisher, 1024).unwrap();
        upload_file("file2.txt", b"Data2", &publisher, 1024).unwrap();

        let files = list_user_files("test_user", &viewer).unwrap();
        assert!(files.len() >= 2);
    }
}
