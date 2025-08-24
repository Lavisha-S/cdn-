// src/cdn_app_backend/src/utils/hashing.rs

use sha2::{Digest, Sha256};
use hex;

/// Hash a byte slice using SHA-256
///
/// # Arguments
/// * `data` - The data to hash
///
/// # Returns
/// A hexadecimal string representing the SHA-256 hash
pub fn hash_bytes(data: &[u8]) -> String {
    if data.is_empty() {
        panic!("hash_bytes: cannot hash empty data");
    }
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Hash a string using SHA-256
///
/// # Arguments
/// * `s` - The string to hash
///
/// # Returns
/// A hexadecimal string representing the SHA-256 hash
pub fn hash_string(s: &str) -> String {
    if s.is_empty() {
        panic!("hash_string: cannot hash empty string");
    }
    hash_bytes(s.as_bytes())
}

/// Generate a unique file ID using content + timestamp
///
/// # Arguments
/// * `content` - File content bytes
/// * `timestamp` - Current timestamp in seconds
///
/// # Returns
/// A unique SHA-256-based file ID
pub fn generate_file_id(content: &[u8], timestamp: u64) -> String {
    if content.is_empty() {
        panic!("generate_file_id: content cannot be empty");
    }
    let mut combined = content.to_vec();
    combined.extend_from_slice(timestamp.to_string().as_bytes());
    hash_bytes(&combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_bytes() {
        let data = b"hello world";
        let hash = hash_bytes(data);
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfade9e5e6c4529e6a1f6a3f7d9e2b0c1a3"
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_hash_bytes() {
        hash_bytes(b"");
    }

    #[test]
    fn test_hash_string() {
        let s = "hello world";
        let hash = hash_string(s);
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfade9e5e6c4529e6a1f6a3f7d9e2b0c1a3"
        );
    }

    #[test]
    fn test_generate_file_id_unique() {
        let content = b"file content";
        let id1 = generate_file_id(content, 1);
        let id2 = generate_file_id(content, 2);
        assert_ne!(id1, id2);
    }
}
