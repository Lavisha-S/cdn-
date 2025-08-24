// src/cdn_app_backend/src/utils/time.rs

use chrono::{DateTime, Utc};

/// Get current UTC timestamp in seconds since epoch
pub fn current_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

/// Get current UTC datetime
pub fn current_utc_datetime() -> DateTime<Utc> {
    Utc::now()
}

/// Format a DateTime<Utc> to RFC3339 string
pub fn datetime_to_rfc3339(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Parse RFC3339 string into DateTime<Utc>
/// Returns Result<DateTime<Utc>, chrono::ParseError>
pub fn parse_rfc3339(datetime_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    datetime_str.parse::<DateTime<Utc>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let ts = current_timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_datetime_to_rfc3339() {
        let dt = current_utc_datetime();
        let formatted = datetime_to_rfc3339(&dt);
        assert!(formatted.contains("T") && formatted.contains("Z"));
    }

    #[test]
    fn test_parse_rfc3339() {
        let dt_str = "2025-08-24T12:00:00Z";
        let dt = parse_rfc3339(dt_str).unwrap();
        assert_eq!(dt.to_rfc3339(), dt_str);
    }

    #[test]
    fn test_parse_invalid_rfc3339() {
        let dt_str = "invalid-date";
        assert!(parse_rfc3339(dt_str).is_err());
    }
}
