use candid::{CandidType, Deserialize};
use ic_cdk::api::caller::msg_caller;
use ic_cdk::storage::{stable_restore, stable_save};
use std::cell::RefCell;

use crate::auth::{require_role, Role};

/// CDN Configuration parameters (extendable).
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Max file size allowed (bytes).
    pub max_file_size_bytes: u64,
    /// Whether new file uploads are allowed.
    pub uploads_enabled: bool,
    /// Optional CDN domain name (for gateway redirects).
    pub cdn_domain: Option<String>,
    /// Timestamp (nanos) of last update.
    pub last_updated_nanos: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 50 * 1024 * 1024, // 50 MB default
            uploads_enabled: true,
            cdn_domain: None,
            last_updated_nanos: ic_cdk::api::time(),
        }
    }
}

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

/// Get current configuration (always allowed).
pub fn get_config() -> Config {
    CONFIG.with(|c| c.borrow().clone())
}

/// Update configuration. Only Admins can call.
/// Partial update: callers may provide `Some(new_value)` or `None` to keep old.
pub fn update_config(
    max_file_size_bytes: Option<u64>,
    uploads_enabled: Option<bool>,
    cdn_domain: Option<Option<String>>,
) -> Result<Config, String> {
    require_role(&msg_caller(), &[Role::Admin])?;

    CONFIG.with(|c| {
        let mut conf = c.borrow_mut();

        if let Some(size) = max_file_size_bytes {
            if size == 0 {
                return Err("max_file_size_bytes must be > 0".into());
            }
            // Hard cap to prevent malicious configs (e.g. 1 TB).
            if size > 1_000_000_000 {
                return Err("max_file_size_bytes exceeds 1 GB hard cap".into());
            }
            conf.max_file_size_bytes = size;
        }

        if let Some(enabled) = uploads_enabled {
            conf.uploads_enabled = enabled;
        }

        if let Some(domain_opt) = cdn_domain {
            if let Some(domain) = &domain_opt {
                if !is_valid_domain(domain) {
                    return Err("invalid cdn_domain format".into());
                }
            }
            conf.cdn_domain = domain_opt;
        }

        conf.last_updated_nanos = ic_cdk::api::time();

        Ok(conf.clone())
    })
}

/// Reset config to defaults. Only Admins can call.
pub fn reset_config() -> Result<Config, String> {
    require_role(&msg_caller(), &[Role::Admin])?;

    CONFIG.with(|c| {
        *c.borrow_mut() = Config::default();
        Ok(c.borrow().clone())
    })
}

/// Validate domain name (basic check).
fn is_valid_domain(domain: &str) -> bool {
    // Example: very simple check — letters, digits, dashes, dots
    let valid_chars = domain
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '.');
    valid_chars && domain.contains('.') && !domain.starts_with('-') && !domain.ends_with('-')
}

/// Persist config state before upgrade.
pub fn pre_upgrade() -> Result<(), String> {
    CONFIG.with(|c| stable_save((c.borrow().clone(),)).map_err(|e| format!("stable_save failed: {e}")))
}

/// Restore config state after upgrade.
pub fn post_upgrade() -> Result<(), String> {
    match stable_restore::<(Config,)>() {
        Ok((conf,)) => {
            CONFIG.with(|c| *c.borrow_mut() = conf);
            Ok(())
        }
        Err(_) => {
            // fallback to default config
            CONFIG.with(|c| *c.borrow_mut() = Config::default());
            Ok(())
        }
    }
}
