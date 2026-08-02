//! Origin / Referer checks for cookie-authenticated state changes.

use axum::http::{HeaderMap, HeaderValue};

use crate::error::AppError;

pub fn assert_trusted_origin(headers: &HeaderMap, frontend_origin: &str) -> Result<(), AppError> {
    if let Some(origin) = headers.get(axum::http::header::ORIGIN) {
        return origin_matches(origin, frontend_origin);
    }

    if let Some(referer) = headers.get(axum::http::header::REFERER) {
        let referer_str = referer.to_str().map_err(|_| AppError::Forbidden)?;
        if referer_str.starts_with(frontend_origin) {
            return Ok(());
        }
        return Err(AppError::Forbidden);
    }

    // Non-browser clients (curl/tests) without Origin/Referer are rejected for state changes.
    Err(AppError::Forbidden)
}

fn origin_matches(value: &HeaderValue, frontend_origin: &str) -> Result<(), AppError> {
    let origin = value.to_str().map_err(|_| AppError::Forbidden)?;
    if origin == frontend_origin {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::assert_trusted_origin;
    use axum::http::{HeaderMap, HeaderValue};

    #[test]
    fn accepts_matching_origin() {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::ORIGIN,
            HeaderValue::from_static("http://127.0.0.1:5173"),
        );
        assert!(assert_trusted_origin(&headers, "http://127.0.0.1:5173").is_ok());
    }

    #[test]
    fn rejects_missing_origin_and_referer() {
        let headers = HeaderMap::new();
        assert!(assert_trusted_origin(&headers, "http://127.0.0.1:5173").is_err());
    }
}
