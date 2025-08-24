use ic_cdk::api::time;
use ic_cdk::print;
use candid::Principal;

/// Custom error type for uniform error handling.
#[derive(Debug, Clone)]
pub enum AppError {
    Unauthorized(String),
    InvalidInput(String),
    NotFound(String),
    StorageError(String),
    ConfigError(String),
    InternalError(String),
}

impl ToString for AppError {
    fn to_string(&self) -> String {
        match self {
            AppError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
            AppError::InvalidInput(msg) => format!("Invalid input: {}", msg),
            AppError::NotFound(msg) => format!("Not found: {}", msg),
            AppError::StorageError(msg) => format!("Storage error: {}", msg),
            AppError::ConfigError(msg) => format!("Config error: {}", msg),
            AppError::InternalError(msg) => format!("Internal error: {}", msg),
        }
    }
}

/// Simple logger with timestamps (nanos -> seconds).
pub fn log_info(msg: &str) {
    let ts = time() / 1_000_000_000;
    print(format!("[INFO  {}s] {}", ts, msg));
}

pub fn log_warn(msg: &str) {
    let ts = time() / 1_000_000_000;
    print(format!("[WARN  {}s] {}", ts, msg));
}

pub fn log_error(msg: &str) {
    let ts = time() / 1_000_000_000;
    print(format!("[ERROR {}s] {}", ts, msg));
}

/// Validate a filename (basic check, extendable).
/// - Must not be empty
/// - Max length = 255
/// - Allowed: letters, digits, `_-.`
pub fn validate_filename(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::InvalidInput("filename cannot be empty".into()));
    }
    if name.len() > 255 {
        return Err(AppError::InvalidInput("filename too long".into()));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || ['_', '-', '.'].contains(&c))
    {
        return Err(AppError::InvalidInput("filename contains invalid characters".into()));
    }
    Ok(())
}

/// Validate file size against config.
pub fn validate_file_size(size: u64, max_allowed: u64) -> Result<(), AppError> {
    if size == 0 {
        return Err(AppError::InvalidInput("file size cannot be zero".into()));
    }
    if size > max_allowed {
        return Err(AppError::InvalidInput(format!(
            "file size {} exceeds max allowed {}",
            size, max_allowed
        )));
    }
    Ok(())
}

/// Generate unique file IDs.
/// Combines caller principal + timestamp for uniqueness.
pub fn generate_file_id(caller: &Principal) -> String {
    format!("{}_{}", caller.to_text(), time())
}

/// Safe conversion from `Option<T>` to `Result<T, AppError>`.
pub fn option_to_result<T>(val: Option<T>, msg: &str) -> Result<T, AppError> {
    val.ok_or_else(|| AppError::NotFound(msg.into()))
}

/// Mask a principal for logs (security: avoid leaking full IDs).
pub fn mask_principal(principal: &Principal) -> String {
    let s = principal.to_text();
    if s.len() <= 10 {
        s
    } else {
        format!("{}...{}", &s[..5], &s[s.len() - 5..])
    }
}
