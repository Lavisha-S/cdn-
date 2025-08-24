// src/cdn_app_backend/src/utils/validation.rs

/// Validate username: non-empty, 3-32 chars
pub fn validate_username(username: &str) -> Result<(), String> {
    let len = username.chars().count();
    if len < 3 {
        return Err("Username too short; must be at least 3 characters".into());
    }
    if len > 32 {
        return Err("Username too long; max 32 characters allowed".into());
    }
    Ok(())
}

/// Validate filename: non-empty, only allowed characters
pub fn validate_filename(filename: &str) -> Result<(), String> {
    if filename.is_empty() {
        return Err("Filename cannot be empty".into());
    }
    if !filename
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-')
    {
        return Err("Filename contains invalid characters".into());
    }
    Ok(())
}

/// Validate file size (max_size in bytes)
pub fn validate_file_size(size: usize, max_size: usize) -> Result<(), String> {
    if size > max_size {
        return Err(format!("File size exceeds maximum of {} bytes", max_size));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(validate_username("abc").is_ok());
        assert!(validate_username("a").is_err());
        assert!(validate_username(&"a".repeat(33)).is_err());
    }

    #[test]
    fn test_validate_filename() {
        assert!(validate_filename("file.txt").is_ok());
        assert!(validate_filename("file@.txt").is_err());
        assert!(validate_filename("").is_err());
    }

    #[test]
    fn test_validate_file_size() {
        let max = 1024;
        assert!(validate_file_size(512, max).is_ok());
        assert!(validate_file_size(2048, max).is_err());
    }
}
