// src/cdn_app_backend/src/file_store/chunking.rs

use crate::utils::{hashing, validation};
use crate::errors::BackendError;

/// Split a file into chunks of `chunk_size` bytes
///
/// # Arguments
/// * `content` - Full file bytes
/// * `chunk_size` - Maximum size of each chunk
///
/// # Returns
/// Vector of byte vectors (chunks) or `BackendError`
pub fn split_into_chunks(content: &[u8], chunk_size: usize) -> Result<Vec<Vec<u8>>, BackendError> {
    if content.is_empty() {
        return Err(BackendError::ValidationError("File content is empty".into()));
    }
    if chunk_size == 0 {
        return Err(BackendError::ValidationError("Chunk size must be > 0".into()));
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    while start < content.len() {
        let end = std::cmp::min(start + chunk_size, content.len());
        chunks.push(content[start..end].to_vec());
        start = end;
    }
    Ok(chunks)
}

/// Reassemble chunks into a single file
///
/// # Arguments
/// * `chunks` - Vector of byte vectors
///
/// # Returns
/// Single byte vector or `BackendError`
pub fn reassemble_chunks(chunks: &[Vec<u8>]) -> Result<Vec<u8>, BackendError> {
    if chunks.is_empty() {
        return Err(BackendError::ValidationError("No chunks provided".into()));
    }

    let total_size: usize = chunks.iter().map(|c| c.len()).sum();
    if total_size == 0 {
        return Err(BackendError::ValidationError("All chunks are empty".into()));
    }

    let mut file = Vec::with_capacity(total_size);
    for chunk in chunks {
        file.extend_from_slice(chunk);
    }

    Ok(file)
}

/// Generate hashes for each chunk
///
/// # Arguments
/// * `chunks` - Vector of byte vectors
///
/// # Returns
/// Vector of hash strings corresponding to each chunk
pub fn hash_chunks(chunks: &[Vec<u8>]) -> Vec<String> {
    chunks.iter().map(|chunk| hashing::hash_bytes(chunk)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_reassemble() {
        let data = b"The quick brown fox jumps over the lazy dog".to_vec();
        let chunks = split_into_chunks(&data, 10).unwrap();
        assert!(chunks.len() > 0);

        let reassembled = reassemble_chunks(&chunks).unwrap();
        assert_eq!(data, reassembled);
    }

    #[test]
    fn test_hash_chunks() {
        let data = b"Hello World".to_vec();
        let chunks = split_into_chunks(&data, 5).unwrap();
        let hashes = hash_chunks(&chunks);
        assert_eq!(hashes.len(), chunks.len());
    }

    #[test]
    fn test_split_empty() {
        let result = split_into_chunks(&[], 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_reassemble_empty() {
        let result = reassemble_chunks(&[]);
        assert!(result.is_err());
    }
}
