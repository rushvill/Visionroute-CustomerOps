//! Outbound URL / SSRF policy.
//!
//! Phase 4 domain code must not fetch attacker-controlled URLs.
//! When integrations (Tracksolid, webhooks) are added, route them through
//! `validate_outbound_url` and an allowlisted host set.

use crate::error::AppError;

/// Hosts permitted for server-side HTTP fetches (empty until integrations ship).
const ALLOWED_HOST_SUFFIXES: &[&str] = &[];

/// Reject any outbound URL that is not on the allowlist.
pub fn validate_outbound_url(raw: &str) -> Result<(), AppError> {
    let Ok(url) = url::Url::parse(raw) else {
        return Err(AppError::Validation("Invalid URL.".to_owned()));
    };

    if url.scheme() != "https" {
        return Err(AppError::Validation(
            "Only https outbound URLs are allowed.".to_owned(),
        ));
    }

    let Some(host) = url.host_str() else {
        return Err(AppError::Validation("URL host is required.".to_owned()));
    };

    let host = host.to_ascii_lowercase();
    if is_blocked_literal(&host) {
        return Err(AppError::Validation(
            "Outbound URL host is not allowed.".to_owned(),
        ));
    }

    if ALLOWED_HOST_SUFFIXES.is_empty() {
        return Err(AppError::Validation(
            "Outbound HTTP fetches are disabled until an allowlist is configured.".to_owned(),
        ));
    }

    let allowed = ALLOWED_HOST_SUFFIXES
        .iter()
        .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")));

    if allowed {
        Ok(())
    } else {
        Err(AppError::Validation(
            "Outbound URL host is not allowed.".to_owned(),
        ))
    }
}

fn is_blocked_literal(host: &str) -> bool {
    host == "localhost"
        || host.ends_with(".localhost")
        || host.ends_with(".local")
        || host.starts_with("127.")
        || host.starts_with("10.")
        || host.starts_with("192.168.")
        || host.starts_with("169.254.")
        || host == "::1"
        || host == "0.0.0.0"
        || host == "[::1]"
}

#[cfg(test)]
mod tests {
    use super::validate_outbound_url;

    #[test]
    fn rejects_http_and_localhost() {
        assert!(validate_outbound_url("http://example.com").is_err());
        assert!(validate_outbound_url("https://localhost/x").is_err());
        assert!(validate_outbound_url("https://127.0.0.1/x").is_err());
    }

    #[test]
    fn rejects_when_allowlist_empty() {
        assert!(validate_outbound_url("https://api.example.com/v1").is_err());
    }
}
